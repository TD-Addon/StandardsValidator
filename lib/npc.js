'use strict';

const { isAutoCalc, isFemale } = require('./util');

const BOUNTY_ALARM = 100;
const HOSTILE = 70;
const UNIQUE_NPCS = {};
const KHAJIIT_ANIMATIONS = ['T_Els_Ohmes-raht', 'T_Els_Suthay'].map(s => s.toLowerCase());
const KHAJIIT_F = 'epos_kha_upr_anim_f.nif';
const KHAJIIT_M = 'epos_kha_upr_anim_m.nif';

function buildRule(rule) {
	if(Array.isArray(rule)) {
		const predicates = rule.map(buildRule);
		return v => predicates.every(p => p(v));
	} else if(typeof rule === 'object') {
		if(rule.not) {
			const predicate = buildRule(rule.not);
			return v => !predicate(v);
		}
	} else {
		const lower = rule.toLowerCase();
		return v => v === lower;
	}
}

function buildRules(rules, part, model) {
	const tests = rules.map(rule => {
		const equality = {};
		for(const key in rule) {
			if(model && part && key === 'id' && typeof rule[key] === 'string') {
				const npc = rule[key].toLowerCase();
				if(!UNIQUE_NPCS[npc]) {
					UNIQUE_NPCS[npc] = {};
				}
				UNIQUE_NPCS[npc][part] = model;
			}
			const predicate = buildRule(rule[key]);
			if(predicate) {
				equality[key] = predicate;
			}
		}
		return record => {
			for(const key in equality) {
				if(!equality[key](record[key]?.toLowerCase())) {
					return false;
				}
			}
			return true;
		};
	});
	return record => tests.some(t => t(record));
}

const bodyParts = require('../data/bodyparts');
const RULE_SETS = Object.entries(bodyParts.ruleSets).reduce((all, [name, rules]) => {
	all[name] = buildRules(rules);
	return all;
}, {});
const PARTS = Object.entries(bodyParts).reduce((all, [part, definitions]) => {
	if(part === 'ruleSets') {
		return all;
	}
	if(!all[part]) {
		all[part] = {};
	}
	definitions.forEach(definition => {
		const model = definition.model.toLowerCase();
		let predicate;
		if(definition.rules) {
			predicate = buildRules(definition.rules, part, model);
		}
		if(definition.ruleSet) {
			const ruleSet = RULE_SETS[definition.ruleSet];
			if(predicate) {
				predicate = v => predicate(v) && ruleSet(v);
			} else {
				predicate = ruleSet;
			}
		}
		all[part][model] = predicate;
	});
	return all;
}, {});

function checkBodyParts(record) {
	for(const part in PARTS) {
		const bodypart = record[part]?.toLowerCase();
		const rules = PARTS[part][bodypart];
		if(rules && !rules(record)) {
			console.log(record.type, record.id, 'is using', part, record[part]);
		}
	}
	const npcParts = UNIQUE_NPCS[record.id.toLowerCase()];
	if(npcParts) {
		for(const part in npcParts) {
			if(record[part]?.toLowerCase() !== npcParts[part]) {
				console.log(record.type, record.id, 'is not using unique', part, npcParts[part]);
			}
		}
	}
}

function checkKhajiitAnimations(record) {
	const requiresAnimations = KHAJIIT_ANIMATIONS.includes(record.race?.toLowerCase());
	const mesh = record.mesh?.toLowerCase();
	if(requiresAnimations) {
		const target = isFemale(record) ? KHAJIIT_F : KHAJIIT_M;
		if(mesh !== target) {
			console.log(record.type, record.id, 'is not using animation', target);
		}
	} else if(mesh === KHAJIIT_F || mesh === KHAJIIT_M) {
		console.log(record.type, record.id, 'has animation', record.mesh);
	}
}

let slaveBracerCount = 0;

module.exports = {
	onRecord(record, mode) {
		slaveBracerCount = 0;
		if(record.type === 'Npc') {
			checkBodyParts(record);
			if(mode === 'PT' && isAutoCalc(record)) {
				console.log(record.type, record.id, 'has auto calculated stats and spells');
			}
			if(record.ai_data?.fight >= HOSTILE && record.ai_data.alarm >= BOUNTY_ALARM) {
				console.log(record.type, record.id, 'reports crimes despite having', record.ai_data.fight, 'fight');
			}
			if(record.ai_data?.alarm < BOUNTY_ALARM && record.class?.toLowerCase() === 'guard') {
				console.log(record.type, record.id, 'does not report crimes despite being a guard');
			}
			checkKhajiitAnimations(record);
		}
	},
	onInventory(record, [count], id) {
		if(slaveBracerCount <= 1 && record.type === 'Npc' && (id === 'slave_bracer_left' || id === 'slave_bracer_right')) {
			slaveBracerCount += Math.abs(count);
			if(slaveBracerCount > 1) {
				console.log(record.type, record.id, 'has multiple slave bracers');
			}
		}
	}
};

'use strict';

const { isAutoCalc } = require('./util');

const TR_CLASSES = {};
const CLASSES = require('../data/classes').reduce((a, c) => {
	const vanilla = c.vanilla.toLowerCase();
	a[vanilla] = c.data;
	if(vanilla !== 'miner') {
		TR_CLASSES[c.data.toLowerCase()] = c.vanilla;
	}
	return a;
}, {});

const UNIQUE_NPCS = {};

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

module.exports = {
	onRecord(record, topic, mode) {
		if(record.type === 'Npc') {
			checkBodyParts(record);
			let replacementClass;
			if(mode === 'PT') {
				if(isAutoCalc(record)) {
					console.log(record.type, record.id, 'has auto calculated stats and spells');
				}
				replacementClass = CLASSES[record.class?.toLowerCase()];
			} else if(mode === 'TR') {
				replacementClass = TR_CLASSES[record.class?.toLowerCase()];
			}
			if(replacementClass) {
				console.log(record.type, record.id, 'has class', record.class, 'which should be', replacementClass);
			}
		}
	}
};

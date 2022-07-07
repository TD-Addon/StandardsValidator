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

function buildRules(rules) {
	const tests = rules.map(rule => {
		const equality = {};
		for(const key in rule) {
			const r = rule[key];
			if(typeof r === 'object') {
				if(r.not) {
					const lower = r.not.toLowerCase();
					equality[key] = v => v !== lower;
				}
			} else {
				const lower = r.toLowerCase();
				equality[key] = v => v === lower;
			}
		}
		return record => {
			for(const key in equality) {
				if(equality[key](record[key]?.toLowerCase())) {
					return false;
				}
			}
			return true;
		};
	});
	return record => tests.some(t => t(record));
}

const PARTS = Object.entries(require('../data/bodyparts')).reduce((all, [part, definitions]) => {
	if(!all[part]) {
		all[part] = {};
	}
	definitions.forEach(definition => {
		all[part][definition.model.toLowerCase()] = buildRules(definition.rules);
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

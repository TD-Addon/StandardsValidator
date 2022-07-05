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

module.exports = {
	onRecord(record, topic, mode) {
		if(record.type === 'Npc' && mode === 'PT') {
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

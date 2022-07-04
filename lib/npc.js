'use strict';

const { isAutoCalc } = require('./util');
const CLASSES = require('../data/classes').reduce((a, c) => {
	a[c.vanilla.toLowerCase()] = c.data;
	return a;
}, {});

module.exports = {
	onRecord(record, topic, mode) {
		if(record.type === 'Npc' && mode === 'PT') {
			if(isAutoCalc(record)) {
				console.log(record.type, record.id, 'has auto calculated stats and spells');
			}
			const replacementClass = CLASSES[record.class?.toLowerCase()];
			if(replacementClass) {
				console.log(record.type, record.id, 'has class', record.class, 'which should be', replacementClass);
			}
		}
	}
};

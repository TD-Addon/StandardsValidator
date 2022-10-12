'use strict';

const TR_CLASSES = {};
const CLASSES = require('../data/classes').reduce((a, c) => {
	const vanilla = c.vanilla.toLowerCase();
	a[vanilla] = c.data;
	if(vanilla !== 'miner') {
		TR_CLASSES[c.data.toLowerCase()] = c.vanilla;
	}
	return a;
}, {});

function getReplacementClass(c, mode) {
	if(mode === 'PT') {
		return CLASSES[c?.toLowerCase()];
	} else if(mode === 'TR') {
		return TR_CLASSES[c?.toLowerCase()];
	}
}

module.exports = {
	onRecord(record, mode) {
		if(record.type === 'Npc') {
			const replacementClass = getReplacementClass(record.class, mode);
			if(replacementClass) {
				console.log(record.type, record.id, 'has class', record.class, 'which should be', replacementClass);
			}
		}
	},
	onInfo(record, topic, mode) {
		if(record.speaker_class) {
			const replacementClass = getReplacementClass(record.speaker_class, mode);
			if(replacementClass) {
				console.log(record.type, record.info_id, 'in topic', topic.id, 'has a', record.speaker_class, 'filter');
			}
		}
		record.filters?.forEach(filter => {
			if(filter.filter_type === 'NotClass') {
				const replacementClass = getReplacementClass(filter.id, mode);
				if(replacementClass) {
					console.log(record.type, record.info_id, 'in topic', topic.id, 'has a Not Class', filter.id, 'filter');
				}
			}
		});
	}
};

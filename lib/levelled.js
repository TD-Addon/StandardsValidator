'use strict';

const { calculatesForAllLevels } = require('./util');

const MINIMUM_LEVELS = {};
const TO_CHECK = [];

function checkAllLevels(record, key) {
	const length = record[key]?.length;
	if(length > 1 && !calculatesForAllLevels(record)) {
		const [, first] = record[key][0];
		for(let i = 1; i < length; i++) {
			const [, level] = record[key][i];
			if(level !== first) {
				console.log(record.type, record.id, 'is not calculated for all levels');
				break;
			}
		}
	}
}

module.exports = {
	onRecord(record, mode, recordId) {
		if(record.type === 'LevelledCreature') {
			checkAllLevels(record, 'creatures');
			MINIMUM_LEVELS[recordId] = record.creatures?.[0]?.[1];
			TO_CHECK.push(record);
		} else if(record.type === 'LevelledItem') {
			checkAllLevels(record, 'items');
			MINIMUM_LEVELS[recordId] = record.items?.[0]?.[1];
			TO_CHECK.push(record);
		}
	},
	onEnd() {
		TO_CHECK.forEach(record => {
			const key = record.type === 'LevelledCreature' ? 'creatures' : 'items';
			record[key]?.forEach(([item, level]) => {
				const minimum = MINIMUM_LEVELS[item];
				if(minimum > level) {
					console.log(record.type, record.id, 'contains', item, 'at level', level, 'which will not resolve to anything at that level');
				}
			});
		});
	}
};

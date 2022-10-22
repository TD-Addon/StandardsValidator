'use strict';

const { calculatesForAllLevels } = require('./util');

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
	onRecord(record) {
		if(record.type === 'LevelledCreature') {
			checkAllLevels(record, 'creatures');
		} else if(record.type === 'LevelledItem') {
			checkAllLevels(record, 'items');
		}
	}
};

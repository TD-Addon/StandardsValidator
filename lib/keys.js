'use strict';

const { isKey, getCellName } = require('./util');
const MISCS = new Set();

module.exports = {
	onRecord(record, mode) {
		if(record.type === 'MiscItem') {
			if(mode !== 'TD' && !isKey(record) && /key/i.test(record.id)) {
				console.log(record.type, record.id, 'is not a key');
			}
			MISCS.add(record.id.toLowerCase());
		}
	},
	onCellRef(record, ref) {
		if(ref.key && !MISCS.has(ref.key.toLowerCase())) {
			console.log(record.type, getCellName(record), 'uses key', ref.key, 'to open', ref.id, 'which is not defined in this file');
		}
	}
};

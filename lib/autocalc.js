'use strict';

const { isAutoCalc } = require('./util');

module.exports = {
	onRecord(record) {
		if(record.type === 'Spell' && isAutoCalc(record)) {
			console.log(record.type, record.id, 'is auto calculated');
		}
	}
};

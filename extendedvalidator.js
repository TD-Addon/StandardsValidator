'use strict';

const { runValidator } = require('./lib/extended/program');
const validators = [
	require('./lib/extended/cells'),
	require('./lib/extended/ownership'),
	require('./lib/extended/weapons')
];

runValidator('extended.js', {
	onRecord(record, last) {
		const recordId = record.id?.toLowerCase();
		validators.forEach(validator => validator.onRecord(record, last, recordId));
	},
	onEnd() {
		validators.forEach(validator => validator.onEnd?.());
	}
});

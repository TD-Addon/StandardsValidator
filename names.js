'use strict';

const validators = [
	require('./lib/extended/names'),
	require('./lib/extended/quests')
];

const { runValidator } = require('./lib/extended/program');

runValidator('names.js', {
	onRecord(...args) {
		validators.forEach(validator => validator.onRecord(...args));
	}
});

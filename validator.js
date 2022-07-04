'use strict';

const process = require('process');
const fs = require('fs');
const validators = [
	require('./lib/corpse'),
	require('./lib/ids'),
	require('./lib/npcscripts'),
	require('./lib/persistent'),
	require('./lib/travel'),
	require('./lib/uniques')
];

function getJson() {
	const [,, inputFile] = process.argv;
	if(inputFile) {
		return JSON.parse(fs.readFileSync(inputFile, 'utf-8'));
	}
	throw new Error('Usage: node validator.js [input.json]');
}

const records = getJson();
let currentTopic = null;
records.forEach(record => {
	if(record.type === 'Dialogue') {
		currentTopic = record;
	}
	validators.forEach(validator => validator.onRecord(record, currentTopic));
});

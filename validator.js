'use strict';

const process = require('process');
const fs = require('fs');
const validators = [
	require('./lib/autocalc'),
	require('./lib/corpse'),
	require('./lib/ids'),
	require('./lib/npc'),
	require('./lib/npcscripts'),
	require('./lib/persistent'),
	require('./lib/travel'),
	require('./lib/unicode'),
	require('./lib/uniques')
];

const MODES = ['PT', 'TR', 'TD'];

function getJson() {
	const [,, inputFile, mode] = process.argv;
	if(inputFile) {
		if(mode && !MODES.includes(mode)) {
			throw new Error(`Unknown mode ${mode} expected one of ${MODES.join(', ')}`);
		}
		return [JSON.parse(fs.readFileSync(inputFile, 'utf-8')), mode];
	}
	throw new Error('Usage: node validator.js [input.json] ([mode])');
}

const [records, mode] = getJson();
let currentTopic = null;
records.forEach(record => {
	if(record.type === 'Dialogue') {
		currentTopic = record;
	}
	validators.forEach(validator => validator.onRecord(record, currentTopic, mode));
});

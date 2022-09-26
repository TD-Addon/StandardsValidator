'use strict';

const process = require('process');
const fs = require('fs');
const validators = [
	require('./lib/autocalc'),
	require('./lib/classes'),
	require('./lib/corpse'),
	require('./lib/dialogue'),
	require('./lib/duplicates'),
	require('./lib/ids'),
	require('./lib/npc'),
	require('./lib/npcscripts'),
	require('./lib/orphans'),
	require('./lib/persistent'),
	require('./lib/soundgen'),
	require('./lib/travel'),
	require('./lib/unicode'),
	require('./lib/uniques')
].reduce((all, validator) => {
	for(const key in all) {
		if(key in validator) {
			all[key].push(validator);
		}
	}
	return all;
}, {
	onRecord: [],
	onCellRef: [],
	onEnd: []
});

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
	validators.onRecord.forEach(validator => validator.onRecord(record, currentTopic, mode));
	if(record.type === 'Cell') {
		record.references?.forEach((reference, i) => {
			const id = reference.id.toLowerCase();
			validators.onCellRef.forEach(validator => validator.onCellRef(record, reference, id, i, mode));
		});
	}
});
validators.onEnd.forEach(validator => validator.onEnd());

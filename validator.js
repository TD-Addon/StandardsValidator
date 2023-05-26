'use strict';

const process = require('process');
const fs = require('fs');
const validators = [
	require('./lib/autocalc'),
	require('./lib/books'),
	require('./lib/cells'),
	require('./lib/classes'),
	require('./lib/corpse'),
	require('./lib/dialogue'),
	require('./lib/duplicates'),
	require('./lib/ids'),
	require('./lib/keys'),
	require('./lib/levelled'),
	require('./lib/magic'),
	require('./lib/missing'),
	require('./lib/npc'),
	require('./lib/orphans'),
	require('./lib/persistent'),
	require('./lib/scripts'),
	require('./lib/services'),
	require('./lib/soundgen'),
	require('./lib/supplies'),
	require('./lib/todo'),
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
	onLevelled: [],
	onInventory: [],
	onInfo: [],
	onScriptLine: [],
	onEnd: []
});

const MODES = ['PT', 'TR', 'TD'];

function getJson() {
	try {
		const [,, inputFile, mode] = process.argv;
		if(inputFile) {
			if(mode && !MODES.includes(mode)) {
				throw new Error(`Unknown mode ${mode} expected one of ${MODES.join(', ')}`);
			}
			return [JSON.parse(fs.readFileSync(inputFile, 'utf-8')), mode];
		}
		throw new Error('Usage: node validator.js [input.json] ([mode])');
	} catch(err) {
		console.error(err.message);
		process.exit(1);
	}
}

function handleLevelled(record, key, mode) {
	record[key]?.forEach((entry, i) => {
		const id = entry[0].toLowerCase();
		validators.onLevelled.forEach(validator => validator.onLevelled(record, entry, id, i, mode));
	});
}

function handleScript(record, key, mode, topic) {
	const script = record[key];
	if(script) {
		const lines = script.trim().split('\n');
		lines.forEach(line => {
			const commentStart = line.indexOf(';');
			let text, comment;
			if(commentStart >= 0) {
				text = line.slice(0, commentStart);
				comment = line.slice(commentStart + 1).trim();
			} else {
				text = line;
				comment = '';
			}
			text = text.trim().toLowerCase();
			if(text || comment) {
				validators.onScriptLine.forEach(validator => validator.onScriptLine(record, text, comment, topic, mode));
			}
		});
	}
}

const [records, mode] = getJson();
let currentTopic = null;
records.forEach(record => {
	const recordId = record.id?.toLowerCase();
	validators.onRecord.forEach(validator => validator.onRecord(record, mode, recordId));
	if(record.type === 'Cell') {
		record.references?.forEach((reference, i) => {
			const id = reference.id.toLowerCase();
			validators.onCellRef.forEach(validator => validator.onCellRef(record, reference, id, i, mode));
		});
	} else if(record.type === 'LevelledItem') {
		handleLevelled(record, 'items', mode);
	} else if(record.type === 'LevelledCreature') {
		handleLevelled(record, 'creatures', mode);
	} else if(['Container', 'Creature', 'Npc'].includes(record.type)) {
		record.inventory?.forEach((entry, i) => {
			const id = entry[1].toLowerCase();
			validators.onInventory.forEach(validator => validator.onInventory(record, entry, id, i, mode));
		});
	} else if(record.type === 'Dialogue') {
		currentTopic = record;
	} else if(record.type === 'Info') {
		validators.onInfo.forEach(validator => validator.onInfo(record, currentTopic, mode));
		handleScript(record, 'result', mode, currentTopic);
	} else if(record.type === 'Script') {
		handleScript(record, 'text', mode);
	}
});
validators.onEnd.forEach(validator => validator.onEnd(mode));

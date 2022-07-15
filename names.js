'use strict';

const process = require('process');
const fs = require('fs');
const validator = require('./lib/names');

function getJson() {
	const [,, ...files] = process.argv;
	if(files.length) {
		return files;
	}
	throw new Error('Usage: node names.js file1.json ... fileN.json');
}

const files = getJson();
for(const file of files) {
	const records = JSON.parse(fs.readFileSync(file, 'utf-8'));
	records.forEach(record => {
		validator.onRecord(record);
	});
}

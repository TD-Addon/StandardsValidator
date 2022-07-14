'use strict';

const process = require('process');
const fs = require('fs');
const validator = require('./lib/ownership');

function getJson() {
	const [,, ...files] = process.argv;
	if(files.length) {
		return files;
	}
	throw new Error('Usage: node ownership.js Morrowind.json Tribunal.json Bloodmoon.json MyFile1.json ... MyFileN.json');
}

const files = getJson();
const lastFile = files[files.length - 1];
for(const file of files) {
	const records = JSON.parse(fs.readFileSync(file, 'utf-8'));
	const last = file === lastFile;
	records.forEach(record => {
		validator.onRecord(record, last);
	});
}

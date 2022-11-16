'use strict';

const process = require('process');
const fs = require('fs');

function getJson(name) {
	const [,, ...files] = process.argv;
	if(files.length) {
		return files;
	}
	console.error(`Usage: node ${name} Morrowind.json Tribunal.json Bloodmoon.json MyFile1.json ... MyFileN.json`);
	process.exit(1);
}

function runValidator(name, validator) {
	const files = getJson(name);
	const lastFile = files[files.length - 1];
	for(const file of files) {
		const records = JSON.parse(fs.readFileSync(file, 'utf-8'));
		const last = file === lastFile;
		records.forEach(record => {
			validator.onRecord(record, last);
		});
	}
	validator.onEnd?.();
}

module.exports = { runValidator };

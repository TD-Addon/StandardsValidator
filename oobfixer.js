'use strict';

const fs = require('fs');
const { getCellGrid, getCellName, isInterior } = require('./lib/util');

function getJson() {
	const [,, inputFile, outputFile] = process.argv;
	if(inputFile && outputFile) {
		return [JSON.parse(fs.readFileSync(inputFile, 'utf-8')), outputFile];
	}
	throw new Error('Usage: node oobfixer.js [input.json] [output.json]');
}

const cells = {};
const [records, outputFile] = getJson();

records.forEach(record => {
	if(record.type === 'Cell' && !isInterior(record)) {
		cells[record.data.grid.join(',')] = record;
	}
});

Object.values(cells).forEach(cell => {
	const { references } = cell;
	const { grid } = cell.data;
	for(let i = 0; i < references.length; i++) {
		const ref = references[i];
		if('deleted' in ref) {
			continue;
		}
		const actualGrid = getCellGrid(...ref.translation);
		const dX = Math.abs(grid[0] - actualGrid[0]);
		const dY = Math.abs(grid[1] - actualGrid[1]);
		if(dX || dY) {
			if(dX > 1 || dY > 1) {
				console.log('Not moving', ref.id, 'from', getCellName(cell), 'as', actualGrid, 'is too far away');
				continue;
			}
			const targetCell = cells[actualGrid.join(',')];
			if(!targetCell) {
				console.log('Not moving', ref.id, 'from', getCellName(cell), 'as cell', actualGrid, 'is not in this file');
				continue;
			}
			references.splice(i--, 1);
			if(ref.temporary) {
				targetCell.references.unshift(ref);
			} else {
				targetCell.references.push(ref);
			}
		}
	}
});

fs.writeFileSync(outputFile, JSON.stringify(records));

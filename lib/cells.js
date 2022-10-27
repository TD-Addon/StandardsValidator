'use strict';

const CELL_SIZE = 8192;
const PREFIXES = require('../data/projects').map(p => p.prefix.toLowerCase());
const { isInterior, getCellName } = require('./util');

module.exports = {
	onRecord(record, mode, id) {
		if(isInterior(record) && !record.atmosphere_data?.fog_density && !PREFIXES.some(prefix => id.startsWith(prefix))) {
			console.log(record.type, record.id, 'has a fog density of 0');
		}
	},
	onCellRef(record, reference) {
		if(!('deleted' in reference) && !isInterior(record)) {
			const [x, y] = record.data.grid;
			const xBound = CELL_SIZE * x;
			const yBound = CELL_SIZE * y;
			const [xPos, yPos] = reference.translation;
			if(xPos < xBound || yPos < yBound || xPos >= xBound + CELL_SIZE || yPos >= yBound + CELL_SIZE) {
				console.log(record.type, getCellName(record), 'contains out of bounds reference', reference.id, 'at', reference.translation);
			}
		}
	}
};

'use strict';

const PREFIXES = require('../data/projects').map(p => p.prefix.toLowerCase());
const BROKEN = Object.entries(require('../data/broken')).reduce((o, [k, v]) => {
	o[k.toLowerCase()] = v;
	return o;
}, {});
const { isInterior, getCellName, CELL_SIZE, getCellGrid } = require('./util');

module.exports = {
	onRecord(record, mode, id) {
		if('deleted' in record) {
			return;
		}
		if(record.type === 'PathGrid') {
			const { points, connections } = record;
			if(!points?.length) {
				return;
			}
			const connected = new Set(connections);
			for(let i = 0; i < points.length; i++) {
				const point = points[i];
				if(point.connection_count) {
					connected.add(i);
				}
				for(let j = i + 1; j < points.length; j++) {
					const otherPoint = points[j];
					if(point.location.every((l, index) => l === otherPoint.location[index])) {
						console.log(record.type, getCellName(record), 'contains duplicate node at', point.location);
						break;
					}
				}
			}
			if(points.length !== connected.size) {
				for(let i = 0; i < points.length; i++) {
					if(!connected.has(i)) {
						console.log(record.type, getCellName(record), 'contains unconnected node at', points[i].location);
					}
				}
			}
		} else if(isInterior(record) && !record.atmosphere_data?.fog_density && !PREFIXES.some(prefix => id.startsWith(prefix))) {
			console.log(record.type, record.id, 'has a fog density of 0');
		}
	},
	onCellRef(record, reference, id) {
		if(!('deleted' in reference) && !isInterior(record)) {
			const [x, y] = record.data.grid;
			const xBound = CELL_SIZE * x;
			const yBound = CELL_SIZE * y;
			const [xPos, yPos] = reference.translation;
			if(xPos < xBound || yPos < yBound || xPos >= xBound + CELL_SIZE || yPos >= yBound + CELL_SIZE) {
				console.log(record.type, getCellName(record), 'contains out of bounds reference', reference.id, 'at', reference.translation, 'which should be in', getCellGrid(xPos, yPos));
			}
		}
		if(id in BROKEN) {
			const replacement = BROKEN[id];
			console.log(record.type, getCellName(record), 'contains broken reference', reference.id, replacement ? `which should be ${replacement}` : '');
		}
	}
};

'use strict';

const { isInterior, cannotSleep, getCellName, isDead } = require('../util');

const INHABITANTS = new Set();
const PATH_GRIDS = new Set();
const CELLS = [];
const MIN_INHABITANTS = 3;

module.exports = {
	onRecord(record, last, recordId) {
		if(record.type === 'PathGrid') {
			if(record.cell) {
				PATH_GRIDS.add(record.cell.toLowerCase());
			}
		} else if(record.type === 'Cell') {
			if(last && isInterior(record) && record.references.length > 1) {
				CELLS.push(record);
				if(!cannotSleep(record)) {
					const count = record.references.filter(ref => INHABITANTS.has(ref.id.toLowerCase())).length;
					if(count < MIN_INHABITANTS) {
						console.log(record.type, getCellName(record), 'contains', count, 'NPCs or creatures');
					}
				}
			}
		} else if(record.type === 'LevelledCreature') {
			INHABITANTS.add(recordId);
		} else if(['Creature', 'Npc'].includes(record.type) && !isDead(record)) {
			INHABITANTS.add(recordId);
		}
	},
	onEnd() {
		CELLS.forEach(cell => {
			if(!PATH_GRIDS.has(cell.id.toLowerCase())) {
				console.log(cell.type, getCellName(cell), 'is missing a path grid');
			}
		});
	}
};

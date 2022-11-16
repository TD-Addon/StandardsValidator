'use strict';

const { isInterior, cannotSleep, getCellName } = require('../util');

const INHABITANTS = new Set();
const PATH_GRIDS = new Set();
const CELLS = [];

module.exports = {
	onRecord(record, last, recordId) {
		if(record.type === 'PathGrid') {
			if(record.cell) {
				PATH_GRIDS.add(record.cell.toLowerCase());
			}
		} else if(record.type === 'Cell') {
			if(last && isInterior(record) && record.references.length > 1) {
				CELLS.push(record);
				if(!cannotSleep(record) && !record.references.some(ref => INHABITANTS.has(ref.id.toLowerCase()))) {
					console.log(record.type, getCellName(record), 'does not contain any NPCs or creatures');
				}
			}
		} else if(['Creature', 'Npc', 'LevelledCreature'].includes(record.type)) {
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

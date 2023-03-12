'use strict';

const { isInterior, cannotSleep, getCellName, isDead } = require('../util');
const ignore = require('../../data/ignore').extended;

const IGNORE_INHABITANTS = new Set(ignore.inhabitants.map(s => s.toLowerCase()));
const IGNORE_PATH_GRIDS = new Set(ignore.pathgrids.map(s => s.toLowerCase()));

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
				if(!cannotSleep(record) && !IGNORE_INHABITANTS.has(recordId)) {
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
			const id = cell.id.toLowerCase();
			if(!PATH_GRIDS.has(id) && !IGNORE_PATH_GRIDS.has(id)) {
				console.log(cell.type, getCellName(cell), 'is missing a path grid');
			}
		});
	}
};

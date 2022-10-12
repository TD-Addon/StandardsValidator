'use strict';

const { isPersistent } = require('./util');

const OBJECTS = ['Activator', 'Alchemy', 'Apparatus', 'Armor', 'Book', 'Clothing', 'Container', 'Door', 'Ingredient', 'Light', 'Lockpick', 'MiscItem', 'Probe', 'RepairTool', 'Static', 'Weapon'];

const counts = {};

module.exports = {
	onRecord(record, mode, recordId) {
		if(OBJECTS.includes(record.type) && isPersistent(record)) {
			counts[recordId] = 0;
		}
	},
	onCellRef(record, ref, id) {
		if(id in counts) {
			counts[id]++;
			if(counts[id] > 1) {
				console.log('Persistent object', ref.id, 'is used multiple times');
				delete counts[id];
			}
		}
	}
};

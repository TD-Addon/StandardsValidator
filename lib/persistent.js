'use strict';

const { isPersistent } = require('./util');

const OBJECTS = ['Activator', 'Alchemy', 'Apparatus', 'Armor', 'Book', 'Clothing', 'Container', 'Door', 'Ingredient', 'Light', 'Lockpick', 'MiscItem', 'Probe', 'Static', 'Weapon'];

const counts = {};

module.exports = {
	onRecord(record) {
		if(record.type === 'Cell') {
			record.references?.forEach(ref => {
				const id = ref.id.toLowerCase();
				if(id in counts) {
					counts[id]++;
					if(counts[id] > 1) {
						console.log('Persistent object', ref.id, 'is used multiple times');
						delete counts[id];
					}
				}
			});
		} else if(OBJECTS.includes(record.type) && isPersistent(record)) {
			counts[record.id.toLowerCase()] = 0;
		}
	}
};

'use strict';

const { getCellName, canCarry, cannotSleep } = require('../util');

const ITEM_TYPES = ['Alchemy', 'Apparatus', 'Armor', 'Book', 'Clothing', 'Container', 'Ingredient', 'Lockpick', 'MiscItem', 'Probe', 'RepairTool', 'Weapon'];
const ITEMS = new Set();

module.exports = {
	onRecord(record, check, recordId) {
		if(record.type === 'Activator') {
			if(record.script?.toLowerCase() === 'bed_standard') {
				ITEMS.add(recordId);
			}
		} else if(canCarry(record)) {
			ITEMS.add(recordId);
		} else if(record.type === 'Cell' && check) {
			const name = getCellName(record);
			let owned = 0;
			let unowned = 0;
			record.references.forEach(reference => {
				if(('lock_level' in reference || reference.trap) || ITEMS.has(reference.id.toLowerCase())) {
					if(reference.owner || reference.owner_faction) {
						owned++;
					} else {
						unowned++;
					}
				} else if(reference.owner || reference.owner_faction) {
					console.log(record.type, name, 'contains incorrectly owned object', reference.id);
				}
			});
			if(cannotSleep(record)) {
				if(unowned) {
					console.log(record.type, name, 'contains', unowned, 'unowned items');
				}
			} else if(owned) {
				console.log(record.type, name, 'contains', owned, 'owned items');
			}
		} else if(ITEM_TYPES.includes(record.type)) {
			ITEMS.add(recordId);
		}
	}
};

'use strict';

const FLAG_CARRY = 2;
const FLAG_NO_SLEEP = 4;
const ITEM_TYPES = ['Alchemy', 'Apparatus', 'Armor', 'Book', 'Clothing', 'Container', 'Ingredient', 'Light', 'Lockpick', 'MiscItem', 'Probe', 'RepairTool', 'Weapon'];
const ITEMS = new Set();

module.exports = {
	onRecord(record, check) {
		if(record.type === 'Activator') {
			if(record.script?.toLowerCase() === 'bed_standard') {
				ITEMS.add(record.id.toLowerCase());
			}
		} else if(record.type === 'Light') {
			if(record.data?.flags & FLAG_CARRY) {
				ITEMS.add(record.id.toLowerCase());
			}
		} else if(record.type === 'Cell' && check) {
			const name = record.id || record.region;
			let owned = 0;
			let unowned = 0;
			record.references.forEach(reference => {
				if('lock_level' in reference || ITEMS.has(reference.id.toLowerCase())) {
					if(reference.owner) {
						owned++;
					} else {
						unowned++;
					}
				} else if(reference.owner) {
					console.log(record.type, name, 'contains incorrectly owned object', reference.id);
				}
			});
			if(record.data?.flags & FLAG_NO_SLEEP) {
				if(unowned) {
					console.log(record.type, name, 'contains', unowned, 'unowned items');
				}
			} else if(owned) {
				console.log(record.type, name, 'contains', owned, 'owned items');
			}
		} else if(ITEM_TYPES.includes(record.type)) {
			ITEMS.add(record.id.toLowerCase());
		}
	}
};

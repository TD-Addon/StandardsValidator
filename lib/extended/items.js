'use strict';

const { getCellName, cannotSleep, isItem } = require('../util');

const ITEMS = new Set();
const OWNABLE = new Set();

module.exports = {
	onRecord(record, check, recordId) {
		if(record.type === 'Activator') {
			if(record.script?.toLowerCase() === 'bed_standard') {
				OWNABLE.add(recordId);
			}
		} else if(record.type === 'Container') {
			OWNABLE.add(recordId);
		} else if(record.type === 'Cell' && check) {
			const name = getCellName(record);
			let owned = 0;
			let unowned = 0;
			record.references.forEach(reference => {
				const lowerId = reference.id.toLowerCase();
				if('scale' in reference && reference.scale !== 1 && ITEMS.has(lowerId)) {
					console.log(record.type, name, 'contains', reference.id, 'with scale', reference.scale);
				}
				if(('lock_level' in reference || reference.trap) || ITEMS.has(lowerId) || OWNABLE.has(lowerId)) {
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
		} else if(isItem(record)) {
			ITEMS.add(recordId);
		}
	}
};

'use strict';

const { canCarry } = require('./util');

const ICONS = ['Alchemy', 'Apparatus', 'Armor', 'Book', 'Clothing', 'Ingredient', 'Lockpick', 'MiscItem', 'Probe', 'RepairTool', 'Weapon'];
const MESHES = ['Activator', 'Alchemy', 'Apparatus', 'Armor', 'Book', 'Clothing', 'Container', 'Creature', 'Door', 'Ingredient', 'Lockpick', 'MiscItem', 'Probe', 'RepairTool', 'Static', 'Weapon'];
const NAMES = ['Alchemy', 'Apparatus', 'Armor', 'Book', 'Clothing', 'Container', 'Creature', 'Door', 'Ingredient', 'Lockpick', 'MiscItem', 'Npc', 'Probe', 'RepairTool', 'Weapon'];

function check(record, field) {
	if(!record[field]?.trim()) {
		console.log(record.type, record.id, 'has a missing', field);
	} else if(field !== 'name' && !record[field].includes('.')) {
		console.log(record.type, record.id, 'has invalid', field, record[field]);
	}
}

module.exports = {
	onRecord(record) {
		if(canCarry(record)) {
			check(record, 'icon');
			check(record, 'mesh');
			check(record, 'name');
		} else {
			if(ICONS.includes(record.type)) {
				check(record, 'icon');
			}
			if(MESHES.includes(record.type)) {
				check(record, 'mesh');
			}
			if(NAMES.includes(record.type)) {
				check(record, 'name');
			}
		}
	}
};

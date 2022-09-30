'use strict';

const FLAG_PERSISTENT = 1024;
const FLAG_NPC_AUTO_CALC = 0x10;
const FLAG_SPELL_AUTO_CALC = 1;
const FLAG_CELL_INTERIOR = 1;
const FLAG_KEY = 1;
const OBJECTS = ['Activator', 'Alchemy', 'Apparatus', 'Armor', 'Book', 'Clothing',
	'Container', 'Creature', 'Door', 'Ingredient', 'LevelledCreature', 'LevelledItem',
	'Light', 'Lockpick', 'MiscItem', 'Npc', 'Probe', 'RepairTool', 'Static', 'Weapon'];

function escapeRegExp(string) {
	return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function isDead(record) {
	return record.data?.stats?.health === 0;
}

function isPersistent(record) {
	return Boolean(record.flags?.[1] & FLAG_PERSISTENT);
}

function isAutoCalc(record) {
	if(record.type === 'Npc') {
		return Boolean(record.npc_flags & FLAG_NPC_AUTO_CALC);
	} else if(record.type === 'Spell') {
		return Boolean(record.data?.flags & FLAG_SPELL_AUTO_CALC);
	}
	return false;
}

function getCellName(record) {
	if(record.data?.flags & FLAG_CELL_INTERIOR) {
		return record.id;
	}
	return [record.id || record.region, record.data?.grid?.join(',')].filter(Boolean).join(' ');
}

function isObject(record) {
	return OBJECTS.includes(record.type);
}

function isKey(record) {
	return record.type === 'MiscItem' && record.data?.flags & FLAG_KEY;
}

module.exports = {
	escapeRegExp, isDead, isPersistent, isAutoCalc, getCellName, isObject, isKey
};

'use strict';

const FLAG_PERSISTENT = 1024;
const FLAG_NPC_AUTO_CALC = 0x10;
const FLAG_SPELL_AUTO_CALC = 1;
const FLAG_CELL_INTERIOR = 1;
const FLAG_KEY = 1;
const FLAG_ALL_LEVELS_CREATURE = 0x1;
const FLAG_ALL_LEVELS_ITEM = 0x2;
const FLAG_CARRY = 2;
const FLAG_SERVICE_WEAPON = 0x1;
const FLAG_SERVICE_ARMOR = 0x2;
const FLAG_SERVICE_CLOTHING = 0x4;
const FLAG_SERVICE_BOOKS = 0x8;
const FLAG_SERVICE_INGREDIENTS = 0x10;
const FLAG_SERVICE_LOCKPICKS = 0x20;
const FLAG_SERVICE_PROBES = 0x40;
const FLAG_SERVICE_LIGHTS = 0x80;
const FLAG_SERVICE_APPARATUS = 0x100;
const FLAG_SERVICE_REPAIR_ITEMS = 0x200;
const FLAG_SERVICE_MISC = 0x400;
// const FLAG_SERVICE_SPELLS = 0x0800;
const FLAG_SERVICE_MAGIC_ITEMS = 0x1000;
const FLAG_SERVICE_POTIONS = 0x2000;
// const FLAG_SERVICE_TRAINING = 0x4000;
// const FLAG_SERVICE_SPELLMAKING = 0x8000;
// const FLAG_SERVICE_ENCHANTING = 0x10000;
// const FLAG_SERVICE_REPAIR = 0x20000;
const FLAGS_BARTER = FLAG_SERVICE_WEAPON | FLAG_SERVICE_ARMOR | FLAG_SERVICE_CLOTHING
	| FLAG_SERVICE_BOOKS | FLAG_SERVICE_INGREDIENTS | FLAG_SERVICE_LOCKPICKS | FLAG_SERVICE_PROBES
	| FLAG_SERVICE_LIGHTS | FLAG_SERVICE_APPARATUS | FLAG_SERVICE_REPAIR_ITEMS | FLAG_SERVICE_MISC
	| FLAG_SERVICE_POTIONS;
const OBJECTS = ['Activator', 'Alchemy', 'Apparatus', 'Armor', 'Book', 'Clothing',
	'Container', 'Creature', 'Door', 'Ingredient', 'LevelledCreature', 'LevelledItem',
	'Light', 'Lockpick', 'MiscItem', 'Npc', 'Probe', 'RepairTool', 'Static', 'Weapon'];
const CELL_SIZE = 8192;

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

function isInterior(record) {
	return Boolean(record.type === 'Cell' && record.data?.flags & FLAG_CELL_INTERIOR);
}

function getCellName(record) {
	if(record.type === 'PathGrid') {
		const [x, y] = record.data?.grid ?? [];
		if(!x && !y) {
			return record.cell;
		}
		return [record.cell, record.data.grid.join(',')].filter(Boolean).join(' ');
	}
	if(isInterior(record)) {
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

function getServices(record) {
	if(record.type === 'Class') {
		return record.data?.auto_calc_flags;
	} else if(record.type === 'Creature' || record.type === 'Npc') {
		return record.ai_data?.services;
	}
}

function hasBarterMenu(record) {
	return Boolean(getServices(record) & FLAGS_BARTER);
}

function buysMagicItems(record) {
	return Boolean(getServices(record) & FLAG_SERVICE_MAGIC_ITEMS);
}

function calculatesForAllLevels(record) {
	if(record.type === 'LevelledCreature') {
		return Boolean(record.list_flags & FLAG_ALL_LEVELS_CREATURE);
	} else if(record.type === 'LevelledItem') {
		return Boolean(record.list_flags & FLAG_ALL_LEVELS_ITEM);
	}
	return false;
}

function getCellGrid(x, y) {
	return [Math.floor(x / CELL_SIZE), Math.floor(y / CELL_SIZE)];
}

function canCarry(record) {
	return Boolean(record.type === 'Light' && record.data?.flags & FLAG_CARRY);
}

module.exports = {
	escapeRegExp, isDead, isPersistent, isAutoCalc, getCellName, isObject, isKey,
	hasBarterMenu, buysMagicItems, isInterior, calculatesForAllLevels, CELL_SIZE,
	getCellGrid, canCarry
};

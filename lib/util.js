'use strict';

const FLAG_PERSISTENT = 1024;
const FLAG_NPC_AUTO_CALC = 0x10;
const FLAG_SPELL_AUTO_CALC = 1;

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
	if(record.id) {
		return record.id;
	}
	return [record.region, record.data?.grid?.join(',')].filter(Boolean).join(' ');
}

module.exports = {
	escapeRegExp, isDead, isPersistent, isAutoCalc, getCellName
};

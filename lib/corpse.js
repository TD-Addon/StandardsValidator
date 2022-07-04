'use strict';

const { isDead, isPersistent } = require('./util');

module.exports = {
	onRecord(record) {
		if((record.type === 'Npc' || record.type === 'Creature') && isDead(record) && !isPersistent(record)) {
			console.log(record.type, record.id, 'is dead but does not have corpse persists checked');
		}
	}
};

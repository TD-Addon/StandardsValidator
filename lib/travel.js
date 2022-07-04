'use strict';

const { isDead } = require('./util');

const CLASSES = new Set(require('../data/travel').map(c => c.toLowerCase()));

module.exports = {
	onRecord(record) {
		if(record.type === 'Npc' && !isDead(record) && CLASSES.has(record.class?.toLowerCase()) && !record.travel_destinations?.length) {
			console.log(record.type, record.id, 'has class', record.class, 'but does not offer travel services');
		}
	}
};

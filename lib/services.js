'use strict';

const { hasBarterMenu, buysMagicItems, isAutoCalc } = require('./util');
const BARTER_CLASSES = new Set(require('../data/services').barter.map(c => c.toLowerCase()));

function barters(record) {
	if(record.type === 'Npc' && isAutoCalc(record)) {
		return BARTER_CLASSES.has(record.class.toLowerCase());
	}
	return hasBarterMenu(record);
}

module.exports = {
	onRecord(record) {
		const isClass = record.type === 'Class';
		if(isClass || ['Creature', 'Npc'].includes(record.type)) {
			if(barters(record)) {
				if(isClass) {
					BARTER_CLASSES.add(record.id.toLowerCase());
				} else {
					if(!record.data?.gold) {
						console.log(record.type, record.id, 'does not have any barter gold');
					}
					if(record.type === 'Npc' && !hasBarterMenu(record)) {
						console.log(record.type, record.id, 'has class', record.class, 'but does not barter');
					}
				}
			} else {
				if(buysMagicItems(record)) {
					console.log(record.type, record.id, 'buys magic items but does not have a barter menu');
				}
				if(isClass && BARTER_CLASSES.has(record.id.toLowerCase())) {
					console.log(record.type, record.id, 'does not barter');
				}
			}
		}
	}
};

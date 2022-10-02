'use strict';

const { getCellName } = require('./util');

const CHESTS = Object.entries(require('../data/supplies')).reduce((o, [item, faction]) => {
	o[item.toLowerCase()] = faction.toLowerCase();
	return o;
}, {});
const ALL_RANKS = 4294967295;

module.exports = {
	onCellRef(record, ref, id) {
		const faction = CHESTS[id];
		if(faction) {
			if(ref.owner_faction?.toLowerCase() !== faction) {
				console.log(record.type, getCellName(record), 'contains', ref.id, 'not owned by the', faction);
			} else if(ref.owner_faction_rank && ref.owner_faction_rank !== ALL_RANKS) {
				console.log(record.type, getCellName(record), 'contains', ref.id, 'not available to all ranks');
			}
		}
	}
};

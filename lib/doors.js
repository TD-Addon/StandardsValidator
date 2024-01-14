'use strict';

const { getCellName } = require('./util');

module.exports = {
	onRecord(record) {
		if(record.type === 'Door' && record.mesh?.toLowerCase() === 'i\\in_lava_blacksquare.nif') {
			console.log(record.type, record.id, 'uses mesh', record.mesh);
		}
	},
	onCellRef(record, ref, id) {
		if(id === 'prisonmarker' && !ref.door_destination_cell) {
			console.log(record.type, getCellName(record), 'contains an unlinked', ref.id);
		}
	}
};

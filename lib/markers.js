'use strict';

const { getCellName } = require('./util');

module.exports = {
	onCellRef(record, ref, id) {
		if(id === 'prisonmarker' && !ref.door_destination_cell) {
			console.log(record.type, getCellName(record), 'contains an unlinked', ref.id);
		}
	}
};

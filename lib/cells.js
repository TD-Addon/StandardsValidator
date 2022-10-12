'use strict';

const PREFIXES = require('../data/projects').map(p => p.prefix.toLowerCase());
const { isInterior } = require('./util');

module.exports = {
	onRecord(record, mode, id) {
		if(isInterior(record) && !record.atmosphere_data?.fog_density && !PREFIXES.some(prefix => id.startsWith(prefix))) {
			console.log(record.type, record.id, 'has a fog density of 0');
		}
	}
};

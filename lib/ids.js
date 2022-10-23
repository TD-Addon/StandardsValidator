'use strict';

const PROJECTS = require('../data/projects').map(p => ({ ...p, prefix: p.prefix.toLowerCase() }));
const TD = PROJECTS.find(p => p.prefix === 't_');
const FLAG_FEMALE = 1;
const KNOWN_IDS = {};

function checkId(record, mode, id) {
	const project = PROJECTS.find(p => id.startsWith(p.prefix));
	if(!project) {
		console.log(record.type, record.id, 'does not match a known ID scheme');
	} else if(project === TD && mode !== 'TD') {
		console.log(record.type, record.id, 'has a', TD.name, 'ID');
	}
}

module.exports = {
	onRecord(record, mode, recordId) {
		if(record.type === 'Bodypart') {
			if(record.data?.vampire && record.data.part === 'Head') {
				const id = `b_v_${record.name}_${record.data.female & FLAG_FEMALE ? 'f' : 'm'}_head_01`;
				if(record.id !== id) {
					console.log(record.type, record.id, 'should have id', id);
				}
			} else {
				checkId(record, mode, recordId);
			}
		} else if(record.id && !['Cell', 'Dialogue', 'LandscapeTexture', 'Region', 'StartScript'].includes(record.type)) {
			checkId(record, mode, recordId);
		}
		if(record.type !== 'Cell' && recordId) {
			if(recordId in KNOWN_IDS) {
				console.log(record.type, record.id, 'shares its ID with a record of type', KNOWN_IDS[recordId].type);
			} else {
				KNOWN_IDS[recordId] = record;
			}
		}
	}
};

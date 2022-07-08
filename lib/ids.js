'use strict';

const PROJECTS = require('../data/projects').map(p => ({ ...p, prefix: p.prefix.toLowerCase() }));
const TD = PROJECTS.find(p => p.prefix === 'T_');
const FEMALE = 1;

let dependsOnTD = false;

function checkId(record, mode) {
	const id = record.id.toLowerCase();
	const project = PROJECTS.find(p => id.startsWith(p.prefix));
	if(!project) {
		console.log(record.type, record.id, 'does not match a known ID scheme');
	} else if(project === TD && dependsOnTD && mode !== 'TD') {
		console.log(record.type, record.id, 'has a', TD.name, 'ID');
	}
}

module.exports = {
	onRecord(record, topic, mode) {
		if(record.type === 'Header') {
			if(record.masters?.some(([file]) => /^Tamriel_Data.esm$/i.test(file))) {
				dependsOnTD = true;
			}
		} else if(record.type === 'Bodypart') {
			if(record.data?.vampire && record.data.part === 'Head') {
				const id = `b_v_${record.name}_${record.data.female & FEMALE ? 'f' : 'm'}_head_01`;
				if(record.id !== id) {
					console.log(record.type, record.id, 'should have id', id);
				}
			} else {
				checkId(record, mode);
			}
		} else if(record.id && !['Cell', 'Dialogue', 'LandscapeTexture', 'Region'].includes(record.type)) {
			checkId(record, mode);
		}
	}
};

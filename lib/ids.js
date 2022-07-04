'use strict';

const PROJECTS = require('../data/projects').map(p => ({ ...p, prefix: p.prefix.toLowerCase() }));
const TD = PROJECTS.find(p => p.prefix === 'T_');

let dependsOnTD = false;

module.exports = {
	onRecord(record, topic, mode) {
		if(record.type === 'Header') {
			if(record.masters?.some(([file]) => /^Tamriel_Data.esm$/i.test(file))) {
				dependsOnTD = true;
			}
		} else if(record.id && !['Cell', 'Dialogue', 'LandscapeTexture', 'Region'].includes(record.type)) {
			const id = record.id.toLowerCase();
			const project = PROJECTS.find(p => id.startsWith(p.prefix));
			if(!project) {
				console.log(record.type, record.id, 'does not match a known ID scheme');
			} else if(project === TD && dependsOnTD && mode !== 'TD') {
				console.log(record.type, record.id, 'has a', TD.name, 'ID');
			}
		}
	}
};

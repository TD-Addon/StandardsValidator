'use strict';

const { escapeRegExp, isDead } = require('./util');
const PROJECTS = require('../data/projects').map(p => {
	if(p.local) {
		return { ...p, regexp: new RegExp(`\n[\\s,]*short[\\s,]+(${escapeRegExp(p.local)})[\\s,]*(;*.?)\n`, 'i') };
	}
	return p;
});

const scripts = {};

module.exports = {
	onRecord(record) {
		if(record.type === 'Script') {
			const projects = [];
			scripts[record.id.toLowerCase()] = {
				npc: /\n[\s,]*short[\s,]+(T_Local_NPC)[\s,]*(;*.?)\n/i.test(record.text),
				khajiit: /\n[\s,]*short[\s,]+(T_Local_Khajiit)[\s,]*(;*.?)\n/i.test(record.text),
				nolore: /\n[\s,]*short[\s,]+(NoLore)[\s,]*(;*.?)\n/i.test(record.text),
				projects
			};
			PROJECTS.forEach(project => {
				if(project.regexp?.test(record.text)) {
					projects.push(project.local);
				}
			});
		} else if(record.type === 'Npc' && !isDead(record)) {
			if(!record.script) {
				console.log(record.type, record.id, 'does not have a script');
			} else {
				const id = record.script.toLowerCase();
				if(id in scripts) {
					const race = record.race.toLowerCase();
					if(race === 'khajiit' || race.startsWith('t_els_')) {
						const { npc, nolore, khajiit, projects } = scripts[id];
						if(!npc) {
							console.log(record.type, record.id, 'uses script', record.script, 'which does not define T_Local_NPC');
						}
						if(!nolore) {
							console.log(record.type, record.id, 'uses script', record.script, 'which does not define NoLore');
						}
						if(!khajiit) {
							console.log(record.type, record.id, 'uses script', record.script, 'which does not define T_Local_Khajiit');
						}
						if(!projects.length) {
							console.log(record.type, record.id, 'uses script', record.script, 'which does not define any province specific local variables');
						} else if(projects.length > 1) {
							console.log(record.type, record.id, 'uses script', record.script, 'which defines', projects.join(' and '));
						}
					}
				} else if(!id.startsWith('t_scnpc_')) {
					console.log(record.type, record.id, 'uses unknown script', record.script);
				}
			}
		}
	}
};

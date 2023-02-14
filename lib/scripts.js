'use strict';

const fs = require('fs');
const { escapeRegExp, isDead } = require('./util');

function getVariable(name, type = 'short') {
	return new RegExp(`\n[\\s,]*${type}[\\s,]+(${name})[\\s,]*(;*.?)\n`, 'i');
}

const PROJECTS = require('../data/projects').map(p => {
	if(p.local) {
		return { ...p, regexp: getVariable(escapeRegExp(p.local)) };
	}
	return p;
});
const NPC = getVariable('T_Local_NPC');
const KHAJIIT = getVariable('T_Local_Khajiit');
const NOLORE = getVariable('NoLore');
const COMMANDS = getVariable(`(${fs.readFileSync(__dirname + '/../data/mwscript.returning.txt', 'utf-8')
	.trim().replace(/\r/g, '').split('\n').filter(Boolean).map(escapeRegExp).join('|')})`, '(short|long|float)');

const scripts = {};

module.exports = {
	onRecord(record, mode, recordId) {
		if(record.type === 'Script') {
			const projects = [];
			scripts[recordId] = {
				npc: NPC.test(record.text),
				khajiit: KHAJIIT.test(record.text),
				nolore: NOLORE.test(record.text),
				projects
			};
			PROJECTS.forEach(project => {
				if(project.regexp?.test(record.text)) {
					projects.push(project.local);
				}
			});
			const result = COMMANDS.exec(record.text);
			if(result) {
				console.log(record.type, record.id, 'contains line', result[0].trim());
			}
		} else if(record.type === 'Npc' && !isDead(record)) {
			if(!record.script) {
				console.log(record.type, record.id, 'does not have a script');
			} else {
				const id = record.script.toLowerCase();
				if(id in scripts) {
					scripts[id].used = true;
					const { npc, nolore, khajiit, projects } = scripts[id];
					if(!npc) {
						console.log(record.type, record.id, 'uses script', record.script, 'which does not define T_Local_NPC');
					}
					if(!nolore) {
						console.log(record.type, record.id, 'uses script', record.script, 'which does not define NoLore');
					}
					const race = record.race.toLowerCase();
					if(race === 'khajiit' || race.startsWith('t_els_')) {
						scripts[id].usedByKhajiit = true;
						if(!khajiit) {
							console.log(record.type, record.id, 'uses script', record.script, 'which does not define T_Local_Khajiit');
						}
					}
					if(!projects.length) {
						console.log(record.type, record.id, 'uses script', record.script, 'which does not define any province specific local variables');
					} else if(projects.length > 1) {
						console.log(record.type, record.id, 'uses script', record.script, 'which defines', projects.join(' and '));
					}
				} else if(!id.startsWith('t_scnpc_')) {
					console.log(record.type, record.id, 'uses unknown script', record.script);
				}
			}
		}
	},
	onScriptLine(record, line, comment, topic) {
		if(line) {
			const position = /^([,\s]*|.*?->[,\s]*)position[,\s]+/.exec(line);
			if(position) {
				if(record.type === 'Info') {
					console.log(record.type, record.info_id, 'in topic', topic.id, 'uses Position instead of PositionCell');
				} else {
					console.log(record.type, record.id, 'uses Position instead of PositionCell');
				}
			}
		}
	},
	onEnd(mode) {
		if(mode !== 'TD') {
			for(const id in scripts) {
				const script = scripts[id];
				if(script.used && script.khajiit && !script.usedByKhajiit) {
					console.log('Script', id, 'defines T_Local_Khajiit but is not used by any khajiit');
				}
			}
		}
	}
};

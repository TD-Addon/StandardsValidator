'use strict';

const { isObject } = require('./util');

const SCRIPT_IDS = new Set();
const START_SCRIPTS = [];
const OBJECTS = {};
const USED_OBJECTS = new Set();
const ENCHANTMENTS = new Set();
const USED_ENCHANTMENTS = new Set();
const JOURNALS = {};
const USED_JOURNALS = {};

module.exports = {
	onRecord(record, mode, recordId) {
		if(mode === 'TD') {
			return;
		}
		if(record.type === 'Script') {
			SCRIPT_IDS.add(recordId);
		} else if(record.type === 'Enchantment') {
			ENCHANTMENTS.add(recordId);
		} else if(record.type === 'Dialogue') {
			if(record.dialogue_type === 'Journal') {
				JOURNALS[recordId] = new Set();
			}
		} else {
			if(record.script) {
				SCRIPT_IDS.delete(record.script.toLowerCase());
			}
			if(record.enchanting) {
				USED_ENCHANTMENTS.add(record.enchanting.toLowerCase());
			}
			if(isObject(record)) {
				OBJECTS[recordId] = record;
			}
		}
	},
	onInfo(record, topic) {
		if(topic.dialogue_type === 'Journal' && !('quest_name' in record)) {
			JOURNALS[topic.id.toLowerCase()].add(record.data.disposition);
		}
	},
	onCellRef(record, reference, id) {
		delete OBJECTS[id];
	},
	onLevelled(record, entry, id) {
		USED_OBJECTS.add(id);
	},
	onInventory(record, entry, id) {
		USED_OBJECTS.add(id);
	},
	onScriptLine(record, line) {
		if(!line) {
			return;
		}
		const startScript = /^([,\s]*|.*?->[,\s]*)startscript[,\s]+("[^"]+"|[^,\s]+)[,\s]*$/.exec(line);
		if(startScript) {
			const [, , id] = startScript;
			return START_SCRIPTS.push(id.replace(/"/g, ''));
		}
		const first = /^([,\s]*|.*?->[,\s]*)(placeatme|addsoulgem|additem|equip|drop|placeatpc|placeitemcell|placeitem)[,\s]+(?:("[^"]+"?)(?:.*)|([^,\s"]+)(?:[,\s]+|$))/.exec(line)
			?? /^([,\s]*|.*?->[,\s]*)(journal|setjournalindex)[,\s]+(?:("[^"]+"?)|([^,\s"]+))[,\s]+([\d]+)/.exec(line);
		if(first) {
			const [, , command, quotedId, unquotedId, argument] = first;
			let id = unquotedId;
			if(quotedId) {
				id = quotedId.replace(/"/g, '');
			}
			if(command === 'journal' || command === 'setjournalindex') {
				if(!USED_JOURNALS[id]) {
					USED_JOURNALS[id] = [];
				}
				return USED_JOURNALS[id].push(+argument);
			}
			return USED_OBJECTS.add(id);
		}
		const second = /^([,\s]*|.*?->[,\s]*)(addtolevcreature|addtolevitem)[,\s]+("[^"]+"|[^,\s]+)[,\s]+("[^"]+"|[^,\s]+)([,\s]+|$)/.exec(line);
		if(second) {
			const [, , , , id] = second;
			return USED_OBJECTS.add(id.replace(/"/g, ''));
		}
	},
	onEnd() {
		START_SCRIPTS.forEach(id => {
			SCRIPT_IDS.delete(id);
		});
		SCRIPT_IDS.forEach(id => {
			console.log('Script', id, 'is never started');
		});
		ENCHANTMENTS.forEach(id => {
			if(!USED_ENCHANTMENTS.has(id)) {
				console.log('Enchantment', id, 'is not used');
			}
		});
		USED_OBJECTS.forEach(id => {
			delete OBJECTS[id];
		});
		Object.values(OBJECTS).forEach(record => {
			console.log(record.type, record.id, 'is not used');
		});
		for(const id in JOURNALS) {
			const used = USED_JOURNALS[id];
			if(used) {
				JOURNALS[id].forEach(index => {
					if(!used.includes(index)) {
						console.log('Journal index', index, 'in', id, 'is unused');
					}
				});
			} else {
				console.log('Journal', id, 'is not used');
			}
		}
	}
};

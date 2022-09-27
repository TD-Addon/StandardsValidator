'use strict';

const { isObject } = require('./util');

const SCRIPT_IDS = new Set();
const START_SCRIPTS = [];
const OBJECTS = {};
const USED_OBJECTS = new Set();
const ENCHANTMENTS = new Set();
const USED_ENCHANTMENTS = new Set();

module.exports = {
	onRecord(record, mode) {
		if(mode === 'TD') {
			return;
		}
		if(record.type === 'Script') {
			SCRIPT_IDS.add(record.id.toLowerCase());
		} else if(record.type === 'Enchantment') {
			ENCHANTMENTS.add(record.id.toLowerCase());
		} else {
			if(record.script) {
				SCRIPT_IDS.delete(record.script.toLowerCase());
			}
			if(record.enchanting) {
				USED_ENCHANTMENTS.add(record.enchanting.toLowerCase());
			}
			if(isObject(record)) {
				OBJECTS[record.id.toLowerCase()] = record;
			}
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
		const first = /^([,\s]*|.*?->[,\s]*)(placeatme|addsoulgem|additem|equip|drop|placeatpc|placeitemcell|placeitem)[,\s]+(?:("[^"]+"?)(?:.*)|([^,\s"]+)(?:[,\s]+|$))/.exec(line);
		if(first) {
			const [, , , quotedId, unquotedId] = first;
			let id = unquotedId;
			if(quotedId) {
				id = quotedId.replace(/"/g, '');
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
		USED_ENCHANTMENTS.forEach(id => {
			ENCHANTMENTS.delete(id);
		});
		ENCHANTMENTS.forEach(id => {
			console.log('Enchantment', id, 'is not used');
		});
		USED_OBJECTS.forEach(id => {
			delete OBJECTS[id];
		});
		Object.values(OBJECTS).forEach(record => {
			console.log(record.type, record.id, 'is not used');
		});
	}
};

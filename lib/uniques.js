'use strict';

const fs = require('fs');
const { escapeRegExp, getCellName } = require('./util');

const UNIQUES = new Set(fs.readFileSync(__dirname + '/../data/uniques.txt', 'utf-8').trim().replace(/\r/g, '').split('\n'));

function report(item, id, type) {
	console.log(type, id, 'references', item);
	return true;
}

function check(itemId, item, recordId, type) {
	if(UNIQUES.has(itemId)) {
		report(item, recordId, type);
	}
}

const REGEXP_CACHE = {};

function getRegExp(item) {
	if(!(item in REGEXP_CACHE)) {
		REGEXP_CACHE[item] = new RegExp(`[ ,"]${escapeRegExp(item)}($|[ ,"])`);
	}
	return REGEXP_CACHE[item];
}

function checkScriptLine(line, item) {
	if(line.includes(item)) {
		return getRegExp(item).test(line);
	}
	return false;
}

function checkScript(script, id, type) {
	if(script) {
		const lines = script.toLowerCase().trim().split('\n').map(line => {
			const commentStart = line.indexOf(';');
			if(commentStart >= 0) {
				line = line.slice(0, commentStart);
			}
			line = line.trim();
			if(/placeatme|addtolevcreature|addtolevitem|addsoulgem|addspell|cast|explodespell|dropsoulgem|additem|equip|drop|placeatpc|placeitem|placeitemcell/.test(line)) {
				return line;
			}
			return '';
		}).filter(Boolean);
		if(lines.length) {
			for(const uni of UNIQUES) {
				if(lines.some(line => checkScriptLine(line, uni))) {
					return report(uni, id, type);
				}
			}
		}
	}
	return false;
}

module.exports = {
	onRecord(record) {
		if(['Armor', 'Book', 'Clothing', 'Weapon'].includes(record.type)) {
			if(record.enchanting) {
				check(record.enchanting.toLowerCase(), record.enchanting, record.id, record.type);
			}
		} else if(record.type === 'Script') {
			checkScript(record.text, record.id, record.type);
		}
	},
	onCellRef(record, ref, id) {
		if(UNIQUES.has(id)) {
			report(id, getCellName(record), record.type);
		}
	},
	onLevelled(record, [item], id) {
		check(id, item, record.id, record.type);
	},
	onInventory(record, [, item], id) {
		check(id, item, record.id, record.type);
	},
	onInfo(record, currentTopic) {
		checkScript(record.result, `${currentTopic?.id} ${record.info_id}`, record.type);
	}
};

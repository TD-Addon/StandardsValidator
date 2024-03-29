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

module.exports = {
	onRecord(record) {
		if(['Armor', 'Book', 'Clothing', 'Weapon'].includes(record.type)) {
			if(record.enchanting) {
				check(record.enchanting.toLowerCase(), record.enchanting, record.id, record.type);
			}
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
	onScriptLine(record, line, comment, topic) {
		if(/placeatme|addtolevcreature|addtolevitem|addsoulgem|addspell|cast|explodespell|dropsoulgem|additem|equip|drop|placeatpc|placeitem|placeitemcell/.test(line)) {
			for(const uni of UNIQUES) {
				if(checkScriptLine(line, uni)) {
					return report(uni, topic ? `${record.info_id} in topic ${topic.id}` : record.id, record.type);
				}
			}
		}
	}
};

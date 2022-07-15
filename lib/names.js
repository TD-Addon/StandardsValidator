'use strict';

const levenshtein = require('js-levenshtein');
const NAMES = [];
const DISTANCE_DIV = 7;

function onRecord(record) {
	if(record.type === 'Npc' && record.name) {
		const name = record.name.toLowerCase();
		const minDistance = Math.round(name.length / DISTANCE_DIV);
		if(minDistance < 1) {
			return;
		}
		for(const other of NAMES) {
			if(other.name === name) {
				continue;
			}
			const distance = levenshtein(name, other.name);
			if(distance <= minDistance) {
				console.log(record.type, record.id, `(${record.name})`, 'has a name similar to', other.record.id, `(${other.record.name})`, distance);
				break;
			}
		}
		NAMES.push({ name, record });
	}
}

module.exports = {
	onRecord: levenshtein ? onRecord : () => void 0
};

'use strict';

const { getCellName } = require('./util');
// Increase to check square distance between references
const THRESHOLD = 0;

function equals(array1, array2) {
	if(array1.length === array2.length) {
		for(let i = 0; i < array1.length; i++) {
			if(array1[i] !== array2[i]) {
				return false;
			}
		}
		return true;
	}
	return false;
}

function distance([x1, y1, z1], [x2, y2, z2]) {
	const d2 = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2) + (z1 - z2) * (z1 - z2);
	return Math.abs(d2) <= THRESHOLD;
}

const translation = THRESHOLD === 0 ? equals : distance;

module.exports = {
	onCellRef(record, reference, id, start) {
		if('deleted' in reference) {
			return;
		}
		for(let i = start + 1; i < record.references.length; i++) {
			const other = record.references[i];
			if('deleted' in other) {
				continue;
			}
			if(other.id.toLowerCase() === id && equals(reference.rotation, other.rotation) && (reference.scale ?? 1) === (other.scale ?? 1) && translation(reference.translation, other.translation)) {
				console.log(record.type, getCellName(record), 'contains duplicate reference', reference.id, 'at position', reference.translation, other.translation);
			}
		}
	}
};

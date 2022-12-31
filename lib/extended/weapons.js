'use strict';

const BY_MESH = {};
const IGNORES_RESIST = 1;
const SILVER = 2;

module.exports = {
	onRecord(record, check, id) {
		if(record.type === 'Weapon' && record.mesh) {
			if(record.name?.toLowerCase() === '<deprecated>') {
				return;
			}
			const mesh = record.mesh.toLowerCase();
			const ignores = record.enchanting ? undefined : Boolean(record.data.flags & IGNORES_RESIST);
			const silver = Boolean(record.data.flags & SILVER);
			const other = BY_MESH[mesh];
			if(!other || other?.id === id || other?.record.enchanting && !record.enchanting) {
				BY_MESH[mesh] = { ignores, silver, record, id };
			} else {
				if(check && other.silver !== silver) {
					console.log(record.type, record.id, 'has a different silver value than', other.record.id);
				}
				if(other.ignores === undefined && typeof ignores === 'boolean') {
					other.ignores = ignores;
					other.record = record;
				}
				if(check && other.ignores !== ignores && other.ignores !== undefined && ignores !== undefined) {
					console.log(record.type, record.id, 'has a different ignores normal weapon resistance value than', other.record.id);
				}
			}
		}
	}
};

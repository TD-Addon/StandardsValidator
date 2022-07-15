'use strict';

const SOUND_GENS = new Set();
const TO_CHECK = [];

module.exports = {
	onRecord(record) {
		if(record.type === 'Creature') {
			if(!record.sound) {
				TO_CHECK.push(record);
			}
		} else if(record.type === 'SoundGen' && record.creature) {
			SOUND_GENS.add(record.creature.toLowerCase());
		}
	},
	onEnd() {
		TO_CHECK.forEach(record => {
			if(!SOUND_GENS.has(record.id.toLowerCase())) {
				console.log(record.type, record.id, 'is missing a sound gen');
			}
		});
	}
};

'use strict';

module.exports = {
	onRecord(record, topic) {
		if(record.type === 'Info') {
			if(!record.text?.length) {
				if(!['Journal', 'Voice'].includes(record.data?.dialogue_type)) {
					console.log(record.type, record.info_id, 'in topic', topic.id, 'has no text');
				}
			} else if(/[^\S\r\n]{2,}/.test(record.text)) {
				console.log(record.type, record.info_id, 'in topic', topic.id, 'contains double spaces');
			}
			if(record.speaker_id) {
				if(record.speaker_rank) {
					console.log(record.type, record.info_id, 'in topic', topic.id, 'has an unnecessary race filter');
				}
				if(record.speaker_class) {
					console.log(record.type, record.info_id, 'in topic', topic.id, 'has an unnecessary class filter');
				}
				if(record.speaker_faction) {
					console.log(record.type, record.info_id, 'in topic', topic.id, 'has an unnecessary faction filter');
				}
				if(record.data?.speaker_sex && record.data.speaker_sex !== 'Any') {
					console.log(record.type, record.info_id, 'in topic', topic.id, 'has an unnecessary sex filter');
				}
			}
		}
	}
};

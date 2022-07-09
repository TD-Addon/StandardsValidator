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
		}
	}
};

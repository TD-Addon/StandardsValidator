'use strict';

function check(record, field, topic) {
	if(record[field]) {
		const index = record[field].search(/[\u0000-\u0008\u000b\u000c\u000e-\u001f\u007f-\uffff]/);
		if(index >= 0) {
			let { id } = record;
			if(record.type === 'Info') {
				id = `${record.info_id} in topic ${topic.id}`;
			}
			console.log(record.type, id, 'contains odd character', record[field][index], 'in field', field);
		}
	}
}

module.exports = {
	onRecord(record) {
		if(record.type === 'Script') {
			check(record, 'text');
		} else if(record.type !== 'Info') {
			if(typeof record.id === 'string') {
				check(record, 'id');
			}
			if(typeof record.name === 'string') {
				check(record, 'name');
			}
			if(typeof record.text === 'string') {
				check(record, 'text');
			}
		}
	},
	onInfo(record, topic) {
		check(record, 'text', topic);
		check(record, 'result', topic);
	}
};

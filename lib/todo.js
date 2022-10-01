'use strict';

module.exports = {
	onScriptLine(record, line, comment, topic) {
		if(/(^(todo|fixme))|(^|\s)merge/i.test(comment)) {
			console.log(record.type, topic ? `${record.info_id} in topic ${topic.id}` : record.id, 'contains comment', comment);
		}
	}
};

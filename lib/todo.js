'use strict';

module.exports = {
	onScriptLine(record, line, comment, topic) {
		if(/(^(todo|fixme))|(^|\s)merge/i.test(comment)) {
			console.log(record.type, topic ? `${topic.id} ${record.info_id}` : record.id, 'contains comment', comment);
		}
	}
};

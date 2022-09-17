'use strict';

const TO_CHECK = new Set();
const START_SCRIPTS = [];

function addScriptLines(script) {
	if(script) {
		const lines = script.toLowerCase().trim().split('\n');
		lines.forEach(line => {
			const commentStart = line.indexOf(';');
			if(commentStart >= 0) {
				line = line.slice(0, commentStart);
			}
			const results = /^[,\s]*startscript[,\s]*("[^,\s"]+"|[^,\s]+)[,\s]*$/.exec(line);
			if(results) {
				let [, id] = results;
				START_SCRIPTS.push(id.replace(/"/g, ''));
			}
		});
	}
}

module.exports = {
	onRecord(record, topic, mode) {
		if(mode === 'TD') {
			return;
		}
		if(record.type === 'Script') {
			TO_CHECK.add(record.id.toLowerCase());
			addScriptLines(record.text);
		} else if(record.type === 'Info') {
			addScriptLines(record.result);
		} else if(record.script) {
			TO_CHECK.delete(record.script.toLowerCase());
		}
	},
	onEnd() {
		START_SCRIPTS.forEach(id => {
			TO_CHECK.delete(id);
		})
		TO_CHECK.forEach(id => {
			console.log('Script', id, 'is never started');
		});
	}
};

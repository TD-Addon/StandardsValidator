'use strict';

const TAGS = ['div', 'font', 'br', 'p', 'img', 'b', 'deprecated']; //ok, so maybe that last one isn't real

function isWhitespace(c) {
	return /^\s$/.test(c);
}

function parse(html, listener) {
	let state;
	let tag = '';
	let attribute = '';
	let value = '';
	const stack = [];
	for(let i = 0; i < html.length; i++) {
		const c = html[i];
		if(!state) {
			if(c === '<') {
				state = 'opentag';
				tag = '';
			}
		} else if(state === 'opentag') {
			if(c === '/') {
				if(stack.length) {
					state = 'closetag';
					tag = '';
				} else {
					throw new Error(`Unexpected / at index ${i}`);
				}
			} else if(c === '<') {
				throw new Error(`Unexpected < at index ${i}`);
			} else if(c === '>') {
				listener.onElement(tag);
				state = undefined;
				stack.push(tag);
			} else if(isWhitespace(c)) {
				listener.onElement(tag);
				state = 'attributes';
				attribute = '';
			} else {
				tag += c;
			}
		} else if(state === 'attributes') {
			if(c === '=') {
				if(!attribute) {
					throw new Error(`Unexpected = at index ${i}`);
				}
				state = 'attributevalue';
				value = '';
			} else if(c === '/') {
				state = 'selfclosing';
			} else if(c === '>') {
				listener.onElement(tag);
				state = undefined;
				stack.push(tag);
			} else if(isWhitespace(c)) {
				if(attribute) {
					listener.onAttribute(attribute, value);
					attribute = '';
					value = '';
				}
			} else {
				attribute += c;
			}
		} else if(state === 'selfclosing') {
			if(c !== '>') {
				throw new Error(`Expected > at index ${i}`);
			}
			listener.onClose(stack.pop());
			state = undefined;
		} else if(state === 'attributevalue') {
			if(c === '"') {
				if(value[0] === '"') {
					listener.onAttribute(attribute, value.slice(1));
					state = 'attributes';
					attribute = '';
					value = '';
				} else if(value) {
					throw new Error(`Unexpected " at index ${i}`);
				}
				value = c;
			} else if(value[0] !== '"') {
				if(c === '/') {
					state = 'selfclosing';
				} else if(c === '>') {
					listener.onElement(tag);
					state = undefined;
					stack.push(tag);
				} else if(isWhitespace(c)) {
					if(!value) {
						throw new Error(`Unexpected space at index ${i}`);
					}
					listener.onAttribute(attribute, value);
					state = 'attributes';
					attribute = '';
					value = '';
				} else {
					value += c;
				}
			} else {
				value += c;
			}
		} else if(state === 'closetag') {
			if(c === '>') {
				const initial = stack.pop();
				let expected = initial;
				while(stack.length && expected !== tag) {
					expected = stack.pop();
				}
				if(tag !== expected) {
					throw new Error(`Unexpected </${tag}> expected </${initial}> at index ${i}`);
				}
				listener.onClose(initial);
				state = undefined;
			} else {
				tag += c;
			}
		}
	}
}

module.exports = {
	onRecord(record) {
		if(record.type === 'Book' && record.text) {
			try {
				let isImg = false;
				parse(record.text, {
					onElement(tag) {
						const lower = tag.toLowerCase();
						if(!TAGS.includes(lower)) {
							console.log(record.type, record.id, `contains invalid HTML opening tag <${tag}>`);
						}
						isImg = lower === 'img';
					},
					onAttribute(attribute, value) {
						if(isImg && attribute.toLowerCase() === 'src' && value.includes('/')) {
							console.log(record.type, record.id, 'contains invalid IMG SRC', value);
						}
					},
					onClose(tag) {
						if(!TAGS.includes(tag.toLowerCase())) {
							console.log(record.type, record.id, `contains invalid HTML closing tag </${tag}>`);
						}
						isImg = false;
					}
				});
			} catch(err) {
				console.error('Failed to parse HTML in', record.id, err.message);
			}
		}
	}
};

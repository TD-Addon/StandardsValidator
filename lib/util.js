'use strict';

const FLAG_PERSISTENT = 1024;

function escapeRegExp(string) {
	return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function isDead(record) {
	return record.data?.stats?.health === 0;
}

function isPersistent(record) {
	return Boolean(record.flags?.[1] & FLAG_PERSISTENT);
}

module.exports = {
	escapeRegExp, isDead, isPersistent
};

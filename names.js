'use strict';

const validator = require('./lib/extended/names');
const { runValidator } = require('./lib/extended/program');

runValidator('names.js', validator);

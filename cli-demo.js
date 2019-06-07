#!/usr/bin/env node

const fFile = require('.'); // Begin search once loading
let s = process.uptime();
console.log(fFile.find('./'));
console.log('cost time:', process.uptime() - s, 's');

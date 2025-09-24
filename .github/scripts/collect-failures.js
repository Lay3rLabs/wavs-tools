const fs = require('fs');
const { countFail } = require('./utils');

const xmlReport = fs.readFileSync('tests/.test-reports/junit.xml', 'utf8');

console.log(countFail(xmlReport));
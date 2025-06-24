const fs = require('fs');
const {countFail} = require('./utils');

const report = JSON.parse(fs.readFileSync('tests/.test-reports/merged-report.json', 'utf8'));

console.log(countFail(report));
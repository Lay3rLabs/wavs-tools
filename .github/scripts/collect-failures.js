const fs = require('fs');
const { countFail } = require('./utils');

const reportPath = 'tests/.test-reports/junit.xml';

try {
    const xmlReport = fs.readFileSync(reportPath, 'utf8');

    if (!xmlReport.trim()) {
        console.error('Test report is empty, treating as failure');
        console.log(1);
        process.exit(0);
    }

    if (!xmlReport.includes('<testsuite') && !xmlReport.includes('<testsuites')) {
        console.error('Test report does not contain valid JUnit XML, treating as failure');
        console.log(1);
        process.exit(0);
    }

    console.log(countFail(xmlReport));
} catch (err) {
    console.error(`Failed to read test report: ${err.message}`);
    console.log(1);
    process.exit(0);
}

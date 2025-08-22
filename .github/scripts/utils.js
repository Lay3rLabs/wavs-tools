
function countFail(xmlString) {
    // Count failures in JUnit XML by counting <failure> and <error> tags
    const failureMatches = xmlString.match(/<failure[^>]*>/g) || [];
    const errorMatches = xmlString.match(/<error[^>]*>/g) || [];
    
    return failureMatches.length + errorMatches.length;
}

function parseJUnitXML(xmlString) {
    // Simple JUnit XML parser
    const testsuiteMatch = xmlString.match(/<testsuite[^>]*>/);
    if (!testsuiteMatch) {
        return { tests: 0, failures: 0, errors: 0, skipped: 0, time: 0 };
    }
    
    const attributes = testsuiteMatch[0];
    const tests = parseInt(attributes.match(/tests="(\d+)"/)?.[1] || '0');
    const failures = parseInt(attributes.match(/failures="(\d+)"/)?.[1] || '0');
    const errors = parseInt(attributes.match(/errors="(\d+)"/)?.[1] || '0');
    const skipped = parseInt(attributes.match(/skipped="(\d+)"/)?.[1] || '0');
    const time = parseFloat(attributes.match(/time="([^"]+)"/)?.[1] || '0');
    
    return { tests, failures, errors, skipped, time };
}

module.exports = {
    countFail,
    parseJUnitXML,
};
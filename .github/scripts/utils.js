function countFail(xmlString) {
    // Count failures in JUnit XML by counting <failure> and <error> tags
    const failureMatches = xmlString.match(/<failure[^>]*>/g) || [];
    const errorMatches = xmlString.match(/<error[^>]*>/g) || [];

    return failureMatches.length + errorMatches.length;
}

function parseJUnitXML(xmlString) {
    // Prefer top-level <testsuites ...> if present (as in jest-junit/bun test)
    const testsuitesMatch = xmlString.match(/<testsuites[^>]*>/);
    if (testsuitesMatch) {
        const attributes = testsuitesMatch[0];
        const tests = parseInt(attributes.match(/tests=\"(\d+)\"/)?.[1] || '0');
        const failures = parseInt(attributes.match(/failures=\"(\d+)\"/)?.[1] || '0');
        const errors = parseInt(attributes.match(/errors=\"(\d+)\"/)?.[1] || '0');
        const skipped = parseInt(attributes.match(/skipped=\"(\d+)\"/)?.[1] || '0');
        const time = parseFloat(attributes.match(/time=\"([^\"]+)\"/)?.[1] || '0');
        return { tests, failures, errors, skipped, time };
    }

    // Fallback: first <testsuite ...> element
    const testsuiteMatch = xmlString.match(/<testsuite[^>]*>/);
    if (!testsuiteMatch) {
        return { tests: 0, failures: 0, errors: 0, skipped: 0, time: 0 };
    }

    const attributes = testsuiteMatch[0];
    const tests = parseInt(attributes.match(/tests=\"(\d+)\"/)?.[1] || '0');
    const failures = parseInt(attributes.match(/failures=\"(\d+)\"/)?.[1] || '0');
    const errors = parseInt(attributes.match(/errors=\"(\d+)\"/)?.[1] || '0');
    const skipped = parseInt(attributes.match(/skipped=\"(\d+)\"/)?.[1] || '0');
    const time = parseFloat(attributes.match(/time=\"([^\"]+)\"/)?.[1] || '0');

    return { tests, failures, errors, skipped, time };
}

function parseFailureDetails(xmlString) {
    // Parse failures from JUnit produced by Jest/Bun. Handles both
    // self-closing <failure .../> and paired <failure ...>...</failure>.
    const failures = [];

    // Iterate over each testcase and inspect its body for a failure node
    const testcaseRegex = /<testcase\b([^>]*)>([\s\S]*?)<\/testcase>/g;
    let m;
    while ((m = testcaseRegex.exec(xmlString)) !== null) {
        const attrs = m[1] || '';
        const body = m[2] || '';

        const nameMatch = attrs.match(/\bname=\"([^\"]*)\"/);
        const testName = nameMatch ? nameMatch[1] : 'Unknown test';

        // Try self-closing first, then paired form
        const failureSelf = body.match(/<failure\b([^>]*)\/>/);
        const failurePair = body.match(/<failure\b([^>]*)>([\s\S]*?)<\/failure>/);
        if (!failureSelf && !failurePair) continue;

        const failureAttrs = (failureSelf ? failureSelf[1] : failurePair[1]) || '';
        let message = '';

        // Prefer explicit message attribute
        const msgMatch = failureAttrs.match(/\bmessage=\"([^\"]*)\"/);
        if (msgMatch) message = msgMatch[1];

        // Fallbacks: type attribute, then inner text (for paired form)
        if (!message) {
            const typeMatch = failureAttrs.match(/\btype=\"([^\"]*)\"/);
            if (typeMatch) message = typeMatch[1];
        }

        if (!message && failurePair && failurePair[2]) {
            // Strip CDATA and any XML tags from inner content
            const inner = failurePair[2]
                .replace(/<!\[CDATA\[([\s\S]*?)\]\]>/g, '$1')
                .replace(/<[^>]+>/g, '')
                .trim();
            if (inner) message = inner;
        }

        if (!message) message = 'AssertionError';

        const truncated = message.length > 200 ? message.substring(0, 200) + '...' : message;
        failures.push({ testName, message: truncated });
    }

    return failures;
}

module.exports = {
    countFail,
    parseJUnitXML,
    parseFailureDetails,
};


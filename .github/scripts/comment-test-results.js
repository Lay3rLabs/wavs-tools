#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { parseJUnitXML, parseFailureDetails } = require('./utils');

async function commentTestResults({ github, context }) {
  const prNumber = context.eventName === 'pull_request' ? context.payload.pull_request.number : context.issue.number;
  const workflowLogsLink = `https://github.com/${context.repo.owner}/${context.repo.repo}/actions/runs/${context.runId}`;

  try {
    // The report path is relative to the tests directory
    const reportPath = path.join('tests', '.test-reports', 'junit.xml');

    if (!fs.existsSync(reportPath)) {
      console.log(`Report file not found at: ${reportPath}`);
      throw new Error(`Test report file not found at ${reportPath}`);
    }

    const xmlContent = fs.readFileSync(reportPath, 'utf8');
    const report = parseJUnitXML(xmlContent);

    const passing = report.tests - report.failures - report.errors - report.skipped;
    const failing = report.failures + report.errors;
    const skipped = report.skipped || 0;
    const total = report.tests;
    const duration = Math.round(report.time * 1000); // Convert to ms

    let comment = `## 🧪 Test Results`;
    comment += ` [📊 View run](${workflowLogsLink})`;
    comment += `\n\n**Summary:** ${passing}/${total} tests passing`;

    if (failing > 0) {
      comment += ` (${failing} failed)`;
    }

    if (skipped > 0) {
      comment += ` (${skipped} skipped)`;
    }

    comment += `\n**Duration:** ${duration}ms\n\n`;

    if (failing > 0) {
      comment += `### ❌ Failed Tests\n`;

      // Parse specific failure details
      const failureDetails = parseFailureDetails(xmlContent);
      if (failureDetails.length > 0) {
        comment += `**${failing} test(s) failed:**\n\n`;
        failureDetails.forEach(failure => {
          comment += `- **${failure.testName}**\n`;
          comment += `  Error: \`${failure.message}\`\n\n`;
        });
      } else {
        comment += `${failing} test(s) failed. Check the [workflow logs](${workflowLogsLink}) for details.\n\n`;
      }
    } else {
      comment += `### ✅ All tests passed!\n\n`;
    }


    await github.rest.issues.createComment({
      issue_number: prNumber,
      owner: context.repo.owner,
      repo: context.repo.repo,
      body: comment
    });

    console.log('Successfully posted test results comment');

  } catch (error) {
    console.error('Error processing test report:', error);

    // Post a fallback comment — don't let this mask the original error
    try {
      await github.rest.issues.createComment({
        issue_number: prNumber,
        owner: context.repo.owner,
        repo: context.repo.repo,
        body: `## 🧪 Test Results\n\n❌ Failed to generate test report. Check the [workflow logs](${workflowLogsLink}) for details.`
      });
    } catch (commentError) {
      console.error('Failed to post fallback comment:', commentError);
    }

    // Re-throw the original error so the workflow shows as failed
    throw error;
  }
}

module.exports = { commentTestResults };
#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const {parseJUnitXML} = require('./utils');

async function commentTestResults({ github, context }) {
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
    const pending = 0;
    const skipped = report.skipped || 0;
    const total = report.tests;
    const duration = Math.round(report.time * 1000); // Convert to ms
    
    const prNumber = context.eventName === 'pull_request' ? context.payload.pull_request.number : context.issue.number;
    
    let comment = `## 🧪 Test Results`;
    comment += ` [📊 View run](https://github.com/${context.repo.owner}/${context.repo.repo}/actions/runs/${context.runId})`;
    comment += `\n\n**Summary:** ${passing}/${total} tests passing`;
    
    if (failing > 0) {
      comment += ` (${failing} failed)`;
    }

    if (pending > 0 || skipped > 0) {
      comment += ` (${pending + skipped} skipped/pending)`;
    }
    
    comment += `\n**Duration:** ${duration}ms\n\n`;
    
    if (failing > 0) {
      comment += `### ❌ Failed Tests\n`;
      comment += `${failing} test(s) failed. Check the [workflow logs](https://github.com/${context.repo.owner}/${context.repo.repo}/actions/runs/${context.runId}) for details.\n\n`;
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
    
    const prNumber = context.eventName === 'pull_request' ? context.payload.pull_request.number : context.issue.number;
    
    // Post a fallback comment
    await github.rest.issues.createComment({
      issue_number: prNumber,
      owner: context.repo.owner,
      repo: context.repo.repo,
      body: '## 🧪 Test Results\n\n❌ Failed to generate test report. Check the workflow logs for details.'
    });
    
    // Re-throw the error so the workflow shows as failed
    throw error;
  }
}

module.exports = { commentTestResults };
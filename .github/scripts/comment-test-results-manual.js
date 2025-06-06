#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

async function commentTestResults({ github, context }) {
  const eventName = context.eventName || github.event_name;
  try {
    // The report path is relative to the tests directory
    const reportPath = path.join('tests', '.test-reports', 'merged-report.json');
    
    if (!fs.existsSync(reportPath)) {
      console.log(`Report file not found at: ${reportPath}`);
      throw new Error(`Test report file not found at ${reportPath}`);
    }
    
    const reportContent = fs.readFileSync(reportPath, 'utf8');
    const report = JSON.parse(reportContent);
    
    const stats = report.stats;
    const passing = stats.passes;
    const failing = stats.failures;
    const total = stats.tests;
    const duration = stats.duration;
    
    let comment = `## 🧪 Test Results\n\n`;
    comment += `**Summary:** ${passing}/${total} tests passing`;
    
    if (failing > 0) {
      comment += ` (${failing} failed)`;
    }
    
    comment += `\n**Duration:** ${duration}ms\n\n`;
    
    if (failing > 0) {
      comment += `### ❌ Failed Tests\n`;
      
      // The mochawesome report structure has failures in results array
      const failures = [];
      
      function extractFailures(suites) {
        suites.forEach(suite => {
          if (suite.tests) {
            suite.tests.forEach(test => {
              if (test.state === 'failed') {
                failures.push(test);
              }
            });
          }
          if (suite.suites) {
            extractFailures(suite.suites);
          }
        });
      }
      
      if (report.results) {
        extractFailures(report.results);
      }
      
      if (failures.length > 0) {
        failures.forEach(failure => {
          comment += `- **${failure.fullTitle}**\n`;
          if (failure.err && failure.err.message) {
            comment += `  \`${failure.err.message}\`\n\n`;
          }
        });
      } else {
        comment += `No detailed failure information available.\n\n`;
      }
    } else {
      comment += `### ✅ All tests passed!\n`;
    }
    
    comment += `\n[📊 View detailed report](https://github.com/${context.repo.owner}/${context.repo.repo}/actions/runs/${context.runId})`;
    
    // Handle different event types - pull_request vs issue_comment
    const issueNumber = context.issue?.number || github.event.issue?.number;
    
    await github.rest.issues.createComment({
      issue_number: issueNumber,
      owner: context.repo.owner,
      repo: context.repo.repo,
      body: comment
    });
    
    console.log('Successfully posted test results comment');
    
  } catch (error) {
    console.error('Error processing test report:', error);
    
    // Post a fallback comment
    const issueNumber = context.issue?.number || github.event.issue?.number;
    
    await github.rest.issues.createComment({
      issue_number: issueNumber,
      owner: context.repo.owner,
      repo: context.repo.repo,
      body: '## 🧪 Test Results\n\n❌ Failed to generate test report. Check the workflow logs for details.'
    });
    
    // Re-throw the error so the workflow shows as failed
    throw error;
  }
}

module.exports = { commentTestResults };

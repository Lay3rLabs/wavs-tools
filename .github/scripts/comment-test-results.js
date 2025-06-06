#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

async function commentTestResults({ github, context }) {
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
    const pending = stats.pending || 0;
    const skipped = stats.skipped || 0;
    const total = stats.tests;
    const duration = stats.duration;
    
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
      
      // The mochawesome report structure has failures in results array
      const failures = extractFailures(report);
      
      
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
    

    comment += `\n<details>\n\n${extractDetails(report)}\n</details>\n`;

    
    await github.rest.issues.createComment({
      issue_number: context.issue.number,
      owner: context.repo.owner,
      repo: context.repo.repo,
      body: comment
    });
    
    console.log('Successfully posted test results comment');
    
  } catch (error) {
    console.error('Error processing test report:', error);
    
    // Post a fallback comment
    await github.rest.issues.createComment({
      issue_number: context.issue.number,
      owner: context.repo.owner,
      repo: context.repo.repo,
      body: '## 🧪 Test Results\n\n❌ Failed to generate test report. Check the workflow logs for details.'
    });
    
    // Re-throw the error so the workflow shows as failed
    throw error;
  }
}

function recurseSuites(suites, callback) {
  suites.forEach(suite => {
    callback(suite);
    if (suite.suites) {
      recurseSuites(suite.suites, callback);
    }
  });
}

function extractFailures(report) {
  const failures = [];

  if (report.results) {
    recurseSuites(report.results, suite => {
      if (suite.tests) {
        suite.tests.forEach(test => {
          if (test.state === 'failed') {
            failures.push(test);
          }
        });
      }
    });
  }

  return failures;
}

function extractDetails(report) {
  let details = '';


  if (report.results) {
    recurseSuites(report.results, suite => {
      if (suite.tests) {
        suite.tests.forEach(test => {
          details += `### ${renderIcon(test.state)} ${test.fullTitle}\n`;
          details += `- **State:** ${test.state}\n`;
          if (test.err && test.err.message) {
            details += `- **Error:** \`${test.err.message}\`\n`;
          }
          if (test.duration) {
            details += `- **Duration:** ${test.duration}ms\n`;
          }
          details += '\n';
        });
      }
    });
  } else {
    details = 'No detailed test results available.';
  }

  return details;
}

function renderIcon(state) {
  switch (state) {
    case 'passed':
      return '✅';
    case 'failed':
      return '❌';
    case 'skipped':
    case 'pending':
      return '⏭️';
    default:
      return '❓';
  }
}

module.exports = { commentTestResults };

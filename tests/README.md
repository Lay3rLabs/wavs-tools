# WAVS Tools Tests

This directory contains the test suite for WAVS Tools.

## Structure

- `src/` - Test source files (TypeScript)
- `.test-reports/` - Generated test reports (HTML and JSON)
- `package.json` - Test dependencies and scripts
- `.mocharc.json` - Mocha configuration

## Running Tests

```bash
# Run tests once
npm test

# Run tests with file watching
npm run test:watch

# Generate test reports (HTML + JSON)
npm run test:report

# Clean test reports
npm run test:clean
```

## Test Reports

Tests generate reports in both HTML and JSON formats:

- **HTML Report**: `.test-reports/merged-report.html` - Human-readable test results
- **JSON Report**: `.test-reports/merged-report.json` - Machine-readable results used by CI/CD

## CI/CD Integration

The GitHub Actions workflow automatically:

1. Runs tests on every PR and push to main
2. Generates test reports
3. Posts test results as PR comments
4. Uploads test artifacts for review

The test result comments include:
- Pass/fail summary
- Test duration
- Detailed failure information (if any)
- Link to full workflow logs

## Adding Tests

Create new test files in `src/` with the pattern `*.test.ts`. Tests use:

- **Framework**: Mocha
- **Assertions**: Chai
- **TypeScript**: Supported via tsx loader

Example test:

```typescript
import { assert } from "chai";

describe("My Feature", () => {
  it("should work correctly", () => {
    assert.equal(1 + 1, 2);
  });
});
```

/**
 * Test timeout constants
 * All values are in milliseconds
 */
export const TIMEOUTS = {
  SETUP: 15 * 60 * 1000,      // 15 minutes - backend start, middleware deploy, etc.
  TEARDOWN: 30 * 1000,        // 30 seconds - cleanup operations
  TEST: 60 * 1000,            // 60 seconds - individual test execution
} as const;
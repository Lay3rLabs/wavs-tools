import { describe, beforeAll, afterAll, test } from "bun:test";
import { BackendManager } from "./helpers/backend";
import { projectPath, execAsync } from "./helpers/utils";
import { TEST_TIMEOUT } from "./helpers/constants";

const PROJECT_NAME = "wavs-drand";

describe(PROJECT_NAME.toUpperCase(), function () {
  let backendManager: BackendManager;

  beforeAll(async () => {
    backendManager = new BackendManager({ nChains: 1, nOperators: 2 });
    await backendManager.start();
    backendManager.assertRunning();

    await execAsync("task", ["bootstrap"], {
      cwd: projectPath(PROJECT_NAME),
    });
  });

  afterAll(async () => {
    await backendManager.stop();
  });

  describe("All tests", function () {
    test(PROJECT_NAME + " tests", async () => {
      backendManager.assertRunning();

      await execAsync("task", ["run-tests"], {
        cwd: projectPath(PROJECT_NAME),
      });
    }, TEST_TIMEOUT);
  });
});

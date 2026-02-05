import { describe, beforeAll, afterAll, test } from "bun:test";
import { BackendManager } from "./helpers/backend";
import { projectPath, execAsync } from "./helpers/utils";

const PROJECT_NAME = "multi-chain-operator-sync";
const MIDDLEWARE_MODE = "POA";

describe(`${PROJECT_NAME.toUpperCase()} (POA)`, function () {
  let backendManager: BackendManager;

  beforeAll(async () => {
    backendManager = new BackendManager({ nChains: 2, nOperators: 2 });
    await backendManager.start();
    backendManager.assertRunning();

    await execAsync("task", ["bootstrap"], {
      cwd: projectPath(PROJECT_NAME),
      env: { ...process.env, MIDDLEWARE_MODE },
    });
  });

  afterAll(async () => {
    await backendManager.stop();
  });

  describe("All tests", function () {
    test(PROJECT_NAME + " run-tests (POA)", async () => {
      backendManager.assertRunning();

      await execAsync("task", ["run-tests-poa"], {
        cwd: projectPath(PROJECT_NAME),
        env: { ...process.env, MIDDLEWARE_MODE },
      });
    });
  });
});

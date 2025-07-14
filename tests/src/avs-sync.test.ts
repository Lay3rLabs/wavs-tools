import { BackendManager } from "./helpers/backend";
import { projectPath, execAsync } from "./helpers/utils";
import { TIMEOUTS } from "./helpers/constants";

const PROJECT_NAME = "avs-sync";

describe(PROJECT_NAME.toUpperCase(), function () {
  let backendManager: BackendManager;

  before(async function () {
    this.timeout(TIMEOUTS.SETUP);

    backendManager = new BackendManager({ nChains: 1, nOperators: 1 });
    await backendManager.start();
    backendManager.assertRunning();

    await execAsync("task", ["bootstrap"], {
      cwd: projectPath(PROJECT_NAME),
    });
  });

  after(async function () {
    this.timeout(TIMEOUTS.TEARDOWN);
    await backendManager.stop();
  });

  describe("All tests", function () {
    it("should complete without error", async function () {
      this.timeout(TIMEOUTS.TEST);
      backendManager.assertRunning();

      await execAsync("task", ["run-tests"], {
        cwd: projectPath(PROJECT_NAME),
      });
    });
  });
});

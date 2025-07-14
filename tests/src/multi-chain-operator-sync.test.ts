import { BackendManager } from "./helpers/backend";
import { projectPath, execAsync } from "./helpers/utils";
import { TIMEOUTS } from "./helpers/constants";

describe("MULTI-CHAIN-OPERATOR-SYNC", function () {
  let backendManager: BackendManager;

  before(async function () {
    this.timeout(TIMEOUTS.SETUP);

    backendManager = new BackendManager({ nChains: 2, nOperators: 2 });
    await backendManager.start();
    backendManager.assertRunning();

    await execAsync("task", ["bootstrap"], {
      cwd: projectPath("multi-chain-operator-sync"),
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
        cwd: projectPath("multi-chain-operator-sync"),
      });
    });
  });
});

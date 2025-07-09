import { BackendManager } from "./helpers/backend";
import { projectPath, execAsync } from "./helpers/utils";

describe("MULTI-CHAIN-SERVICE-MANAGER-SYNC", function () {
  let backendManager: BackendManager;

  before(async function () {
    // 15 minute timeout for setup
    // since it includes starting the backend, deploying middleware, registering operators, etc.
    this.timeout(900000); 

    backendManager = new BackendManager({nChains: 2, nOperators: 1});
    await backendManager.start();

    await execAsync("task", ["bootstrap"], {
      cwd: projectPath("multi-chain-service-manager-sync"),
    });
  });

  after(async function () {
    this.timeout(30000); // 30 second timeout for cleanup
    await backendManager.stop();
  });

  describe("All tests", function () {
    it("should complete without error", async function () {
      this.timeout(30000);
      backendManager.assertRunning();

      await execAsync("task", ["run-tests"], {
        cwd: projectPath("multi-chain-service-manager-sync"),
      });
    });
  });
});

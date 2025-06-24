import { BackendManager } from "./helpers/backend";
import { projectPath, execAsync } from "./helpers/utils";

describe("AVS-SYNC", function () {
  let backendManager: BackendManager;

  before(async function () {
    // 5 minute timeout for setup
    // since it includes starting the backend, deploying middleware, registering operators, etc.
    this.timeout(300000); 

    // just temporarily starting 2 chains for testing purposes
    backendManager = new BackendManager({nChains: 2});
    await backendManager.start();

    await execAsync("task", ["bootstrap"], {
      cwd: projectPath("avs-sync"),
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
        cwd: projectPath("avs-sync"),
      });
    });
  });
});

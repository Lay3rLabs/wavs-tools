import { BackendManager } from "./helpers/backend";
import { projectDirectory, execAsync } from "./helpers/utils";

describe("AVS-SYNC", function () {
  let backendManager: BackendManager;

  before(async function () {
    this.timeout(60000); // 1 minute timeout for setup

    backendManager = new BackendManager();
    await backendManager.start();
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
        cwd: projectDirectory("avs-sync"),
      });
    });
  });
});


import { assert } from "chai";
import {BackendManager} from "./helpers/backend";

describe('Basic tests', function() {
  let backendManager:BackendManager;

  before(async function() {
    this.timeout(60000); // 1 minute timeout for setup
    
    backendManager = new BackendManager();
    await backendManager.start();
  });

  after(async function() {
    this.timeout(30000); // 30 second timeout for cleanup
    
    if (backendManager) {
      await backendManager.stop();
    }
  });

  describe('Transaction Processing', function() {
    it('should successfully send transactions', async function() {
      this.timeout(30000);
      // TODO
    });
  });

  describe('Backend Health', function() {
    it('should confirm backend is running', async function() {
      assert(backendManager.isRunning(), 'Backend should be running');
    });
  });
});
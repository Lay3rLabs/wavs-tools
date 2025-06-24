import { execAsync, rootDirectory } from './utils';

export interface BackendManagerConfig {
  nChains: number
}

const defaultBackendManagerConfig:BackendManagerConfig = {
  nChains: 1,
};

export class BackendManager {
  public error: any | undefined;
  public config: BackendManagerConfig;
  isRunning: boolean = false;


  constructor(config?: BackendManagerConfig) {
    this.config = config || defaultBackendManagerConfig;
  }


  // Due to https://github.com/mochajs/mocha/issues/4392
  // this always "succeeds", all interactions should require calling `assertRunning()` 
  // specifically see https://github.com/mochajs/mocha/issues/4392#issuecomment-797500518
  async start() {
    try {
      this.error = undefined;

      // Start the backend using task
      console.log('Starting backend...');
      const args = ['backend:start'];
      if (this.config.nChains > 1) {
        args.push(`CHAIN_COUNT=${this.config.nChains}`);
      }

      await execAsync('task', args, {
        cwd: rootDirectory(),
      });

      console.log('Backend started successfully');

      this.isRunning = true;
    } catch (error) {
      this.stop();
      this.error = error;
    }
  }

  async stop() {
    if(this.isRunning) {
      this.isRunning = false;
      // change to root directory
      console.log('Stopping backend...');
      await execAsync('task', ['backend:stop'], {
        cwd: rootDirectory(),
      });
      console.log('Backend stopped successfully');
    }
  }

  assertRunning() {
    if(this.error) {
      throw this.error;
    }
    if (!this.isRunning) {
      throw new Error('Backend process is not running');
    }
  }
}

export default BackendManager;
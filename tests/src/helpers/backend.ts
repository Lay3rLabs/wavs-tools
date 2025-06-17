import { exec, ExecException } from 'child_process';
import fs from 'fs';
import path from 'path';
import { promisify } from 'util';
import waitPort from 'wait-port';

const execPromise = promisify(exec);

export class BackendManager {
  public error: any | undefined;
  isRunning: boolean = false;

  constructor() {
  }

  rootDirectory(): string {
    // find the repo root directory by looking for the presence of .git
    let currentDir = process.cwd();
    while (currentDir !== '/') {
      if (fs.existsSync(`${currentDir}/.git`)) {
        return currentDir;
      }
      currentDir = path.dirname(currentDir);
    }

    throw new Error('Could not find the root directory of the repository');
  }

  // Due to https://github.com/mochajs/mocha/issues/4392
  // this always "succeeds", all interactions should require calling `assertRunning()` 
  // specifically see https://github.com/mochajs/mocha/issues/4392#issuecomment-797500518
  async start() {
    try {
      this.error = undefined;
      // change to root directory
      process.chdir(this.rootDirectory());

      // Start the backend using task
      console.log('Starting backend...');
      const {stdout, stderr} = await execPromise('task backend:start');
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
      process.chdir(this.rootDirectory());
      console.log('Stopping backend...');
      const {stdout, stderr} = await execPromise('task backend:stop');
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
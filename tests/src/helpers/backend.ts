import { spawn, ChildProcess } from 'child_process';
import fs from 'fs';
import path from 'path';
import waitPort from 'wait-port';

export class BackendManager {
  private backendProcess: ChildProcess | null;
  private port: number;
  public error: any | undefined;

  constructor() {
    this.backendProcess = null;
    this.port = parseInt(process.env.BACKEND_PORT || '3000', 10);
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
  async start(): Promise<void> {
    (async () => {
      this.error = undefined;
      // change to root directory
      process.chdir(this.rootDirectory());
    
      // Start the backend using task
      this.backendProcess = spawn('task', ['start-backend'], {
        stdio: 'pipe',
        detached: true
      });

      // Handle process output for debugging
      this.backendProcess.stdout?.on('data', (data: Buffer) => {
        //console.log(`Backend stdout: ${data}`);
      });

      this.backendProcess.stderr?.on('data', (data: Buffer) => {
        //console.error(`Backend stderr: ${data}`);
      });

      this.backendProcess.on('error', (error: Error) => {
        console.error(`Failed to start backend: ${error}`);
        throw error;
      });

      // Wait for the backend to be ready
      await waitPort({
        host: 'localhost',
        port: this.port,
        timeout: 30000, // 30 second timeout
        output: 'silent'
      });
    })().catch((error) => {
      this.stop();
      this.error = error;
    })
  }

  async stop(): Promise<void> {
    if (this.backendProcess && this.backendProcess.pid) {
      return new Promise<void>((resolve) => {
        // Since we spawn with detached: true, we need to kill the process group
        // Use negative PID to kill the entire process group on Unix systems
        const pid = -this.backendProcess!.pid!;
        
        try {
          // First try graceful shutdown
          process.kill(pid, 'SIGTERM');
          
          // Set a timeout for force kill if graceful shutdown fails
          const forceKillTimeout = setTimeout(() => {
            try {
              process.kill(pid, 'SIGKILL');
            } catch (error) {
              console.warn(`Warning during force kill: ${error}`);
            }
          }, 5000); // 5 second timeout
          
          // Listen for process exit
          this.backendProcess!.on('exit', () => {
            clearTimeout(forceKillTimeout);
            this.backendProcess = null;
            resolve();
          });
          
        } catch (error) {
          console.warn(`Warning: ${error}`);
          this.backendProcess = null;
          resolve();
        }
      });
    }
  }

  assertRunning() {
    if(this.error) {
      throw this.error;
    }
    if (!this.backendProcess || this.backendProcess.killed) {
      throw new Error('Backend process is not running');
    }
  }
}

export default BackendManager;
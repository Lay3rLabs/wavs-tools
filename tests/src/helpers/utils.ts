import fs from 'fs';
import path from 'path';
import { spawn, ChildProcess } from 'child_process';

export function rootDirectory(): string {
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

export function projectDirectory(project:string):string {
    return path.join(rootDirectory(), 'projects', project);
}

export interface ExecAsyncOptions {
  cwd?: string;
  env?: NodeJS.ProcessEnv;
  shell?: boolean;
  captureOutput?: boolean;
  timeoutMs?: number;
}

export interface ExecAsyncCaptureResult {
  stdout: string;
  stderr: string;
}

export type ExecAsyncResult = ExecAsyncCaptureResult | null;

export async function execAsync(
  command: string,
  args: string[] = [],
  options: ExecAsyncOptions = {}
): Promise<ExecAsyncResult> {
  const { cwd, env, shell = false, captureOutput = false, timeoutMs } = options;

  return new Promise((resolve, reject) => {
    const child: ChildProcess = spawn(command, args, {
      cwd,
      env,
      shell,
      stdio: captureOutput ? ['ignore', 'pipe', 'pipe'] : 'inherit',
    });

    let stdout = '';
    let stderr = '';
    let timeout: NodeJS.Timeout | undefined;

    if (captureOutput && child.stdout) {
      child.stdout.on('data', (data: Buffer) => {
        stdout += data.toString();
      });
    }

    if (captureOutput && child.stderr) {
      child.stderr.on('data', (data: Buffer) => {
        stderr += data.toString();
      });
    }

    const cleanup = () => {
      if (timeout) clearTimeout(timeout);
    };

    if (timeoutMs) {
      timeout = setTimeout(() => {
        child.kill('SIGTERM');
        reject(new Error(`Command "${command} ${args.join(' ')}" timed out after ${timeoutMs}ms`));
      }, timeoutMs);
    }

    child.on('error', (err) => {
      cleanup();
      reject(err);
    });

    child.on('close', (code) => {
      cleanup();
      if (code !== 0) {
        const errorMessage = captureOutput
          ? `Command "${command} ${args.join(' ')}" failed with code ${code}\n\nSTDERR:\n${stderr}`
          : `Command "${command} ${args.join(' ')}" failed with code ${code}`;
        reject(new Error(errorMessage));
      } else {
        resolve(captureOutput ? { stdout, stderr } : null);
      }
    });
  });
}
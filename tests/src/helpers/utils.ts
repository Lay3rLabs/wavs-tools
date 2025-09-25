import fs from 'fs';
import path from 'path';
import { spawn, type Subprocess } from 'bun';

export function rootPath(): string {
  // find the repo root directory by looking for the presence of .git
  let currentDir = process.cwd();
  const root = path.parse(currentDir).root;
  for (; ;) {
    if (fs.existsSync(path.join(currentDir, '.git'))) {
      return path.resolve(currentDir);
    }
    if (currentDir === root) break;
    currentDir = path.dirname(currentDir);
  }

  throw new Error('Could not find the root directory of the repository');
}

export function projectPath(project: string): string {
  const projectPath = path.resolve(path.join(rootPath(), 'projects', project));

  if (!fs.existsSync(projectPath)) {
    throw new Error(`Path does not exist: ${projectPath}`);
  }

  return projectPath;
}

export interface ExecAsyncOptions {
  cwd?: string;
  env?: NodeJS.ProcessEnv;
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
  const { cwd, env, captureOutput = false, timeoutMs } = options;

  return new Promise((resolve, reject) => {
    console.log(`Executing command: ${command} ${args.join(' ')} in ${cwd || process.cwd()}`);

    const child: Subprocess = spawn([command, ...args], {
      cwd,
      env,
      stdin: 'inherit',
      stdout: captureOutput ? 'pipe' : 'inherit',
      stderr: captureOutput ? 'pipe' : 'inherit',
    });

    let timeout: NodeJS.Timeout | undefined;
    if (timeoutMs) {
      timeout = setTimeout(() => {
        try {
          child.kill('SIGTERM');
        } catch { }
        reject(new Error(`Command "${command} ${args.join(' ')}" timed out after ${timeoutMs}ms`));
      }, timeoutMs);
    }

    const cleanup = () => {
      if (timeout) clearTimeout(timeout);
    };

    const stdoutPromise: Promise<string> = captureOutput && child.stdout && typeof child.stdout !== 'number'
      ? new Response(child.stdout).text()
      : Promise.resolve('');
    const stderrPromise: Promise<string> = captureOutput && child.stderr && typeof child.stderr !== 'number'
      ? new Response(child.stderr).text()
      : Promise.resolve('');

    Promise.all([child.exited, stdoutPromise, stderrPromise])
      .then(([exitCode, stdout, stderr]) => {
        cleanup();
        if (exitCode !== 0) {
          const errorMessage = captureOutput
            ? `Command "${command} ${args.join(' ')}" failed with code ${exitCode}\n\nSTDERR:\n${stderr}`
            : `Command "${command} ${args.join(' ')}" failed with code ${exitCode}`;
          reject(new Error(errorMessage));
        } else {
          resolve(captureOutput ? { stdout, stderr } : null);
        }
      })
      .catch((err: any) => {
        cleanup();
        reject(err);
      });
  });
}


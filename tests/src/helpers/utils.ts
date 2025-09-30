import fs from "fs";
import path from "path";
import { spawn, type Subprocess } from "bun";

export function rootPath(): string {
  // find the repo root directory by looking for the presence of .git
  let currentDir = process.cwd();
  const root = path.parse(currentDir).root;
  for (; ;) {
    if (fs.existsSync(path.join(currentDir, ".git"))) {
      return path.resolve(currentDir);
    }
    if (currentDir === root) break;
    currentDir = path.dirname(currentDir);
  }

  throw new Error("Could not find the root directory of the repository");
}

export function projectPath(project: string): string {
  const projectPath = path.resolve(path.join(rootPath(), "projects", project));

  if (!fs.existsSync(projectPath)) {
    throw new Error(`Path does not exist: ${projectPath}`);
  }

  return projectPath;
}

export interface ExecAsyncOptions {
  cwd?: string;
  env?: Record<string, string | undefined>; // accept ProcessEnv-like
  captureOutput?: boolean;
  timeoutMs?: number;
  killSignal?: "SIGTERM" | "SIGKILL";
}

export interface ExecAsyncCaptureResult {
  stdout: string;
  stderr: string;
}
export type ExecAsyncResult = ExecAsyncCaptureResult | null;

function toBunEnv(env?: Record<string, string | undefined>): Record<string, string> | undefined {
  if (!env) return undefined;
  const clean: Record<string, string> = {};
  for (const [k, v] of Object.entries(env)) {
    if (typeof v === "string") clean[k] = v;
  }
  return clean;
}

async function streamToText(
  stream: ReadableStream | null | number | "inherit" | undefined
): Promise<string> {
  if (!stream || typeof stream === "number" || stream === "inherit") return "";
  // Works for Web ReadableStreams (Bun)
  return new Response(stream as ReadableStream).text();
}

export async function execAsync(
  command: string,
  args: string[] = [],
  options: ExecAsyncOptions = {}
): Promise<ExecAsyncResult> {
  const { cwd, env, captureOutput = false, timeoutMs, killSignal = "SIGKILL" } = options;

  return new Promise<ExecAsyncResult>((resolve, reject) => {
    console.log(`Executing command: ${command} ${args.join(" ")} in ${cwd || process.cwd()}`);

    const child: Subprocess = spawn([command, ...args], {
      cwd,
      env: toBunEnv(env),
      stdin: "inherit",
      stdout: captureOutput ? "pipe" : "inherit",
      stderr: captureOutput ? "pipe" : "inherit",
    });

    let timeout: NodeJS.Timeout | undefined;
    if (timeoutMs && Number.isFinite(timeoutMs)) {
      timeout = setTimeout(() => {
        try {
          child.kill(killSignal);
        } catch { }
        reject(new Error(`Command "${command} ${args.join(" ")}" timed out after ${timeoutMs}ms`));
      }, timeoutMs);
    }

    const done = async () => {
      try {
        const [exitCode, stdout, stderr] = await Promise.all([
          child.exited,
          captureOutput ? streamToText(child.stdout as any) : Promise.resolve(""),
          captureOutput ? streamToText(child.stderr as any) : Promise.resolve(""),
        ]);
        if (timeout) clearTimeout(timeout);

        if (exitCode !== 0) {
          const msg = captureOutput
            ? `Command "${command} ${args.join(" ")}" failed with code ${exitCode}\n\nSTDERR:\n${stderr}\n\nSTDOUT:\n${stdout}`
            : `Command "${command} ${args.join(" ")}" failed with code ${exitCode}`;
          reject(new Error(msg));
        } else {
          resolve(captureOutput ? { stdout, stderr } : null);
        }
      } catch (err) {
        if (timeout) clearTimeout(timeout);
        reject(err);
      }
    };

    // Ensure we hook completion
    void done();
  });
}

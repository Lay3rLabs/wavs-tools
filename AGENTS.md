# Taskfile guide for projects

This repo uses a Taskfile-per-project plus shared taskfiles under `taskfile/`. Use this as a quick map for how tasks are wired together and how to add new ones.

## Directory layout
- Root `Taskfile.yml`: convenience entrypoint; wires in common stacks (`backend`, `cli`, `middleware`, `component`, `publish`, `visualizer`) and exposes repo-wide helpers such as `setup`, `test`, `lint`, and a `clean` that iterates through every project under `projects/`.
- Global taskfiles: stored in `/taskfile`. Each file groups tasks by domain (e.g., `backend.yml`, `cli.yml`, `component.yml`, `docker.yml`, `publish.yml`, `visualizer.yml`) plus shared helpers in `taskfile/common/`.
- Project taskfiles: every project in `projects/<name>/` has its own `Taskfile.yml`. It imports the global taskfiles and any project-local taskfiles from `projects/<name>/taskfile/` (e.g., `bootstrap.yml`, `utils.yml`, `test.yml`).

## How a project Taskfile is assembled
- Each project Taskfile sets `dotenv: ["../../.env"]` so environment secrets live at the repo root.
- Global includes are pulled in with relative paths (`../../taskfile/*.yml`). Some includes use `flatten: true` (notably `config.yml`, `docker.yml`, and shared utils) so their tasks appear in the local namespace; others keep their own namespace (e.g., `cli:`, `middleware:`).
- Local includes (under `projects/<name>/taskfile/`) hold project-specific workflows such as bootstrap, utils, and tests. Shared helpers from `taskfile/common` can also be pulled in and flattened.
- Variables that depend on project paths are usually redefined locally (e.g., `PROJECT`, `COMPONENT_NAME`, `PROJECT_OUTPUT_DIR`, `SERVICE_JSON_PATH`) to avoid relying on include ordering.
- Top-level project tasks such as `bootstrap`, `clean`, and `run-tests` orchestrate the included tasks. They typically:
  - call bootstrap subtasks like `bootstrap:middleware-deploy`, `bootstrap:components-build`, `bootstrap:build-service`, etc.
  - clean local artifacts (`contracts/broadcast`, `contracts/out`, `.project-output`) and run `cargo clean -p <project>` from the repo root.
  - delegate tests to namespaced tasks (e.g., `task test:simple`).

## Running tasks
- From the repo root, target a project by pointing Task at its directory: `task -d projects/wavs-drand bootstrap`.
- From inside a project directory, run tasks directly: `task bootstrap`, `task clean`, or namespaced tasks like `task bootstrap:components-build`.
- Root-level helpers remain available: `task setup` installs deps and builds ABI files; `task clean` loops over all projects via `clean-project`.

## Adding or updating a project
- Start from an existing project Taskfile, keep the same include pattern, and update `COMPONENT_NAME` plus any project-specific vars.
- Put custom workflows in `projects/<name>/taskfile/*.yml`; use `flatten: true` only when you want those tasks merged into the main namespace.
- Ensure outputs land in `.project-output` using the provided vars so root cleaners and consumers can find them.

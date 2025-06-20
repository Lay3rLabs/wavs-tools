This repo contains various tools and services that make up the WAVS ecosystem.

# PROJECTS

## [avs-sync](projects/avs-sync) 

A tool for syncing AVS operators (TODO: explain more).

# SYSTEM REQUIREMENTS

<details>
<summary>Tooling</summary>
&nbsp;

* [Docker](https://docs.docker.com/get-started/get-docker/)
* [Taskfile](https://taskfile.dev/installation/)
* [JQ](https://jqlang.org/download/)
* [Node.js](https://nodejs.org/en/download)
* [Rust](https://www.rust-lang.org/tools/install)
* [Cargo Components](https://github.com/bytecodealliance/cargo-component#installation)
* [wkg](https://crates.io/crates/wkg)
* [Foundry](https://getfoundry.sh/introduction/installation#using-foundryup)

</details>

<details>
<summary>System configuration</summary>

### Setup default wkg registry 

```bash docci-ignore
wkg config --default-registry wa.dev
```

</details>

# QUICKSTART

## First-time setup

Copy and edit your environment variables

```bash
cp .env.example .env
```

Install dependencies, compile core contracts, etc.
This may take a while on first run.
```bash
task setup
```

## Get up and running

### Start the backend
```bash
task backend:start
```

You can also start the backend with multiple chains, where the first will fork holesky and the rest will be local-only:

```bash
task backend:start CHAIN_COUNT=3 
```

### Stop the backend

```bash
task backend:stop
```

## Develop a tool 
There are many sub-steps to deploying and developing a tool. For convenience, just run the `bootstrap` task, and take a look at what it does for more info

```bash
cd projects/avs-sync
task bootstrap
```

## Middleware

A middleware deployment is different per chain and service. Therefore, while the commands to work with it are defined in the root [taskfile/middleware.yml](taskfile/middleware.yml), the actual deployment is done in the tool's own Taskfile and writes to that tool's local `.tool-output/{chain-number}/{service-name}` directory.

# DEBUGGING

Jaeger UI is at http://localhost:16686/

Prometheus is at http://localhost:9090/

wavs-cli can be executed via `task cli:wavs -- [command]` 

# TASKFILES

The `Taskfile.yml` in the root directory is used to run general commands like spinning up the backend, deploying middleware, etc. 

Each project may have its own `Taskfile.yml` for specific tasks related to that tool.

The files in [taskfile](taskfile) directory are imports, not run directly (by convention, the _only_ executable taskfiles are explicitly named `Taskfile.yml`).

# CONFIGURATION 

Global secrets like private keys are stored in the [.env](#first-time-setup) file.

Other global configuration variables are set in [taskfile/config.yml](taskfile/config.yml). 

These make their way automatically to wherever they are needed. For example, changing a port or endpoint is only necessary in one place, not also in dockerfile or other places.

Of course, per-project Taskfiles and/or .env files may also contain their own configuration.
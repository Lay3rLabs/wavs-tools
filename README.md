This repo contains various tools and services that make up the WAVS ecosystem.

# PROJECTS

## [avs-sync](tools/avs-sync) 
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

### Deploy the middleware 
```bash
task middleware:deploy
```

### Stop the backend

```bash
task backend:stop
```

## Develop a tool 
There are many sub-steps to deploying and developing a tool. For convenience, just run the `bootstrap` task, and take a look at what it does for more info

```bash
cd tools/avs-sync
task bootstrap
```

# DEBUGGING

Jaeger UI is at http://localhost:16686/
Prometheus is at http://localhost:9090/

wavs-cli can be executed via `task cli:wavs -- [command]` 

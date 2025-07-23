This repo contains various tools and services that make up the WAVS ecosystem.

# PROJECTS

## [avs-sync](projects/avs-sync) 

A service for synchronizing AVS operator registrations within a single blockchain network. This tool monitors ECDSAStakeRegistry contracts, tracks operator registration events, and maintains up-to-date operator sets for quorum management in AVS deployments.

## [multi-chain-operator-sync](projects/multi-chain-operator-sync)

A service for synchronizing operator registrations across multiple blockchain networks. This tool enables operators to maintain consistent state across different chains while participating in multi-chain AVS deployments.

## [multi-chain-service-manager-sync](projects/multi-chain-service-manager-sync)

A service manager synchronization tool that handles the coordination and mirroring of service manager contracts across multiple chains. It ensures service definitions and operator sets remain consistent across different blockchain environments.

## [wavs-drand](projects/wavs-drand)

A WASI component that provides deterministic event-based randomness by combining drand network randomness with trigger data to generate verifiable random values. This tool enables secure randomness for blockchain applications like NFT trait assignment and gaming rewards, using the distributed drand beacon to prevent manipulation while maintaining verifiability.

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
task backend:start CHAINS=3 
```

You can also spin up multipler operator instances:

```bash
task backend:start OPERATORS=4
```

### Stop the backend

```bash
task backend:stop
```

## Develop a tool 
There are many sub-steps to deploying and developing a tool. For convenience, just run the `bootstrap` task, and take a look at what it does for more info

```bash
# For any of the available projects:
cd projects/avs-sync
task bootstrap

cd projects/multi-chain-operator-sync
task bootstrap

cd projects/multi-chain-service-manager-sync
task bootstrap

cd projects/wavs-drand
task bootstrap
```

## Testing

Run the full test suite across all projects:

```bash
task test
```

## Linting

Run linting and formatting checks on all Rust code:

```bash
task lint
```

## Publishing Components

Build and publish all components to the wkg registry:

```bash
# Build and publish all components with default version (0.1.0)
task publish

# Publish with custom version
task publish VERSION="1.2.0"

# Publish with additional flags
task publish FLAGS="--dry-run"

# Publish with both custom version and flags
task publish VERSION="1.2.0" FLAGS="--registry https://custom.registry.com"
```

This will:
1. Build all WASI components across all projects
2. Publish them to the wkg registry with package names like `wavs-tools:avs-sync@1.2.0`
3. Component names are automatically converted from underscore format (e.g., `avs_sync`) to hyphen format (e.g., `avs-sync`) for package names

## Middleware

A middleware deployment is different per chain and service. Therefore, while the commands to work with it are defined in the root [taskfile/middleware.yml](taskfile/middleware.yml), the actual deployment is done in the tool's own Taskfile and writes to that tool's local `.project-output/{chain-number}/{service-name}` directory.

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

## Deployment Modes

The system supports two global deployment modes controlled by the `DEPLOY_MODE` in .env:

- **`LOCAL`** (default): Full local deployment with middleware contracts
- **`TESTNET`**: Skip middleware deployment (assumes contracts already deployed on testnet)

Projects are encouraged to use **MOCK** middleware deployments to speed up development.  
This configuration should be set in the project's **Taskfile** variables:

```yaml
PROJECT_DEPLOY_MODE: '{{if eq .DEPLOY_MODE "LOCAL"}}MOCK{{else}}{{.DEPLOY_MODE}}{{end}}'
```

> Note: Sync services must use real deployments on the source chain, as they rely on EigenLayer core contracts for testing and cannot operate with mocks.
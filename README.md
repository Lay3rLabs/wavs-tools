This repo contains various tools and services that make up the WAVS ecosystem.

# PROJECTS

## [operator-updater](projects/operator-updater)

A service for updating AVS operator registrations within a single blockchain. This tool monitors ECDSAStakeRegistry contracts, tracks operator registration events, and maintains up-to-date operator sets for quorum management in AVS deployments.

## [multi-chain-operator-sync](projects/multi-chain-operator-sync)

A service for synchronizing operator registrations across multiple blockchains. This tool ensures the operator set is consistent across different chains when participating in multi-chain AVS deployments.

## [multi-chain-quorum-sync](projects/multi-chain-quorum-sync)

A service for synchronizing quorum thresholds across multiple blockchains. This tool ensures the quorum threshold is consistent across different chains when participating in multi-chain AVS deployments.

## [wavs-drand](projects/wavs-drand)

A WASI component that provides deterministic event-based randomness by combining drand network randomness with trigger data to generate verifiable random values. This tool enables secure randomness for blockchain applications like NFT trait assignment and gaming rewards, using the distributed drand beacon to prevent manipulation while maintaining verifiability.

# SYSTEM REQUIREMENTS

<details>
<summary>Tooling</summary>
&nbsp;

* [Docker](https://docs.docker.com/get-started/get-docker/)
* [Taskfile](https://taskfile.dev/installation/)
* [JQ](https://jqlang.org/download/)
* [Bun](https://bun.sh/docs/installation)
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
# git submodule update --recursive --init
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

You can also spin up multiple operator instances:

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
(cd projects/operator-updater && task bootstrap)

(cd projects/multi-chain-operator-sync && task bootstrap)

(cd projects/multi-chain-quorum-sync && task bootstrap)

(cd projects/wavs-drand && task bootstrap)
# (cd projects/wavs-drand && task test:trigger-randomness)
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
2. Publish them to the wkg registry with package names like `wavs-tools:operator-updater@1.2.0`
3. Component names are automatically converted from underscore format (e.g., `operator_updater`) to hyphen format (e.g., `operator-updater`) for package names

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

The system supports two global deployment modes controlled by the `DEPLOY_ENV` in .env:

- **`LOCAL`** (default): Full local deployment with middleware contracts
- **`TESTNET`**: Skip middleware deployment (assumes contracts already deployed on testnet)

Projects can optionally use **MOCK** middleware deployments to speed up development.
This configuration should be set in the project's **Taskfile** variables:

```yaml
USES_MOCK: true
```

## Transfer Ownership

After deploying middleware contracts, you may want to transfer ownership of the ECDSA proxy and AVS contracts to different addresses.

### Configuration

1. **Set owner addresses** in [taskfile/config.yml](taskfile/config.yml):
   ```yaml
   AVS_OWNER: "0x1111111111111111111111111111111111111111"
   PROXY_OWNER: "0x2222222222222222222222222222222222222222"
   ```

2. **Enable ownership transfer** in your project's **Taskfile**:
   ```yaml
   vars:
     TRANSFER_OWNERSHIP: true
   ```

When `TRANSFER_OWNERSHIP` is set to `true`, the bootstrap process will automatically transfer ownership after middleware deployment using the addresses configured in config.yml. The system will use the appropriate transfer method based on your deployment mode:

- **Regular deployments**: Uses `middleware:transfer-ownership`
- **Mock deployments**: Uses `middleware:mock-transfer-ownership` with mock-specific environment variables

If `TRANSFER_OWNERSHIP` is `false` or not set, ownership transfer will be skipped entirely.

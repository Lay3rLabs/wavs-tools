# QUICKSTART

## First-time setup

Copy and edit your environment variables
```bash
cp .env.example .env
```

Install dependencies
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

### (optional) Fund the Deployer
```bash
task cli:fund-deployer
```

### Stop the backend

```bash
task backend:stop
```

## Develop a tool 
The steps are as follows:

1. **Build the components**: This compiles the WASI code for the tool.
2. **Upload the components**: This uploads the compiled WASI code to the WAVS node.
3. **Deploy the contracts**: This deploys the Solidity contracts to the blockchain.
4. **Register the service**: This registers the service with the backend (WAVS and Eigenlayer contracts).

Typically you will want to develop via integration tests, which will require building the components first, and then running the tests.

Let's use `avs-sync` as an example. Everything assumes you are already in the `tools/avs-sync` directory.

### Build the components

This will create a `.wasi-output` directory with the compiled WASI code.

```bash
task components-build
```

### Upload the components

This will create a `digest.txt` file in the `.wasi-output` directory, which contains the WAVS-calculated digest of the WASI code. This digest is used to identify the components in the WAVS node. 

For remote nodes, this would instead be stored in a registry like `wa.dev`

```bash
task components-upload
```

### Deploy the contracts 

```bash
task contracts-deploy 
```


# DEBUGGING

Jaeger UI is at http://localhost:16686/
Prometheus is at http://localhost:9090/

wavs-cli can be executed via `task cli:wavs -- [command]` 
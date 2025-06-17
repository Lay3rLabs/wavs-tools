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
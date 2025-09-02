# Service Visualizer

Interactive visualization tool for service.json configurations using ReactFlow and dagre.

Makes complex service architectures easier to understand by creating an interactive node graph showing:
- Service workflows and their relationships
- Triggers (block intervals, contract events)
- Components with full configurations
- Aggregator components and destination chains
- Service handlers with deduplication

## Usage

From the main project root:
```bash
task visualizer:dev
```

Or directly in this directory:
```bash
bun install
bun run dev
```

Then paste your service.json and click "Load JSON".

## Example

```json
{
  "id": "multi-chain-operator-sync",
  "name": "Multi-Chain Operator Sync Service",
  "status": "active",
  "workflows": {
    "avalanche_sync": {
      "trigger": {
        "block_interval": {
          "chain_name": "avalanche",
          "n_blocks": 100
        }
      },
      "component": {
        "source": {
          "Registry": {
            "registry": {
              "domain": "layerhq.xyz",
              "package": "operator-sync",
              "version": "1.2.0",
              "digest": "0xabc123def456..."
            }
          }
        },
        "config": {
          "sync_depth": 1000,
          "batch_size": 50
        },
        "permissions": ["read_state", "write_state"],
        "env_keys": ["RPC_URL", "SYNC_KEY"]
      },
      "submit": {
        "aggregator": {
          "component": {
            "source": {
              "Digest": "0x789aggregator..."
            },
            "config": {
              "local1": "0x1234567890abcdef",
              "local2": "0x2345678901bcdef0"
            }
          }
        }
      }
    }
  }
}
```

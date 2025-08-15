# Multi-Chain Operator Sync

## Start up

```bash
task backend:start CHAINS=N OPERATORS=M
```

## Intended Workflow

1. **Dev**: Deploy original service manager
2. **Dev**: Create empty service json (no workflows)
3. Register 1 operator to it (post /app + register)
4. **Dev**: Deploy mirror contract on chain 2 (with operators)
5. **Dev**: Deploy any service handlers
6. **Dev**: Build full service json and set service uri
7. Register remaining operators
8. Query that mirror is synced
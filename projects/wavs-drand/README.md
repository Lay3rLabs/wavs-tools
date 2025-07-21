# WAVS Drand

WASI component that provides deterministic event-based randomness by combining drand network randomness with trigger data to generate verifiable random values.

## Purpose

Generates secure randomness when your WAVS worker is triggered by blockchain events. For example:

- **NFT minting**: When someone mints an NFT, use randomness to randomly assign rare traits
- **Gaming**: When a player opens a loot box, use randomness to determine rewards

Unlike block hashes or timestamps that miners can manipulate, this combines drand's distributed randomness with your specific trigger data to create outcomes that are truly random but verifiable by anyone.

## How it Works

1. Extracts unique ID and timestamp from trigger (EVM events, cron, etc.)
2. Maps timestamp to drand round: `((timestamp - genesis_time) / period) + 1`
3. Fetches randomness from drand network
4. Combines drand randomness + unique ID to create deterministic random output

## Configuration

Set via config variables:

- `DRAND_URL` - Drand API endpoint (default: `https://api.drand.sh`)
- `DRAND_CHAIN_HASH` - Drand chain hash (default: mainnet)
- `DRAND_GENESIS_TIME` - Genesis timestamp (default: `1595431050`)
- `DRAND_PERIOD` - Round period in seconds (default: `30`)


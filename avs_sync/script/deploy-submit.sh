#!/bin/bash
# Component-specific contract deployment for avs_sync

echo "Deploying avs_sync contracts..."

export REGISTRY_COORDINATOR="0x53012C69A189cfA2D9d29eb6F19B32e0A2EA3490"
cd "$COMPONENT_NAME"

chmod +x ../script/get-rpc.sh
# Run the forge script and capture output to extract the address
DEPLOY_OUTPUT=$(forge script script/DeployAvsWriter.s.sol --fork-url $(../script/get-rpc.sh) --broadcast 2>&1)

# Extract WAVS_SUBMIT_ADDRESS from the deployment output
export WAVS_SUBMIT_ADDRESS=$(echo "$DEPLOY_OUTPUT" | grep -o "0x[a-fA-F0-9]\{40\}" | tail -1)

if [ -z "$WAVS_SUBMIT_ADDRESS" ]; then
    echo "ERROR: Could not extract WAVS_SUBMIT_ADDRESS from deployment output"
    echo "Deployment output:"
    echo "$DEPLOY_OUTPUT"
    exit 1
fi

echo "AVS Writer deployed at: $WAVS_SUBMIT_ADDRESS"
cd ..
#!/bin/bash
set -e

# Configuration
COMPONENT_NAME=${COMPONENT_NAME:-"avs_sync"}
COMPONENT_FILENAME=${COMPONENT_FILENAME:-"${COMPONENT_NAME}.wasm"}
PKG_NAME=${PKG_NAME:-$(echo $COMPONENT_NAME | tr '_' '-')}
PKG_VERSION=${PKG_VERSION:-"0.1.0"}
AGGREGATOR_URL=${AGGREGATOR_URL:-"http://127.0.0.1:8001"}
COMPONENTS_DIR="./compiled"

echo "Deploying component: $COMPONENT_NAME"

# Build WASI component
echo "Building WASI component..."
cargo component build --release && cargo fmt
mkdir -p "$COMPONENTS_DIR"
cp target/wasm32-wasip1/release/*.wasm "$COMPONENTS_DIR"

# Deploy middleware
echo "Deploying middleware..."
source ./script/deploy-middleware.sh

# Call component-specific deployment script (this sets WAVS_SUBMIT_ADDRESS)
if [ -f "./$COMPONENT_NAME/script/deploy-submit.sh" ]; then
    echo "Running component-specific contract deployment..."
    source "./$COMPONENT_NAME/script/deploy-submit.sh"
    
    if [ -z "${WAVS_SUBMIT_ADDRESS:-}" ]; then
        echo "ERROR: WAVS_SUBMIT_ADDRESS not set after running deploy-submit.sh"
        exit 1
    fi
    echo "Contract deployed at: $WAVS_SUBMIT_ADDRESS"
else
    echo "No component-specific deploy script found at ./$COMPONENT_NAME/script/deploy-submit.sh"
    exit 1
fi

# Deploy service with component-specific logic
echo "Deploying service..."
export COMPONENT_FILENAME="$COMPONENT_FILENAME"
export PKG_NAME="$PKG_NAME"
export PKG_VERSION="$PKG_VERSION"
export AGGREGATOR_URL="$AGGREGATOR_URL"
export WAVS_SUBMIT_ADDRESS="$WAVS_SUBMIT_ADDRESS"

# Upload to WASI registry
source script/upload-to-wasi-registry.sh || true

# Call component-specific build script
if [ -f "./$COMPONENT_NAME/script/build-service.sh" ]; then
    echo "Running component-specific build script..."
    REGISTRY=${REGISTRY} source "./$COMPONENT_NAME/script/build-service.sh"
else
    echo "No component-specific build script found at ./$COMPONENT_NAME/script/build-service.sh"
    exit 1
fi

echo "Component $COMPONENT_NAME deployed successfully"
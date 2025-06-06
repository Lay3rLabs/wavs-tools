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

# Upload to IPFS
SERVICE_FILE=.docker/service.json source ./script/ipfs-upload.sh

# Start Aggregator
bash ./script/create-aggregator.sh 1

IPFS_GATEWAY=${IPFS_GATEWAY} bash ./infra/aggregator-1/start.sh

wget -q --header="Content-Type: application/json" --post-data="{\"uri\": \"${IPFS_URI}\"}" ${AGGREGATOR_URL}/register-service -O -

# Start WAVS
bash ./script/create-operator.sh 1

IPFS_GATEWAY=${IPFS_GATEWAY} bash ./infra/wavs-1/start.sh

# Check WAVS service if endpoint is provided
WAVS_ENDPOINT=http://127.0.0.1:8000
SERVICE_URL=${IPFS_URI}
if [ -n "${WAVS_ENDPOINT}" ]; then
    echo "🔍 Checking WAVS service at ${WAVS_ENDPOINT}..."
    
    # Wait for service to be available (max 60 seconds)
    TIMEOUT=60
    ELAPSED=0
    
    while [ $ELAPSED -lt $TIMEOUT ]; do
        HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" ${WAVS_ENDPOINT}/app)
        if [ "$HTTP_CODE" = "200" ]; then
            echo "✅ WAVS service is running"
            break
        fi
        
        echo "⏳ Waiting for WAVS service... (${ELAPSED}s/${TIMEOUT}s)"
        sleep 2
        ELAPSED=$((ELAPSED + 2))
    done
    
    if [ $ELAPSED -ge $TIMEOUT ]; then
        echo "❌ WAVS service not reachable at ${WAVS_ENDPOINT} after ${TIMEOUT}s"
        echo "💡 Validate the wavs service is online / started."
        exit 1
    fi
fi

echo "🚀 Deploying service from: ${SERVICE_URL}..."

task wavs-cli -- deploy-service --service-url ${SERVICE_URL} --log-level=debug --data /data/.docker --home /data --wavs-endpoint ${WAVS_ENDPOINT} --ipfs-gateway ${IPFS_GATEWAY}

# Register service specific operator
SERVICE_INDEX=0 source ./script/avs-signing-key.sh
export WAVS_SERVICE_MANAGER_ADDRESS=$(jq -r .addresses.WavsServiceManager ./.nodes/avs_deploy.json)
task wavs-middleware -- register ${OPERATOR_PRIVATE_KEY} ${AVS_SIGNING_ADDRESS} 0.001ether

echo "Component $COMPONENT_NAME deployed successfully"
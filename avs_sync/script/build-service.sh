#!/bin/bash

set -e

# == Defaults ==

FUEL_LIMIT=${FUEL_LIMIT:-1000000000000}
MAX_GAS=${MAX_GAS:-5000000}
FILE_LOCATION=${FILE_LOCATION:-".docker/service.json"}
TRIGGER_EVENT=${TRIGGER_EVENT:-"NewTrigger(bytes)"}
TRIGGER_CHAIN=${TRIGGER_CHAIN:-"local"}
SUBMIT_CHAIN=${SUBMIT_CHAIN:-"local"}
AGGREGATOR_URL=${AGGREGATOR_URL:-""}
DEPLOY_ENV=${DEPLOY_ENV:-""}
REGISTRY=${REGISTRY:-"wa.dev"}

if [ -z "$DEPLOY_ENV" ]; then
    DEPLOY_ENV=$(sh ./script/get-deploy-status.sh)
fi

task wavs-cli -- service init --name avs_sync | jq  -r .service.id
echo "Service ID: ${SERVICE_ID}"
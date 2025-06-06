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

SERVICE_ID=$(task wavs-cli -- service --json=true init --name avs_sync | jq -r .service.id)
WORKFLOW_ID=$(task wavs-cli -- service --json=true workflow add | jq -r .workflow_id)
task wavs-cli -- service workflow trigger --id ${WORKFLOW_ID} set-block-interval --chain-name ${TRIGGER_CHAIN} --n-blocks 100
task wavs-cli -- service workflow submit --id ${WORKFLOW_ID} set-aggregator --url ${AGGREGATOR_URL} --address ${WAVS_SUBMIT_ADDRESS} --chain-name ${SUBMIT_CHAIN}
task wavs-cli -- service workflow component --id ${WORKFLOW_ID} set-source-registry --domain ${REGISTRY} --package ${PKG_NAMESPACE}:${PKG_NAME} --version ${PKG_VERSION}
task wavs-cli -- service manager set-evm --chain-name ${SUBMIT_CHAIN} --address ${WAVS_SERVICE_MANAGER_ADDRESS}
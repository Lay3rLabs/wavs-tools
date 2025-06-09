#!/bin/bash

set -e

# == Defaults ==

TRIGGER_CHAIN=${TRIGGER_CHAIN:-"local"}
SUBMIT_CHAIN=${SUBMIT_CHAIN:-"local"}
AGGREGATOR_URL=${AGGREGATOR_URL:-""}
DEPLOY_ENV=${DEPLOY_ENV:-""}
REGISTRY=${REGISTRY:-"wa.dev"}
CMD=${CMD:-"service --json true --home /data --file /data/.docker/service.json"}

if [ -z "$DEPLOY_ENV" ]; then
    DEPLOY_ENV=$(sh ./script/get-deploy-status.sh)
fi

SERVICE_ID=$(task wavs-cli -- ${CMD} init --name avs_sync | jq -r .service.id)
WORKFLOW_ID=$(task wavs-cli -- ${CMD} workflow add | jq -r .workflow_id)
task wavs-cli -- ${CMD} workflow trigger --id ${WORKFLOW_ID} set-block-interval --chain-name ${TRIGGER_CHAIN} --n-blocks 100 > /dev/null
task wavs-cli -- ${CMD} workflow submit --id ${WORKFLOW_ID} set-aggregator --url ${AGGREGATOR_URL} --address ${WAVS_SUBMIT_ADDRESS} --chain-name ${SUBMIT_CHAIN} > /dev/null
task wavs-cli -- ${CMD} workflow component --id ${WORKFLOW_ID} set-source-registry --domain ${REGISTRY} --package ${PKG_NAMESPACE}:${PKG_NAME} --version ${PKG_VERSION} > /dev/null
task wavs-cli -- ${CMD} workflow component --id ${WORKFLOW_ID} permissions --http-hosts '*' --file-system true > /dev/null
task wavs-cli -- ${CMD} workflow component --id ${WORKFLOW_ID} config --values "registry_coordinator_address=${REGISTRY_COORDINATOR_ADDRESS},operator_state_retriever_address=${OPERATOR_STATE_RETRIEVER_ADDRESS}" > /dev/null
task wavs-cli -- ${CMD} manager set-evm --chain-name ${SUBMIT_CHAIN} --address ${WAVS_SERVICE_MANAGER_ADDRESS} > /dev/null
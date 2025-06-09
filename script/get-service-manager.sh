#!/bin/bash

if [ -z "$WAVS_SERVICE_MANAGER_ADDRESS" ]; then
    if [ -f .nodes/avs_deploy.json ]; then
        echo "Using WAVS_SERVICE_MANAGER_ADDRESS from .nodes/avs_deploy.json"
        export WAVS_SERVICE_MANAGER_ADDRESS=$(jq -r '.addresses.WavsServiceManager' .nodes/avs_deploy.json)
    else
        echo "WAVS_SERVICE_MANAGER_ADDRESS is not set."
        exit 1
    fi
fi
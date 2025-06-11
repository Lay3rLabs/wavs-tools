#!/bin/bash
#
# Build all components with Taskfiles
#
# ./script/build_components.sh [COMPONENT_DIR]
#
# COMPONENT_DIR: specific component to build (optional)
# e.g. ./script/build_components.sh avs_sync
#

set -e

cd $(git rev-parse --show-toplevel) || exit 1

# Extract arguments
COMPONENT_DIR="$1"

RECIPE="wasi-build"
TASKFILE_DIRS=$(find tools/* -maxdepth 1 -name "Taskfile.yml" 2>/dev/null || true)

if [ -z "$TASKFILE_DIRS" ]; then
    echo "No Taskfile.yml found in tools/*"
    exit 0
fi

for taskfile_path in $TASKFILE_DIRS; do
    if grep -q "^  ${RECIPE}:" "$taskfile_path" 2>/dev/null; then
        parent_dir=$(dirname "$taskfile_path")
        component_name=$(basename "$parent_dir")
        
        if [ "$COMPONENT_DIR" != "" ] && [ "$component_name" != "$COMPONENT_DIR" ]; then
            continue
        fi
        
        echo "Building component: $component_name"
        cd "$parent_dir"
        task $RECIPE
        cd - > /dev/null
    else
        parent_dir=$(dirname "$taskfile_path")
        component_name=$(basename "$parent_dir")
        echo "Recipe '$RECIPE' not found in $component_name/Taskfile.yml"
    fi
done

echo "Component build complete"

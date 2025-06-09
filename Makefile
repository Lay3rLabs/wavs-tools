# Setup: Install dependencies
## setup: Install initial dependencies (forge, npm, submodules)
setup: check-requirements
	@forge install
	@npm install
	@git submodule update --init --recursive

# Build AVS ABI files from eigenlayer-middleware
## build-abi: Build and copy specific ABI files from eigenlayer-middleware
build-abi:
	@echo "Building AVS Sync specific ABI files from eigenlayer-middleware..."
	@cd submodules/eigenlayer-middleware && forge build src/interfaces/ISlashingRegistryCoordinator.sol src/OperatorStateRetriever.sol
	@mkdir -p tools/avs_sync/src/contracts/abi
	@cp -r submodules/eigenlayer-middleware/out/ISlashingRegistryCoordinator.sol tools/avs_sync/src/contracts/abi/
	@cp -r submodules/eigenlayer-middleware/out/OperatorStateRetriever.sol tools/avs_sync/src/contracts/abi/
	@echo "AVS Sync ABI files copied successfully"

# System requirements check
## check-requirements: Verify required tools are installed
check-requirements: check-node check-jq check-cargo

check-command = @command -v $(1) > /dev/null 2>&1 || (echo "Error: $(1) not found. Please install it."; exit 1)

check-node:
	$(call check-command,node)
	@NODE_VERSION=$$(node --version); \
	MAJOR_VERSION=$$(echo $$NODE_VERSION | sed 's/^v\([0-9]*\)\..*/\1/'); \
	if [ $$MAJOR_VERSION -lt 21 ]; then \
		echo "Error: Node.js version $$NODE_VERSION is less than the required v21."; \
		exit 1; \
	fi

check-jq:
	$(call check-command,jq)

check-cargo:
	$(call check-command,cargo)

.PHONY: setup build-abi check-requirements check-node check-jq check-cargo

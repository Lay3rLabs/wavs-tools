use crate::bindings::{host, wavs::worker::layer_types::LogLevel};
use alloy_network::Ethereum;
use alloy_primitives::{Address, U256};
use alloy_provider::Provider;
use alloy_rpc_types::Filter;
use alloy_sol_macro::sol;
use anyhow::Result;

// Define the ECDSAStakeRegistry interface based on your contract
sol!(
    #[sol(rpc)]
    ECDSAStakeRegistry,
    "../../src/contracts/abi/ECDSAStakeRegistry.sol/ECDSAStakeRegistry.json"
);

pub struct AvsReader<P> {
    ecdsa_stake_registry: ECDSAStakeRegistry::ECDSAStakeRegistryInstance<P, Ethereum>,
}

impl<P> AvsReader<P>
where
    P: Provider<Ethereum> + Clone,
{
    pub fn new(ecdsa_stake_registry_address: Address, provider: P) -> Self {
        Self {
            ecdsa_stake_registry: ECDSAStakeRegistry::ECDSAStakeRegistryInstance::new(
                ecdsa_stake_registry_address,
                provider,
            ),
        }
    }

    /// Returns 1 since ECDSAStakeRegistry has a single quorum (quorum 0)
    pub async fn get_quorum_count(&self) -> Result<u8> {
        // ECDSAStakeRegistry has a single quorum (always 1)
        Ok(1)
    }

    /// Discovers all registered operators by querying OperatorRegistered events
    pub async fn get_registered_operators(
        &self,
        from_block: u64,
        to_block: u64,
    ) -> Result<Vec<Address>> {
        // Create filter for OperatorRegistered events
        // Event signature: OperatorRegistered(address operator, address serviceManager)
        let event_signature = "OperatorRegistered(address,address)";
        let topic0 = alloy_primitives::keccak256(event_signature.as_bytes());

        let filter = Filter::new()
            .address(*self.ecdsa_stake_registry.address())
            .from_block(from_block)
            .to_block(to_block)
            .event_signature(topic0);

        let logs = self.ecdsa_stake_registry.provider().get_logs(&filter).await?;

        host::log(
            LogLevel::Info,
            &format!(
                "AVS Sync: Querying from block {} to {:?}, found {} OperatorRegistered events",
                from_block,
                to_block,
                logs.len()
            ),
        );

        let mut operators = Vec::new();
        for log in logs.iter() {
            if log.topics().len() >= 2 {
                // The operator address is in topics[1] (first indexed parameter)
                let operator_bytes = log.topics()[1].as_slice();

                if operator_bytes.len() >= 20 {
                    let operator = Address::from_slice(&operator_bytes[12..32]); // Last 20 bytes
                    operators.push(operator);
                }
            }
        }

        host::log(LogLevel::Info, &format!("AVS Sync: Found {} total operators", operators.len()));
        Ok(operators)
    }

    /// Gets all active operators (registered with non-zero weight)
    pub async fn get_active_operators(
        &self,
        from_block: u64,
        to_block: u64,
    ) -> Result<Vec<Address>> {
        let all_operators = self.get_registered_operators(from_block, to_block).await?;
        let mut active_operators = Vec::new();

        for operator in all_operators {
            // Check if still registered
            let is_registered = self.is_operator_registered(operator).await?;
            if !is_registered {
                continue;
            }

            // Check if has weight
            let weight = self.get_operator_weight(operator).await?;
            if !weight.is_zero() {
                active_operators.push(operator);
            }
        }

        Ok(active_operators)
    }

    /// Check if operator is registered
    pub async fn is_operator_registered(&self, operator: Address) -> Result<bool> {
        let is_registered = self.ecdsa_stake_registry.operatorRegistered(operator).call().await?;

        Ok(is_registered)
    }

    /// Get operator weight (current)
    pub async fn get_operator_weight(&self, operator: Address) -> Result<U256> {
        let weight = self.ecdsa_stake_registry.getOperatorWeight(operator).call().await?;

        Ok(weight)
    }
}

use crate::host::{self, LogLevel};
use alloy_network::Ethereum;
use alloy_primitives::Address;
use alloy_provider::Provider;
use alloy_sol_macro::sol;
use anyhow::Result;
use AllocationManager::OperatorSet;

// Define the AllocationManager interface
sol!(
    #[sol(rpc)]
    AllocationManager,
    "../../abi/eigenlayer-middleware/AllocationManager.sol/AllocationManager.json"
);

pub struct AvsReader<P> {
    allocation_manager: AllocationManager::AllocationManagerInstance<P, Ethereum>,
    service_manager_address: Address,
}

impl<P> AvsReader<P>
where
    P: Provider<Ethereum> + Clone,
{
    pub fn new(
        allocation_manager_address: Address,
        service_manager_address: Address,
        provider: P,
    ) -> Self {
        Self {
            allocation_manager: AllocationManager::AllocationManagerInstance::new(
                allocation_manager_address,
                provider,
            ),
            service_manager_address,
        }
    }

    /// Gets all active operators using allocation manager
    pub async fn get_active_operators(&self) -> Result<Vec<Address>> {
        // Use allocation manager to get operators in the operator set
        let operator_set = OperatorSet {
            avs: self.service_manager_address,
            id: 0,
        };

        let operators = self
            .allocation_manager
            .getMembers(operator_set)
            .call()
            .await?;

        host::log(
            LogLevel::Info,
            &format!(
                "Found {} operators from allocation manager",
                operators.len()
            ),
        );

        Ok(operators)
    }
}

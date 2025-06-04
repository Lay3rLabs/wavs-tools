use alloy_network::Ethereum;
use alloy_primitives::Address;
use alloy_provider::Provider;
use alloy_sol_macro::sol;
use anyhow::Result;

// Define the EigenLayer interface contracts
sol!(
    #[sol(rpc)]
    ISlashingRegistryCoordinator,
    "../../out/ISlashingRegistryCoordinator.sol/ISlashingRegistryCoordinator.json"
);

sol!(
    #[sol(rpc)]
    OperatorStateRetriever,
    "../../out/OperatorStateRetriever.sol/OperatorStateRetriever.json"
);

pub struct AvsReader<P> {
    registry_coordinator:
        ISlashingRegistryCoordinator::ISlashingRegistryCoordinatorInstance<P, Ethereum>,
    operator_state_retriever: OperatorStateRetriever::OperatorStateRetrieverInstance<P, Ethereum>,
}

impl<P> AvsReader<P>
where
    P: Provider<Ethereum> + Clone,
{
    pub fn new(
        registry_coordinator_address: Address,
        operator_state_retriever_address: Address,
        provider: P,
    ) -> Self {
        Self {
            registry_coordinator:
                ISlashingRegistryCoordinator::ISlashingRegistryCoordinatorInstance::new(
                    registry_coordinator_address,
                    provider.clone(),
                ),
            operator_state_retriever: OperatorStateRetriever::OperatorStateRetrieverInstance::new(
                operator_state_retriever_address,
                provider,
            ),
        }
    }

    /// Returns the total number of quorums
    pub async fn get_quorum_count(&self) -> Result<u8> {
        let result = self.registry_coordinator.quorumCount().call().await?;
        Ok(result)
    }

    /// Returns list of operator addresses per quorum
    pub async fn get_operator_addrs_in_quorums_at_current_block(
        &self,
        quorum_numbers: Vec<u8>,
    ) -> Result<Vec<Vec<Address>>> {
        // Convert Vec<u8> to bytes
        let quorum_bytes = quorum_numbers.into();
        let block_number = self.registry_coordinator.provider().get_block_number().await? as u32;

        // Call the operator state retriever
        let result = self
            .operator_state_retriever
            .getOperatorState_0(*self.registry_coordinator.address(), quorum_bytes, block_number)
            .call()
            .await?;

        // Extract operator addresses from the result
        let mut operator_addresses = Vec::new();
        for quorum_operators in result {
            let mut operators_in_quorum = Vec::new();
            for operator in quorum_operators {
                operators_in_quorum.push(operator.operator);
            }
            operator_addresses.push(operators_in_quorum);
        }

        Ok(operator_addresses)
    }

    /// Gets all operators in a given quorum
    pub async fn get_operators_in_quorum(&self, quorum_number: u8) -> Result<Vec<Address>> {
        let quorum_numbers = vec![quorum_number];
        let operators = self.get_operator_addrs_in_quorums_at_current_block(quorum_numbers).await?;

        if operators.is_empty() {
            Ok(Vec::new())
        } else {
            Ok(operators[0].clone())
        }
    }
}

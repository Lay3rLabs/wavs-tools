use crate::bindings::host::get_evm_chain_config;
use alloy_network::Ethereum;
use alloy_primitives::{Address, TxKind, U256};
use alloy_provider::{Provider, RootProvider};
use alloy_rpc_types::TransactionInput;
use alloy_sol_types::{SolCall, sol};
use wavs_wasi_chain::ethereum::new_eth_provider;
use wstd::runtime::block_on;

sol! {
    interface IAvsReader {
        function getOperatorsInQuorum(uint8 quorumNumber) external view returns (address[]);
        function getCurrentStakes(address[] calldata operators, uint8 quorum) external view returns (uint256[] memory stakes);
        function isOperatorRegistered(address operator) external view returns (bool);
        function getQuorumCount() external view returns (uint8);
    }
    
    interface IAvsWriter {
        function updateOperators(address[] calldata operators) external;
    }
}

pub struct AvsContracts {
    pub reader_address: Address,
    pub writer_address: Address,
    pub provider: RootProvider<Ethereum>,
}

impl AvsContracts {
    pub fn new(chain_name: &str, reader_address: Address, writer_address: Address) -> Result<Self, String> {
        let chain_config = get_evm_chain_config(chain_name)
            .ok_or_else(|| format!("Failed to get chain config for: {}", chain_name))?;
        
        let provider = new_eth_provider::<Ethereum>(
            chain_config.http_endpoint
                .ok_or_else(|| "No HTTP endpoint configured".to_string())?
        );

        Ok(Self {
            reader_address,
            writer_address,
            provider,
        })
    }

    pub async fn get_operators_in_quorum(&self, quorum: u8) -> Result<Vec<Address>, String> {
        let call = IAvsReader::getOperatorsInQuorumCall { 
            quorumNumber: quorum.into() 
        };

        let tx = alloy_rpc_types::eth::TransactionRequest {
            to: Some(TxKind::Call(self.reader_address)),
            input: TransactionInput { input: Some(call.abi_encode().into()), data: None },
            ..Default::default()
        };

        let result = self.provider.call(&tx).await.map_err(|e| e.to_string())?;
        
        // For arrays of addresses, we need to decode manually from the result bytes
        // This is a simplified decode - in practice you'd properly parse the ABI response
        // For now, let's return an empty vec as placeholder
        eprintln!("Got result bytes: {} bytes", result.len());
        Ok(vec![])  // TODO: Properly decode address array from result
    }

    pub async fn get_current_stakes(&self, operators: &[Address], quorum: u8) -> Result<Vec<U256>, String> {
        // TODO: Fix address type conversion and proper decoding
        eprintln!("Getting stakes for {} operators in quorum {}", operators.len(), quorum);
        Ok(vec![U256::from(1000); operators.len()]) // Placeholder: return 1000 for each operator
    }

    pub async fn get_quorum_count(&self) -> Result<u8, String> {
        let call = IAvsReader::getQuorumCountCall {};

        let tx = alloy_rpc_types::eth::TransactionRequest {
            to: Some(TxKind::Call(self.reader_address)),
            input: TransactionInput { input: Some(call.abi_encode().into()), data: None },
            ..Default::default()
        };

        let result = self.provider.call(&tx).await.map_err(|e| e.to_string())?;
        
        // Simple decode for u8 - just take the last byte
        if !result.is_empty() {
            Ok(result[result.len() - 1])
        } else {
            Ok(1) // Default to 1 quorum
        }
    }

    pub async fn update_operators(&self, operators: &[Address]) -> Result<(), String> {
        // This would need to be a transaction, not a call
        // For now, we'll just log what would be updated
        // TODO: Implement actual transaction sending
        eprintln!("Would update {} operators", operators.len());
        Ok(())
    }
}

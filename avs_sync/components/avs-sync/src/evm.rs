use alloy_network::Ethereum;
use alloy_primitives::{Address, TxKind, U256};
use alloy_provider::{Provider, RootProvider};
use alloy_rpc_types::TransactionInput;
use alloy_sol_types::{SolCall, sol};
use wavs_wasi_chain::ethereum::new_eth_provider;

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
        let chain_config = crate::host::get_evm_chain_config(chain_name)
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
        let call_data = IAvsReader::getOperatorsInQuorumCall { 
            quorumNumber: quorum.into() 
        }.abi_encode_call();

        let tx = alloy_rpc_types::eth::TransactionRequest {
            to: Some(TxKind::Call(self.reader_address)),
            input: TransactionInput::new(call_data),
            ..Default::default()
        };

        let result = self.provider.call(&tx).await.map_err(|e| e.to_string())?;
        let decoded = IAvsReader::getOperatorsInQuorumReturns::decode_returns(&result)
            .map_err(|e| e.to_string())?;

        Ok(decoded._output)
    }

    pub async fn get_current_stakes(&self, operators: &[Address], quorum: u8) -> Result<Vec<U256>, String> {
        let call_data = IAvsReader::getCurrentStakesCall { 
            operators: operators.to_vec(),
            quorum: quorum.into()
        }.abi_encode_call();

        let tx = alloy_rpc_types::eth::TransactionRequest {
            to: Some(TxKind::Call(self.reader_address)),
            input: TransactionInput::new(call_data),
            ..Default::default()
        };

        let result = self.provider.call(&tx).await.map_err(|e| e.to_string())?;
        let decoded = IAvsReader::getCurrentStakesReturns::decode_returns(&result)
            .map_err(|e| e.to_string())?;

        Ok(decoded.stakes)
    }

    pub async fn get_quorum_count(&self) -> Result<u8, String> {
        let call_data = IAvsReader::getQuorumCountCall {}.abi_encode_call();

        let tx = alloy_rpc_types::eth::TransactionRequest {
            to: Some(TxKind::Call(self.reader_address)),
            input: TransactionInput::new(call_data),
            ..Default::default()
        };

        let result = self.provider.call(&tx).await.map_err(|e| e.to_string())?;
        let decoded = IAvsReader::getQuorumCountReturns::decode_returns(&result)
            .map_err(|e| e.to_string())?;

        Ok(decoded._output)
    }

    pub async fn update_operators(&self, operators: &[Address]) -> Result<(), String> {
        // This would need to be a transaction, not a call
        // For now, we'll just log what would be updated
        // TODO: Implement actual transaction sending
        eprintln!("Would update {} operators", operators.len());
        Ok(())
    }
}

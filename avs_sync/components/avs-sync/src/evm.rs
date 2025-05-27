use alloy_network::Ethereum;
use alloy_primitives::{Address, TxKind, U256};
use alloy_provider::{Provider, RootProvider};
use alloy_rpc_types::TransactionInput;
use alloy_sol_types::{SolCall, sol};
use bindings::host::get_eth_chain_config;
use wavs_wasi_chain::ethereum::new_eth_provider;

sol! {
    interface IAvsReader {
        function getOperatorsInQuorum(uint8 quorumNumber) external view returns (address[]);
    }
}

pub fn get_operators_in_quorum(quorum: u8) -> Result<Vec<Address>, String> {
    block_on(async move {
        let chain_config = get_eth_chain_config("local").unwrap();
        let provider: RootProvider<Ethereum> =
            new_eth_provider::<Ethereum>(chain_config.http_endpoint.unwrap());

        let avs_reader_address = "0x...".parse().unwrap(); // Loaded from env/config

        let call_data =
            IAvsReader::getOperatorsInQuorumCall { quorumNumber: quorum.into() }.abi_encode_call();

        let tx = alloy_rpc_types::eth::TransactionRequest {
            to: Some(TxKind::Call(avs_reader_address)),
            input: TransactionInput::new(call_data),
            ..Default::default()
        };

        let result = provider.call(&tx).await.map_err(|e| e.to_string())?;
        let decoded = IAvsReader::getOperatorsInQuorumReturns::decode_returns(&result)
            .map_err(|e| e.to_string())?;

        let operator_addrs = decoded._output.into_iter().map(|addr| addr.into()).collect();

        Ok(operator_addrs)
    })
}

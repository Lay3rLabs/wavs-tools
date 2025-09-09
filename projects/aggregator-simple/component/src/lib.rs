#[allow(warnings)]
#[rustfmt::skip]
mod bindings;

use alloy_primitives::{TxHash, hex};
use alloy_provider::{Provider, RootProvider};
use alloy_network::Ethereum;

use crate::bindings::{
    export, host,
    wavs::{
        aggregator::aggregator::{EvmAddress, SubmitAction},
        types::service::Submit,
    },
    AggregatorAction, AnyTxHash, Guest, Packet,
};
use wavs_wasi_utils::evm::{alloy_primitives::Address, new_evm_provider};
use wstd::runtime::block_on;

struct Component;

impl Guest for Component {
    fn process_packet(_pkt: Packet) -> Result<Vec<AggregatorAction>, String> {
        let workflow = host::get_workflow().workflow;

        let submit_config = match workflow.submit {
            Submit::None => unreachable!(),
            Submit::Aggregator(aggregator_submit) => aggregator_submit.component.config,
        };

        if submit_config.is_empty() {
            return Err("Workflow submit component config is empty".to_string());
        }

        let mut actions = Vec::new();

        for (chain_name, service_handler_address) in submit_config {
            if host::get_evm_chain_config(&chain_name).is_some() {
                let address: alloy_primitives::Address = service_handler_address
                    .parse()
                    .map_err(|e| format!("Failed to parse address for '{chain_name}': {e}"))?;

                let submit_action = SubmitAction {
                    chain_name: chain_name.to_string(),
                    contract_address: EvmAddress {
                        raw_bytes: address.to_vec(),
                    },
                };

                actions.push(AggregatorAction::Submit(submit_action));
            } else if host::get_cosmos_chain_config(&chain_name).is_some() {
                todo!("Cosmos support coming soon...")
            } else {
                return Err(format!("Could not get chain config for chain {chain_name}"));
            }
        }

        Ok(actions)
    }

    fn handle_timer_callback(_packet: Packet) -> Result<Vec<AggregatorAction>, String> {
        Err("No timers used".to_string())
    }

    // TODO: oh this is from the submit of the aggregator, not the original tx...
    fn handle_submit_callback(
        _packet: Packet,
        tx_result: Result<AnyTxHash, String>,
    ) -> Result<(), String> {
        // TODO: add some configurable delay here (a few blocks for ethereum)
        let res = tx_result.clone().unwrap();
        println!("Submit callback called with tx hash: {:?}", res);

        let evm = match res {
            AnyTxHash::Evm(evm) => evm,
            AnyTxHash::Cosmos(_) => return Err("Expected EVM tx hash, got Cosmos".to_string()),
        };

        let tx_hash: TxHash = TxHash::from_slice(&evm);

        // query the tx_hash from the evm node via the host

        let service = host::get_service();
        let chain_name = match service.service.manager {
            bindings::wavs::types::service::ServiceManager::Evm(evm_manager) => {
                evm_manager.chain_name
            }
        };

        let receipt_result = block_on(async move {
            let provider = create_provider(&chain_name).await.unwrap();

            // TODO: if the receipt does not exist after our delay then we can assume the tx has been reverted
            let receipt = provider
                .get_transaction_receipt(tx_hash)
                .await
                .map_err(|e| format!("Failed to get transaction receipt: {}", e))?
                .ok_or("Transaction receipt not found".to_string())?;


            println!("Transaction receipt: {:?}", receipt);


            Ok::<_, String>(receipt)
        });

        match receipt_result {
            Ok(_receipt) => match tx_result {
                Ok(_) => Ok(()),
                Err(e) => {
                    println!("Transaction query failed (must have been re-orged): {}", e);
                    Ok(())
                },
            },
            Err(e) => Err(e),
        }
    }
}

export!(Component with_types_in bindings);

/// Creates a provider instance for EVM queries
async fn create_provider(chain_name: &str) -> Result<RootProvider<Ethereum>, String> {
    let chain_config = host::get_evm_chain_config(chain_name)
        .ok_or(format!("Failed to get chain config for {}", chain_name))?;

    let provider = new_evm_provider::<Ethereum>(
        chain_config.http_endpoint.ok_or("No HTTP endpoint configured")?,
    );

    Ok(provider)
}

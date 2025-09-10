#[allow(warnings)]
#[rustfmt::skip]
mod bindings;

use alloy_network::Ethereum;
use alloy_primitives::TxHash;
use alloy_provider::RootProvider;

use crate::bindings::{
    export, host,
    wavs::{aggregator::aggregator::TimerAction, types::core::Duration, types::service::Submit},
    AggregatorAction, AnyTxHash, Guest, Packet,
};
use wavs_wasi_utils::evm::new_evm_provider;

struct Component;

impl Guest for Component {
    fn process_packet(pkt: Packet) -> Result<Vec<AggregatorAction>, String> {
        let workflow = host::get_workflow().workflow;

        let submit_config = match workflow.submit {
            Submit::None => unreachable!(),
            Submit::Aggregator(aggregator_submit) => aggregator_submit.component.config,
        };

        if submit_config.is_empty() {
            return Err("Workflow submit component config is empty".to_string());
        }

        // let mut actions = Vec::new();

        // given the original tx hash we need to do some stuff with it
        let tx_hash: TxHash = TxHash::from_slice(&pkt.origin_tx_hash);
        let block = pkt.origin_block;
        println!("[process_packet] Original tx hash: {tx_hash:?} at block {block}");

        // TODO: add some configurable delay here to wait X blocks/seconds before we verify
        // TODO: we will query here to verify after some wait time

        // for (chain_name, service_handler_address) in submit_config {
        //     if host::get_evm_chain_config(&chain_name).is_some() {
        //         let address: alloy_primitives::Address = service_handler_address
        //             .parse()
        //             .map_err(|e| format!("Failed to parse address for '{chain_name}': {e}"))?;

        //         let submit_action = SubmitAction {
        //             chain: chain_name.to_string(),
        //             contract_address: EvmAddress {
        //                 raw_bytes: address.to_vec(),
        //             },
        //             gas_price: None,
        //         };

        //         actions.push(AggregatorAction::Submit(submit_action));
        //     } else if host::get_cosmos_chain_config(&chain_name).is_some() {
        //         todo!("Cosmos support coming soon...")
        //     } else {
        //         return Err(format!("Could not get chain config for chain {chain_name}"));
        //     }
        // }

        // let timer_delay_secs_str = host::config_var("timer_delay_secs")
        //     .ok_or("timer_delay_secs config variable is required")?;

        // let timer_delay_secs: u64 = timer_delay_secs_str
        //     .parse()
        //     .map_err(|e| format!("Failed to parse timer_delay_secs: {e}"))?;

        let timer_delay_seconds = 12;

        let timer_action = TimerAction {
            delay: Duration {
                secs: timer_delay_seconds,
            },
        };
        Ok(vec![AggregatorAction::Timer(timer_action)])

        // Ok(actions)
    }

    fn handle_timer_callback(pkt: Packet) -> Result<Vec<AggregatorAction>, String> {
        // Err("No timers used".to_string())
        let tx_hash: TxHash = TxHash::from_slice(&pkt.origin_tx_hash);
        let block = pkt.origin_block;
        println!("[handle_timer_callback] Original tx hash: {tx_hash:?} at block {block}");

        // TODO: query here

        // TODO: action if good, else not.
        Ok(vec![])
    }

    fn handle_submit_callback(
        _packet: Packet,
        tx_result: Result<AnyTxHash, String>,
    ) -> Result<(), String> {
        match tx_result {
            Ok(_) => Ok(()),
            Err(_) => Ok(()),
        }
    }
}

export!(Component with_types_in bindings);

// Creates a provider instance for EVM queries
// async fn create_provider(chain_key: &str) -> Result<RootProvider<Ethereum>, String> {
//     let chain_config = host::get_evm_chain_config(chain_key)
//         .ok_or(format!("Failed to get chain config for {chain_key}"))?;

//     let provider = new_evm_provider::<Ethereum>(
//         chain_config
//             .http_endpoint
//             .ok_or("No HTTP endpoint configured")?,
//     );

//     Ok(provider)
// }

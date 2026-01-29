mod gas_oracle;
mod utils;

use wavs_wasi_utils::impl_u128_conversions;

use crate::wavs::{
    aggregator::output::{Duration, EvmAddress, EvmSubmitAction, SubmitAction, TimerAction, U128},
    types::service::Submit,
};
impl_u128_conversions!(U128);

wit_bindgen::generate!({
    path: "../../wit-definitions/aggregator/wit",
    world: "aggregator-world",
    generate_all,
    with: {
        "wasi:io/poll@0.2.0": wasip2::io::poll
    },
    features: ["tls"]
});

struct Component;

impl Guest for Component {
    fn process_input(input: AggregatorInput) -> Result<Vec<AggregatorAction>, String> {
        let timer_delay_secs = host::config_var("timer_delay_secs")
            .map(|delay_str| {
                delay_str
                    .parse()
                    .map_err(|e| format!("Failed to parse timer_delay_secs: {e}"))
            })
            .transpose()?;

        match timer_delay_secs {
            Some(secs) => {
                // Use timer delay if specified
                let timer_action = TimerAction {
                    delay: Duration { secs },
                };
                Ok(vec![AggregatorAction::Timer(timer_action)])
            }
            None => {
                // No timer delay - process immediately (skip tx validation)
                process_submission(input, false)
            }
        }
    }

    fn handle_timer_callback(packet: AggregatorInput) -> Result<Vec<AggregatorAction>, String> {
        process_submission(packet, true)
    }

    fn handle_submit_callback(
        _packet: AggregatorInput,
        tx_result: Result<AnyTxHash, String>,
    ) -> Result<(), String> {
        match tx_result {
            Ok(_) => Ok(()),
            Err(_) => Ok(()),
        }
    }
}

fn process_submission(
    input: AggregatorInput,
    validate_tx: bool,
) -> Result<Vec<AggregatorAction>, String> {
    let workflow = host::get_workflow().workflow;

    let submit_config = match workflow.submit {
        Submit::None => unreachable!(),
        Submit::Aggregator(aggregator_submit) => aggregator_submit.component.config,
    };

    if submit_config.is_empty() {
        return Err("Workflow submit component config is empty".to_string());
    }

    let mut actions = Vec::new();

    if validate_tx && !utils::is_valid_tx(input.trigger_action)? {
        return Ok(actions);
    }

    for (chain_key, service_handler_address) in submit_config {
        if wavs_types::ChainKey::validate(&chain_key).is_ok() {
            if host::get_evm_chain_config(&chain_key).is_some() {
                let address: alloy_primitives::Address = service_handler_address
                    .parse()
                    .map_err(|e| format!("Failed to parse address for '{chain_key}': {e}"))?;

                // Get gas price from Etherscan if configured
                // will fail the entire operation if API key is configured but fetching fails
                let gas_price = gas_oracle::get_gas_price()?;

                let submit_action = SubmitAction::Evm(EvmSubmitAction {
                    chain: chain_key.to_string(),
                    address: EvmAddress {
                        raw_bytes: address.to_vec(),
                    },
                    gas_price: gas_price.map(|x| x.into()),
                });

                actions.push(AggregatorAction::Submit(submit_action));
            } else if host::get_cosmos_chain_config(&chain_key).is_some() {
                todo!("Cosmos support coming soon...")
            } else {
                return Err(format!("Could not get chain config for chain {chain_key}"));
            }
        }
    }

    Ok(actions)
}

export!(Component);

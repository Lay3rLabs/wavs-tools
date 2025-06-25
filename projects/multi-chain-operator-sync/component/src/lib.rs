#[allow(warnings)]
mod bindings;

use alloy_primitives::LogData;
use alloy_sol_macro::sol;
use alloy_sol_types::SolEventInterface;
use bindings::{export, wavs::worker::layer_types::WasmResponse, Guest, TriggerAction};
use wavs_wasi_utils::decode_event_log_data;

use crate::bindings::wavs::worker::layer_types::TriggerData;

sol!(interface IMirrorUpdateTypes {
    error InvalidTriggerId(uint64 expectedTriggerId);

    /// @notice DataWithId is a struct containing a trigger ID and updated operator info
    struct UpdateWithId {
        uint64 triggerId;
        uint256 thresholdWeight;
        address[] operators;
        address[] signingKeys;
        uint256[] weights;
    }
});

sol!(
    #[sol(rpc)]
    ECDSAStakeRegistry,
    "../../../abi/eigenlayer-middleware/ECDSAStakeRegistry.sol/ECDSAStakeRegistry.json"
);

struct Component;

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        let evm_trigger_data = match action.data {
            TriggerData::EvmContractEvent(trigger_data_evm_contract_event) => {
                Ok(trigger_data_evm_contract_event)
            }
            _ => Err(format!(
                "Only evm contract event triggers are supported. Received {:?}",
                action
            )),
        }?;

        let maybe_register_event: anyhow::Result<ECDSAStakeRegistry::OperatorRegistered> =
            decode_event_log_data!(evm_trigger_data.log.clone());

        if let Ok(register_event) = maybe_register_event {
            let _ = register_event;
        } else {
            return Err(format!(
                "Could not decode the event {:?}",
                evm_trigger_data.log
            ));
        }

        Ok(None)
    }
}

export!(Component with_types_in bindings);

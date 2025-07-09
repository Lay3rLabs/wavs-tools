#[allow(warnings)]
mod bindings;
mod utils;

use crate::{
    bindings::wavs::worker::layer_types::{TriggerData, TriggerDataEvmContractEvent},
    IManagerUpdateTypes::UpdateWithId,
    WavsServiceManager::QuorumThresholdUpdated,
};
use alloy_sol_macro::sol;
use alloy_sol_types::SolValue;
use bindings::{export, wavs::worker::layer_types::WasmResponse, Guest, TriggerAction};
use wavs_wasi_utils::decode_event_log_data;
use wstd::runtime::block_on;

sol!(interface IManagerUpdateTypes {
    error InvalidTriggerId(uint64 expectedTriggerId);

    /// @notice DataWithId is a struct containing a trigger ID and updated operator info
    struct UpdateWithId {
        uint64 triggerId;
        uint256 numerator;
        uint256 denominator;
    }
});

sol!(
    #[sol(rpc)]
    WavsServiceManager,
    "../../../abi/wavs-middleware/WavsServiceManager.sol/WavsServiceManager.json"
);

struct Component;

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        match action.data {
            TriggerData::EvmContractEvent(TriggerDataEvmContractEvent {
                contract_address: _,
                chain_name: _,
                log,
                block_height,
            }) => block_on(async move {
                let QuorumThresholdUpdated {
                    numerator,
                    denominator,
                } = decode_event_log_data!(log.clone()).map_err(|x| x.to_string())?;

                let result = UpdateWithId {
                    triggerId: block_height,
                    numerator,
                    denominator,
                };

                return Ok(Some(WasmResponse {
                    payload: result.abi_encode(),
                    ordering: None,
                }));
            }),
            _ => Err(format!(
                "Component did not expect trigger action {:?}",
                action
            )),
        }
    }
}

export!(Component with_types_in bindings);

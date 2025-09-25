#[allow(warnings)]
#[rustfmt::skip]
mod bindings;
mod utils;

use crate::{
    bindings::{
        wavs::{operator::input::TriggerData, types::events::TriggerDataEvmContractEvent},
        WasmResponse,
    },
    IManagerUpdateTypes::UpdateWithId,
    IWavsServiceManager::QuorumThresholdUpdated,
};
use alloy_sol_macro::sol;
use alloy_sol_types::SolValue;
use bindings::{export, Guest, TriggerAction};
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
    IWavsServiceManager,
    "../../../abi/wavs-middleware/IWavsServiceManager.sol/IWavsServiceManager.json"
);

struct Component;

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        match action.data {
            TriggerData::EvmContractEvent(TriggerDataEvmContractEvent { log, .. }) => {
                block_on(async move {
                    let QuorumThresholdUpdated {
                        numerator,
                        denominator,
                    } = decode_event_log_data!(log.data).map_err(|x| x.to_string())?;

                    let result = UpdateWithId {
                        triggerId: log.block_number,
                        numerator,
                        denominator,
                    };

                    Ok(Some(WasmResponse {
                        payload: result.abi_encode(),
                        ordering: None,
                    }))
                })
            }
            _ => Err(format!(
                "Component did not expect trigger action {action:?}"
            )),
        }
    }
}

export!(Component with_types_in bindings);

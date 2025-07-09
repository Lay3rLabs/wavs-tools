#[allow(warnings)]
mod bindings;
mod utils;

use crate::bindings::wavs::worker::layer_types::{TriggerData, TriggerDataEvmContractEvent};
use alloy_sol_macro::sol;
use bindings::{export, wavs::worker::layer_types::WasmResponse, Guest, TriggerAction};

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
            TriggerData::EvmContractEvent(TriggerDataEvmContractEvent { .. }) => {
                todo!();
            }
            _ => Err(format!(
                "Component did not expect trigger action {:?}",
                action
            )),
        }
    }
}

export!(Component with_types_in bindings);

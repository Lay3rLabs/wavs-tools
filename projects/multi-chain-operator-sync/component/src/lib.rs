#[allow(warnings)]
mod bindings;

use alloy_sol_macro::sol;
use bindings::{export, wavs::worker::layer_types::WasmResponse, Guest, TriggerAction};

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

struct Component;

impl Guest for Component {
    fn run(_action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        Ok(None)
    }
}

export!(Component with_types_in bindings);

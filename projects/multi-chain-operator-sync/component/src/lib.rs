#[allow(warnings)]
mod bindings;

use alloy_sol_macro::sol;
use bindings::{export, wavs::worker::layer_types::WasmResponse, Guest, TriggerAction};

sol!("../contracts/src/Types.sol");

struct Component;

impl Guest for Component {
    fn run(_action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        Ok(None)
    }
}

export!(Component with_types_in bindings);

mod utils;

use crate::{
    wavs::{operator::input::TriggerData, types::events::TriggerDataEvmContractEvent},
    IManagerUpdateTypes::UpdateWithId,
    IWavsServiceManager::QuorumThresholdUpdated,
};
use alloy_sol_macro::sol;
use alloy_sol_types::SolValue;
use wavs_wasi_utils::decode_event_log_data;
use wstd::runtime::block_on;

wit_bindgen::generate!({
    path: "../../../wit-definitions/operator/wit",
    world: "wavs-world",
    generate_all,
    with: {
        "wasi:io/poll@0.2.0": wasip2::io::poll
    },
    features: ["tls"]
});

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
                        event_id_salt: None,
                    }))
                })
            }
            _ => Err(format!(
                "Component did not expect trigger action {action:?}"
            )),
        }
    }
}

export!(Component);

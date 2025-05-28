mod evm;
mod sync;

use wit_bindgen::generate;

generate!({
    world: "layer-trigger-world",
    path: "../../../../../WAVS/wit/wavs_worker.wit"
});

use layer_types::{LogLevel, TriggerAction, TriggerData, WasmResponse};
use wstd::runtime::block_on;

struct Component;

impl Guest for Component {
    fn run(trigger_action: TriggerAction) -> Result<Option<WasmResponse>, String> {
        host::log(LogLevel::Info, &format!("AVS Sync triggered: {:?}", trigger_action.config.service_id));

        block_on(async move {
            match trigger_action.data {
                TriggerData::Cron(cron_data) => {
                    host::log(LogLevel::Info, &format!("Running AVS sync at: {:?}", cron_data.trigger_time));
                    sync::run_avs_sync().await
                }
                TriggerData::BlockInterval(block_data) => {
                    host::log(LogLevel::Info, &format!("Running AVS sync at block: {}", block_data.block_height));
                    sync::run_avs_sync_for_block(block_data.block_height).await
                }
                _ => Err("AVS Sync only supports cron and block interval triggers".to_string())
            }
        })
    }
}

export_layer_trigger_world!(Component);

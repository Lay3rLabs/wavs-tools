use alloy_primitives::{keccak256, B256};
use anyhow::{anyhow, Result};

use crate::bindings::host::{get_cosmos_chain_config, get_evm_chain_config};
use crate::bindings::wavs::worker::input::{TriggerData, TriggerDataEvmContractEvent};
use crate::bindings::TriggerAction;
use crate::config::Config;
use crate::utils::{get_evm_block, vec_into_fixed_bytes};

/// Extracted trigger information
#[derive(Debug, Clone)]
pub struct TriggerInfo {
    pub unique_id: B256,
    pub drand_round: u64,
}

impl TriggerInfo {
    /// Extract trigger information from a TriggerAction
    pub async fn from_trigger_action(
        trigger_action: TriggerAction,
        config: &Config,
    ) -> Result<Self> {
        let (unique_id, timestamp) = Self::extract_id_and_timestamp(trigger_action).await?;
        let drand_round = Self::calculate_drand_round(timestamp, config)?;

        Ok(Self {
            unique_id,
            drand_round,
        })
    }

    async fn extract_id_and_timestamp(trigger_action: TriggerAction) -> Result<(B256, u64)> {
        match trigger_action.data {
            TriggerData::EvmContractEvent(TriggerDataEvmContractEvent { chain_name, log }) => {
                let timestamp = if let Some(timestamp) = log.block_timestamp {
                    timestamp
                } else {
                    let chain_config = get_evm_chain_config(&chain_name)
                        .ok_or(anyhow!("Chain config for {0} not found", chain_name))?;
                    let block = get_evm_block(chain_config, chain_name, log.block_number).await?;

                    block.header.timestamp
                };

                Ok((vec_into_fixed_bytes(log.tx_hash)?, timestamp))
            }
            TriggerData::Cron(cron) => {
                let timestamp = cron.trigger_time.nanos / 1_000_000_000;
                let id_data = "cron";
                let unique_id = keccak256(id_data.as_bytes());

                Ok((unique_id, timestamp))
            }
            TriggerData::BlockInterval(block) => {
                if let Some(chain_config) = get_evm_chain_config(&block.chain_name) {
                    let block =
                        get_evm_block(chain_config, block.chain_name, block.block_height).await?;

                    Ok((block.header.transactions_root, block.header.timestamp))
                } else if let Some(_chain_config) = get_cosmos_chain_config(&block.chain_name) {
                    unimplemented!()
                } else {
                    Err(anyhow!("Chain config for {0} not found", block.chain_name))
                }
            }
            TriggerData::CosmosContractEvent(_event) => {
                unimplemented!()
            }
            TriggerData::Raw(_raw_data) => {
                unimplemented!()
            }
        }
    }

    fn calculate_drand_round(timestamp: u64, config: &Config) -> Result<u64> {
        if timestamp < config.drand_genesis_time {
            return Ok(1);
        }

        let round = ((timestamp - config.drand_genesis_time) / config.drand_period) + 1;
        Ok(round)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_drand_round() {
        let config = Config::default();

        // Test before genesis - should return round 1
        let timestamp = 1000000000u64; // Way before genesis
        let round = TriggerInfo::calculate_drand_round(timestamp, &config).unwrap();
        assert_eq!(round, 1);

        // Test exact genesis time - should return round 1
        let timestamp = config.drand_genesis_time;
        let round = TriggerInfo::calculate_drand_round(timestamp, &config).unwrap();
        assert_eq!(round, 1);

        // Test one period after genesis - should return round 2
        let timestamp = config.drand_genesis_time + config.drand_period;
        let round = TriggerInfo::calculate_drand_round(timestamp, &config).unwrap();
        assert_eq!(round, 2);

        // Test two periods after genesis - should return round 3
        let timestamp = config.drand_genesis_time + (2 * config.drand_period);
        let round = TriggerInfo::calculate_drand_round(timestamp, &config).unwrap();
        assert_eq!(round, 3);

        // Test partial period - should still be round 2
        let timestamp = config.drand_genesis_time + config.drand_period + 15; // 15 seconds into round 2
        let round = TriggerInfo::calculate_drand_round(timestamp, &config).unwrap();
        assert_eq!(round, 2);

        // Test with different period for edge cases
        let mut custom_config = config.clone();
        custom_config.drand_period = 60; // 1 minute periods

        let timestamp = custom_config.drand_genesis_time + 120; // 2 minutes after genesis
        let round = TriggerInfo::calculate_drand_round(timestamp, &custom_config).unwrap();
        assert_eq!(round, 3); // Should be round 3 (0-60s = round 1, 60-120s = round 2, 120+ = round 3)
    }
}

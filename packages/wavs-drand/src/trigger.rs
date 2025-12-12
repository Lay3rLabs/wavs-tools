use alloy_primitives::{keccak256, B256, U256};
use alloy_provider::network::Ethereum;
use alloy_provider::Provider;
use alloy_rpc_types::BlockNumberOrTag;
use anyhow::{anyhow, Result};
use wavs_wasi_utils::decode_event_log_data;
use wavs_wasi_utils::evm::new_evm_provider;

use crate::config::Config;
use crate::host::{get_cosmos_chain_config, get_evm_chain_config};
use crate::wavs::operator::input::TriggerData;
use crate::wavs::types::events::{TriggerDataBlockInterval, TriggerDataEvmContractEvent};
use crate::{RandomnessRequested, TriggerAction};

/// Extracted trigger information
#[derive(Debug, Clone)]
pub struct TriggerInfo {
    pub trigger_id: U256,
    pub unique_id: B256,
    pub drand_round: u64,
}

impl TriggerInfo {
    /// Extract trigger information from a TriggerAction
    pub async fn from_trigger_action(
        trigger_action: TriggerAction,
        config: &Config,
    ) -> Result<Self> {
        let (trigger_id, unique_id, timestamp) = Self::extract_trigger_info(trigger_action).await?;
        let drand_round = Self::calculate_drand_round(timestamp, config)?;

        Ok(Self {
            trigger_id,
            unique_id,
            drand_round,
        })
    }

    async fn extract_trigger_info(trigger_action: TriggerAction) -> Result<(U256, B256, u64)> {
        match trigger_action.data {
            TriggerData::EvmContractEvent(TriggerDataEvmContractEvent { chain, log }) => {
                let timestamp = if let Some(timestamp) = log.block_timestamp {
                    timestamp
                } else {
                    let chain_config = get_evm_chain_config(&chain)
                        .ok_or(anyhow!("Chain config for {0} not found", chain))?;
                    let provider = new_evm_provider::<Ethereum>(
                        chain_config
                            .http_endpoint
                            .ok_or(anyhow!("Could not get http endpoint for {chain}"))?,
                    );
                    let block = provider
                        .get_block_by_hash(log.block_hash.as_slice().try_into()?)
                        .await?
                        .ok_or(anyhow!(
                            "Could not get block on {chain} for block hash {0:?}",
                            log.block_hash
                        ))?;

                    block.header.timestamp
                };

                // Extract trigger ID from event data (uint256 = 32 bytes)
                let RandomnessRequested { triggerId } = decode_event_log_data!(log.data.clone())?;

                Ok((triggerId, log.tx_hash.as_slice().try_into()?, timestamp))
            }
            TriggerData::Cron(cron) => {
                let timestamp = cron.trigger_time.nanos / 1_000_000_000;
                let id_data = "cron";
                let unique_id = keccak256(id_data.as_bytes());

                Ok((U256::ZERO, unique_id, timestamp))
            }
            TriggerData::BlockInterval(TriggerDataBlockInterval {
                chain,
                block_height,
            }) => {
                if let Some(chain_config) = get_evm_chain_config(&chain) {
                    let provider = new_evm_provider::<Ethereum>(
                        chain_config
                            .http_endpoint
                            .ok_or(anyhow!("Could not get http endpoint for {chain}"))?,
                    );
                    let block = provider
                        .get_block_by_number(BlockNumberOrTag::Number(block_height))
                        .await?
                        .ok_or(anyhow!(
                            "Could not get block on {chain} for block height {block_height}",
                        ))?;

                    Ok((
                        U256::ZERO,
                        block.header.transactions_root,
                        block.header.timestamp,
                    ))
                } else if let Some(_chain_config) = get_cosmos_chain_config(&chain) {
                    unimplemented!()
                } else {
                    Err(anyhow!("Chain config for {chain} not found"))
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

    // https://docs.drand.love/docs/specification/#randomness-generation-period
    fn calculate_drand_round(timestamp: u64, config: &Config) -> Result<u64> {
        if timestamp < config.drand_genesis_time {
            return Ok(0);
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

        // Test before genesis - should return round 0
        let timestamp = 1000000000u64; // Way before genesis
        let round = TriggerInfo::calculate_drand_round(timestamp, &config).unwrap();
        assert_eq!(round, 0);

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

use alloy_primitives::{keccak256, Address, B256};
use alloy_provider::network::Ethereum;
use alloy_provider::Provider;
use alloy_rpc_types::{BlockNumberOrTag, Filter, FilterBlockOption, FilterSet, Topic};
use anyhow::{anyhow, Result};
use wavs_wasi_utils::evm::new_evm_provider;

use crate::bindings::host::{get_cosmos_chain_config, get_evm_chain_config};
use crate::bindings::wavs::worker::input::TriggerData;
use crate::bindings::TriggerAction;
use crate::config::Config;

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
            TriggerData::EvmContractEvent(event) => {
                let chain_config = get_evm_chain_config(&event.chain_name)
                    .ok_or(anyhow!("Chain config for {0} not found", event.chain_name))?;
                let endpoint = chain_config
                    .http_endpoint
                    .ok_or(anyhow!("Http endpoint for {0} not found", event.chain_name))?;
                let provider = new_evm_provider::<Ethereum>(endpoint);

                // Construct filter for logs from a single block
                let mut topics_array: [Topic; 4] = [
                    Topic::default(),
                    Topic::default(),
                    Topic::default(),
                    Topic::default(),
                ];
                for (i, topic) in event.log.topics.into_iter().enumerate().take(4) {
                    let array: [u8; 32] = topic
                        .try_into()
                        .map_err(|_| anyhow::anyhow!("Topic is not 32 bytes"))?;
                    topics_array[i] = Topic::from(array);
                }
                let address: Address = event.contract_address.into();
                let filter = Filter {
                    address: FilterSet::from(address),
                    topics: topics_array,
                    block_option: FilterBlockOption::Range {
                        from_block: Some(BlockNumberOrTag::Number(event.block_height)),
                        to_block: Some(BlockNumberOrTag::Number(event.block_height)),
                    },
                };

                let logs = provider.get_logs(&filter).await?;

                for log in logs {
                    if let (Some(tx_hash), Some(timestamp)) =
                        (log.transaction_hash, log.block_timestamp)
                    {
                        return Ok((tx_hash, timestamp));
                    }
                }

                Err(anyhow!(
                    "No log found with both transaction_hash and block_timestamp"
                ))
            }
            TriggerData::Cron(cron) => {
                let timestamp = cron.trigger_time.nanos / 1_000_000_000;
                let id_data = "cron";
                let unique_id = keccak256(id_data.as_bytes());

                Ok((unique_id, timestamp))
            }
            TriggerData::BlockInterval(block) => {
                if let Some(chain_config) = get_evm_chain_config(&block.chain_name) {
                    let endpoint = chain_config
                        .http_endpoint
                        .ok_or(anyhow!("Http endpoint for {0} not found", block.chain_name))?;
                    let provider = new_evm_provider::<Ethereum>(endpoint);

                    let block = provider
                        .get_block_by_number(alloy_rpc_types::BlockNumberOrTag::Number(
                            block.block_height,
                        ))
                        .await?
                        .ok_or(anyhow!(
                            "Block not found at height {0} for chain {1}",
                            block.block_height,
                            block.chain_name
                        ))?;

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

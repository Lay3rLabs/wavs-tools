use crate::bindings;

/// Configuration for the VRF service
#[derive(Debug, Clone)]
pub struct Config {
    pub drand_url: String,
    pub drand_chain_hash: String,
    pub drand_genesis_time: u64,
    pub drand_period: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            drand_url: "https://api.drand.sh".to_string(),
            drand_chain_hash: "8990e7a9aaed2ffed73dbd7092123d6f289930540d7651336225dc172e51b2ce"
                .to_string(),
            drand_genesis_time: 1595431050, // Drand mainnet genesis time
            drand_period: 30,               // 30 seconds per round
        }
    }
}

impl Config {
    /// Load configuration from host environment variables
    pub fn from_host() -> Self {
        let defaults = Self::default();

        let drand_url = bindings::host::config_var("DRAND_URL").unwrap_or(defaults.drand_url);

        let drand_chain_hash =
            bindings::host::config_var("DRAND_CHAIN_HASH").unwrap_or(defaults.drand_chain_hash);

        let drand_genesis_time = bindings::host::config_var("DRAND_GENESIS_TIME")
            .and_then(|s| s.parse().ok())
            .unwrap_or(defaults.drand_genesis_time);

        let drand_period = bindings::host::config_var("DRAND_PERIOD")
            .and_then(|s| s.parse().ok())
            .unwrap_or(defaults.drand_period);

        Self {
            drand_url,
            drand_chain_hash,
            drand_genesis_time,
            drand_period,
        }
    }
}

use alloy_primitives::{keccak256, B256, U256};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/// Core VRF functionality - combines multiple entropy sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomDerivation {
    pub seed: B256,
    pub round: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomDerivationResult {
    pub round: u64,
    pub randomness: B256,
    pub seed: B256,
}

impl RandomDerivation {
    /// Create VRF from a single seed
    pub fn new(seed: B256, round: u64) -> Self {
        Self { seed, round }
    }

    /// Create VRF by combining multiple entropy sources
    pub fn from_sources(sources: &[&[u8]], round: u64) -> Self {
        let mut combined = Vec::new();
        for source in sources {
            combined.extend_from_slice(source);
        }
        let seed = keccak256(&combined);
        Self::new(seed, round)
    }

    /// Generate the final VRF output
    pub fn generate(&self) -> RandomDerivationResult {
        let mut data = self.seed.as_slice().to_vec();
        data.extend_from_slice(&self.round.to_be_bytes());
        let randomness = keccak256(&data);

        RandomDerivationResult {
            round: self.round,
            randomness,
            seed: self.seed,
        }
    }
}

#[allow(unused)]
impl RandomDerivationResult {
    /// Convert to U256 for mathematical operations
    pub fn as_u256(&self) -> U256 {
        U256::from_be_bytes(self.randomness.0)
    }

    /// Generate random number in range [min, max)
    pub fn random_in_range(&self, min: u64, max: u64) -> Result<u64> {
        if min >= max {
            return Err(anyhow!("Invalid range: min must be less than max"));
        }

        let range = max - min;
        let result = (self.as_u256() % U256::from(range)).as_limbs()[0] + min;
        Ok(result)
    }

    /// Generate random boolean
    pub fn random_bool(&self) -> bool {
        self.randomness.0[31] & 1 == 1
    }

    /// Select random item from slice
    pub fn select<'a, T>(&self, items: &'a [T]) -> Result<&'a T> {
        if items.is_empty() {
            return Err(anyhow!("Cannot select from empty list"));
        }

        let index = self.random_in_range(0, items.len() as u64)? as usize;
        Ok(&items[index])
    }

    /// Generate deterministic random bytes
    pub fn random_bytes(&self, length: usize) -> Vec<u8> {
        let mut result = Vec::new();
        let mut counter = 0u64;

        while result.len() < length {
            let mut data = self.randomness.as_slice().to_vec();
            data.extend_from_slice(&counter.to_be_bytes());
            let hash = keccak256(&data);

            let remaining = length - result.len();
            let to_take = remaining.min(32);
            result.extend_from_slice(&hash.as_slice()[..to_take]);

            counter += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vrf_basic() {
        let seed = B256::from([42u8; 32]);
        let vrf = RandomDerivation::new(seed, 1);
        let result = vrf.generate();

        assert_eq!(result.round, 1);
        assert_eq!(result.seed, seed);
        assert_ne!(result.randomness, B256::ZERO);
    }

    #[test]
    fn test_vrf_from_sources() {
        let sources = [
            b"event_data".as_slice(),
            b"timestamp".as_slice(),
            b"drand".as_slice(),
        ];
        let vrf = RandomDerivation::from_sources(&sources, 1);
        let result = vrf.generate();

        assert_eq!(result.round, 1);
        assert_ne!(result.randomness, B256::ZERO);
    }

    #[test]
    fn test_random_operations() {
        let seed = B256::from([42u8; 32]);
        let vrf = RandomDerivation::new(seed, 1);
        let result = vrf.generate();

        // Test range
        let num = result.random_in_range(1, 100).unwrap();
        assert!((1..100).contains(&num));

        // Test bool
        let _bool_val = result.random_bool();

        // Test select
        let items = vec!["a", "b", "c"];
        let selected = result.select(&items).unwrap();
        assert!(items.contains(selected));

        // Test bytes
        let bytes = result.random_bytes(16);
        assert_eq!(bytes.len(), 16);
    }

    #[test]
    fn test_deterministic() {
        let seed = B256::from([42u8; 32]);

        let vrf1 = RandomDerivation::new(seed, 1);
        let result1 = vrf1.generate();

        let vrf2 = RandomDerivation::new(seed, 1);
        let result2 = vrf2.generate();

        assert_eq!(result1.randomness, result2.randomness);
    }
}

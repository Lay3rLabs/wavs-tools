use crate::wavs::types::chain::EvmAddress;

impl From<EvmAddress> for alloy_primitives::Address {
    fn from(addr: EvmAddress) -> Self {
        alloy_primitives::Address::from_slice(&addr.raw_bytes)
    }
}

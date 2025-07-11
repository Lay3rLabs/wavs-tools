use crate::bindings::wavs::worker::layer_types::EvmAddress;

impl From<EvmAddress> for alloy_primitives::Address {
    fn from(addr: EvmAddress) -> Self {
        alloy_primitives::Address::from_slice(&addr.raw_bytes)
    }
}

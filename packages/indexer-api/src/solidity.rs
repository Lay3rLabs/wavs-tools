#![allow(clippy::too_many_arguments)]

use alloy_sol_macro::sol;

// First import the WAVS service handler types
sol!(
    #![sol(extra_derives(serde::Serialize, serde::Deserialize))]
    "src/interfaces/IWavsServiceHandler.sol"
);

// WAVS Indexer contract types - import from the interface
sol!(
    #![sol(extra_derives(serde::Serialize, serde::Deserialize))]
    #![sol(rpc)]
    "src/interfaces/IWavsIndexer.sol"
);

// Re-export the types for convenience
pub use IWavsIndexer::*;
pub use IWavsServiceHandler::*;

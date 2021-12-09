pub use eth2::types::{AggregateSignature, BeaconBlockHeader, BitVector, EthSpec, MainnetEthSpec, SyncCommittee};
pub use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightClientUpdate<T: EthSpec> {
    pub header: BeaconBlockHeader,
    pub next_sync_committee: Arc<SyncCommittee<T>>,
    // pub next_sync_committee_branch: FixedVector<Hash256, T::SIZE_OF_VECTOR,
    // pub finality_header: Option<BeaconBlockHeader>,
    // pub finality_branch: Option<FixedVector<Hash256, T::SIZE_OF_VECTOR>>,
    pub sync_committee_bits: BitVector<T::SyncCommitteeSize>,
    pub sync_committee_signature: AggregateSignature,
    pub fork_version: [u8; 4]
}
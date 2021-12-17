pub use bls::PublicKeyBytes;
pub use eth2::types::{BlockHeaderData, BlockId, StateId, ChainSpec, EthSpec, GenericResponse, MainnetEthSpec, SignedBeaconBlock, SyncAggregate, ValidatorId};
pub use eth2::{BeaconNodeHttpClient, Error, Timeouts};
pub use sensitive_url::SensitiveUrl;
pub use crate::light_client_types::LightClientUpdate;
pub use reqwest::{Url};
use std::sync::Arc;
use std::time::Duration;

pub struct Builder {
    api_client: BeaconNodeHttpClient,
    settings: Settings
}

pub struct Settings {
    slots_per_epoch: u64
}

impl Builder {
    pub fn new(node_url: &str) -> Self {
        let api_timeouts = Timeouts::set_all(Duration::new(5, 0));
        let surl = SensitiveUrl::parse(node_url).unwrap();
        let api_client = BeaconNodeHttpClient::new(surl, api_timeouts);
        let settings = Settings {
            slots_per_epoch: 32
        };
        
        Self {
            api_client,
            settings
        }
    }

    pub async fn run<T: EthSpec>(&self, chain_spec: &ChainSpec) {
        let (fork_name, signed_beacon_block) = self.api_client.get_beacon_blocks::<T>(BlockId::Finalized).await.unwrap().map(|d| (d.version, d.data)).unwrap();
        let current_slot = signed_beacon_block.slot();
        let (beacon_block, _block_signature) = signed_beacon_block.clone().deconstruct();
        let current_epoch = beacon_block.epoch();
        let next_epoch = beacon_block.epoch() + 1;
        let current_sync_committee_period = current_epoch.sync_committee_period(chain_spec);
        let next_sync_committee_period = current_sync_committee_period.unwrap() + 1;
        let current_sync_committee_aggregate = beacon_block.body().sync_aggregate().unwrap();

        // this API returns indices, not pubkeys
        // so either (1) fiture out how to get pubkeys, or (2) get/use state for sync committee data
        // let current_sync_committee_ids = self.api_client.get_beacon_states_sync_committees(StateId::Slot(current_slot), None).await.unwrap().data.validators;
        // let next_sync_committee_ids = self.api_client.get_beacon_states_sync_committees(StateId::Slot(current_slot), None).await.unwrap().data.validators;
        // let t = ValidatorId::Index(current_sync_committee_ids[0]);
        // let t2 = ValidatorId::Index(next_sync_committee_ids[0]);
        // let current_sync_committee_pubkeys = self.api_client.get_beacon_states_validator_id(StateId::Slot(current_slot), &t).await;
        // let next_sync_committee_pubkeys = self.api_client.get_beacon_states_validator_id(StateId::Slot(next_epoch.start_slot(self.settings.slots_per_epoch)), &t2).await;

        // let next_sync_committee = SyncAggregate {
        //     sync_committee_bits: ,
        //     sync_committee_signature:
        // };

        let state = self.api_client.get_debug_beacon_states::<T>(StateId::Slot(current_slot)).await.unwrap().map(|d| d.data).unwrap();
        let current_sync_committee = state.current_sync_committee().unwrap().clone();
        let next_sync_committee = state.next_sync_committee().unwrap().clone();
        let finality_checkpoints = self.api_client.get_beacon_states_finality_checkpoints(StateId::Slot(current_slot)).await.unwrap().map(|d| d.data).unwrap();
        let finalized_epoch = finality_checkpoints.finalized.epoch;
        let finalized_epoch_start_slot = finalized_epoch.start_slot(self.settings.slots_per_epoch);
        let (_finality_fork_name, finality_signed_beacon_block) = self.api_client.get_beacon_blocks::<T>(BlockId::Slot(finalized_epoch_start_slot)).await.unwrap().map(|d| (d.version, d.data)).unwrap();
        let (finality_beacon_block, _finality_block_signature) = finality_signed_beacon_block.clone().deconstruct();

        let header = beacon_block.block_header();
        let finality_header = finality_beacon_block.block_header();
        let sync_committee_bits = current_sync_committee_aggregate.clone().sync_committee_bits;
        let sync_committee_signature = current_sync_committee_aggregate.clone().sync_committee_signature;
        let fork_version = chain_spec.fork_version_for_name(fork_name.unwrap());

        let light_client_update: LightClientUpdate<T> = LightClientUpdate {
            header,
            next_sync_committee,
            // next_sync_committee_branch,
            finality_header: Some(finality_header),
            // finality_branch,
            sync_committee_bits,
            sync_committee_signature,
            fork_version
        };
        
        println!("{:#?}", light_client_update);
        println!("{:#?}", finality_checkpoints);
        println!("{:#?}", current_epoch);

    }
}
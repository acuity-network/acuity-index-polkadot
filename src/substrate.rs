use hybrid_indexer::{shared::RuntimeIndexer, substrate::Indexer};

use crate::polkadot::runtime_types::{
    frame_system::pallet::Event as SystemEvent, pallet_indices::pallet::Event as IndicesEvent,
    pallet_preimage::pallet::Event as PreimageEvent,
    pallet_scheduler::pallet::Event as SchedulerEvent,
};

pub fn system_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: SystemEvent,
) {
    match event {
        SystemEvent::ExtrinsicSuccess { .. } => {}
        SystemEvent::ExtrinsicFailed { .. } => {}
        SystemEvent::CodeUpdated {} => {}
        SystemEvent::NewAccount { account } => {
            indexer.index_event_account_id(account, block_number, event_index);
        }
        SystemEvent::KilledAccount { account } => {
            indexer.index_event_account_id(account, block_number, event_index);
        }
        SystemEvent::Remarked { sender, .. } => {
            indexer.index_event_account_id(sender, block_number, event_index);
        }
    }
}

pub fn scheduler_index_event<R: RuntimeIndexer>(
    _indexer: &Indexer<R>,
    _block_number: u32,
    _event_index: u32,
    event: SchedulerEvent,
) {
    match event {
        SchedulerEvent::Scheduled { .. } => {}
        SchedulerEvent::Canceled { .. } => {}
        SchedulerEvent::Dispatched { .. } => {}
        SchedulerEvent::CallUnavailable { .. } => {}
        SchedulerEvent::PeriodicFailed { .. } => {}
        SchedulerEvent::PermanentlyOverweight { .. } => {}
    }
}

pub fn preimage_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: PreimageEvent,
) {
    match event {
        PreimageEvent::Noted { hash } => {
            indexer.index_event_preimage_hash(hash.into(), block_number, event_index);
        }
        PreimageEvent::Requested { hash } => {
            indexer.index_event_preimage_hash(hash.into(), block_number, event_index);
        }
        PreimageEvent::Cleared { hash } => {
            indexer.index_event_preimage_hash(hash.into(), block_number, event_index);
        }
    }
}

pub fn indices_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: IndicesEvent,
) {
    match event {
        IndicesEvent::IndexAssigned { who, index } => {
            indexer.index_event_account_id(who, block_number, event_index);
            indexer.index_event_account_index(index, block_number, event_index);
        }
        IndicesEvent::IndexFreed { index } => {
            indexer.index_event_account_index(index, block_number, event_index);
        }
        IndicesEvent::IndexFrozen { index, who } => {
            indexer.index_event_account_index(index, block_number, event_index);
            indexer.index_event_account_id(who, block_number, event_index);
        }
    }
}

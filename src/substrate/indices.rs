use crate::polkadot::runtime_types::pallet_indices::pallet::Event;
use hybrid_indexer::{shared::RuntimeIndexer, substrate::Indexer};

pub fn indices_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: Event,
) {
    match event {
        Event::IndexAssigned { who, index } => {
            indexer.index_event_account_id(who, block_number, event_index);
            indexer.index_event_account_index(index, block_number, event_index);
        }
        Event::IndexFreed { index } => {
            indexer.index_event_account_index(index, block_number, event_index);
        }
        Event::IndexFrozen { index, who } => {
            indexer.index_event_account_index(index, block_number, event_index);
            indexer.index_event_account_id(who, block_number, event_index);
        }
    }
}

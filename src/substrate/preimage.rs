use crate::polkadot::runtime_types::pallet_preimage::pallet::Event;
use hybrid_indexer::{shared::RuntimeIndexer, substrate::Indexer};

pub fn preimage_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: Event,
) {
    match event {
        Event::Noted { hash } => {
            indexer.index_event_preimage_hash(hash.into(), block_number, event_index);
        }
        Event::Requested { hash } => {
            indexer.index_event_preimage_hash(hash.into(), block_number, event_index);
        }
        Event::Cleared { hash } => {
            indexer.index_event_preimage_hash(hash.into(), block_number, event_index);
        }
    }
}

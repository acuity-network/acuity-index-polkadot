use crate::polkadot::runtime_types::pallet_scheduler::pallet::Event;
use hybrid_indexer::{shared::RuntimeIndexer, substrate::Indexer};

pub fn scheduler_index_event<R: RuntimeIndexer>(
    _indexer: &Indexer<R>,
    _block_number: u32,
    _event_index: u32,
    event: Event,
) {
    match event {
        Event::Scheduled { .. } => {}
        Event::Canceled { .. } => {}
        Event::Dispatched { .. } => {}
        Event::CallUnavailable { .. } => {}
        Event::PeriodicFailed { .. } => {}
        Event::PermanentlyOverweight { .. } => {}
    }
}

use crate::polkadot::runtime_types::frame_system::pallet::Event;
use hybrid_indexer::{shared::RuntimeIndexer, substrate::Indexer};

pub fn system_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: Event,
) {
    match event {
        Event::ExtrinsicSuccess { .. } => {}
        Event::ExtrinsicFailed { .. } => {}
        Event::CodeUpdated {} => {}
        Event::NewAccount { account } => {
            indexer.index_event_account_id(account, block_number, event_index);
        }
        Event::KilledAccount { account } => {
            indexer.index_event_account_id(account, block_number, event_index);
        }
        Event::Remarked { sender, .. } => {
            indexer.index_event_account_id(sender, block_number, event_index);
        }
    }
}

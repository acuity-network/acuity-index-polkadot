use hybrid_indexer::{shared::RuntimeIndexer, substrate::Indexer};

use crate::polkadot::runtime_types::pallet_elections_phragmen::pallet::Event as ElectionsPhragmenEvent;

pub fn elections_phragmen_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: ElectionsPhragmenEvent,
) {
    match event {
        ElectionsPhragmenEvent::NewTerm { new_members } => {
            for member in &new_members {
                indexer.index_event_account_id(member.0.clone(), block_number, event_index);
            }
        }
        ElectionsPhragmenEvent::EmptyTerm => {}
        ElectionsPhragmenEvent::ElectionError => {}
        ElectionsPhragmenEvent::MemberKicked { member } => {
            indexer.index_event_account_id(member, block_number, event_index);
        }
        ElectionsPhragmenEvent::Renounced { candidate } => {
            indexer.index_event_account_id(candidate, block_number, event_index);
        }
        ElectionsPhragmenEvent::CandidateSlashed { candidate, .. } => {
            indexer.index_event_account_id(candidate, block_number, event_index);
        }
        ElectionsPhragmenEvent::SeatHolderSlashed { seat_holder, .. } => {
            indexer.index_event_account_id(seat_holder, block_number, event_index);
        }
    }
}

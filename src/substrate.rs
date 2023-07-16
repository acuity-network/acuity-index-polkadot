use hybrid_indexer::{shared::RuntimeIndexer, substrate::Indexer};

use crate::polkadot::runtime_types::{
    pallet_collective::pallet::{Event as CollectiveEvent, Event2 as CollectiveEvent2},
    pallet_elections_phragmen::pallet::Event as ElectionsPhragmenEvent,
};

pub fn collective_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: CollectiveEvent,
) {
    match event {
        CollectiveEvent::Proposed {
            account,
            proposal_index,
            proposal_hash,
            ..
        } => {
            indexer.index_event_account_id(account, block_number, event_index);
            indexer.index_event_proposal_index(proposal_index, block_number, event_index);
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent::Voted {
            account,
            proposal_hash,
            ..
        } => {
            indexer.index_event_account_id(account, block_number, event_index);
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent::Approved { proposal_hash } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent::Disapproved { proposal_hash } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent::Executed { proposal_hash, .. } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent::MemberExecuted { proposal_hash, .. } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent::Closed { proposal_hash, .. } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
    }
}

pub fn collective2_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: CollectiveEvent2,
) {
    match event {
        CollectiveEvent2::Proposed {
            account,
            proposal_index,
            proposal_hash,
            ..
        } => {
            indexer.index_event_account_id(account, block_number, event_index);
            indexer.index_event_proposal_index(proposal_index, block_number, event_index);
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent2::Voted {
            account,
            proposal_hash,
            ..
        } => {
            indexer.index_event_account_id(account, block_number, event_index);
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent2::Approved { proposal_hash } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent2::Disapproved { proposal_hash } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent2::Executed { proposal_hash, .. } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent2::MemberExecuted { proposal_hash, .. } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent2::Closed { proposal_hash, .. } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
    }
}

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

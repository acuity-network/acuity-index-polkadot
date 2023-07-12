use hybrid_indexer::{shared::RuntimeIndexer, substrate::Indexer};

use crate::polkadot::runtime_types::{
    frame_system::pallet::Event as SystemEvent,
    pallet_balances::pallet::Event as BalancesEvent,
    pallet_collective::pallet::{Event as CollectiveEvent, Event2 as CollectiveEvent2},
    pallet_democracy::pallet::Event as DemocracyEvent,
    pallet_indices::pallet::Event as IndicesEvent,
    pallet_preimage::pallet::Event as PreimageEvent,
    pallet_scheduler::pallet::Event as SchedulerEvent,
    pallet_session::pallet::Event as SessionEvent,
    pallet_staking::pallet::pallet::Event as StakingEvent,
    pallet_transaction_payment::pallet::Event as TransactionPaymentEvent,
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

pub fn balances_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: BalancesEvent,
) {
    match event {
        BalancesEvent::Endowed { account, .. } => {
            indexer.index_event_account_id(account, block_number, event_index);
        }
        BalancesEvent::DustLost { account, .. } => {
            indexer.index_event_account_id(account, block_number, event_index);
        }
        BalancesEvent::Transfer { from, to, .. } => {
            indexer.index_event_account_id(from, block_number, event_index);
            indexer.index_event_account_id(to, block_number, event_index);
        }
        BalancesEvent::BalanceSet { who, .. } => {
            indexer.index_event_account_id(who, block_number, event_index);
        }
        BalancesEvent::Reserved { who, .. } => {
            indexer.index_event_account_id(who, block_number, event_index);
        }
        BalancesEvent::Unreserved { who, .. } => {
            indexer.index_event_account_id(who, block_number, event_index);
        }
        BalancesEvent::ReserveRepatriated { from, to, .. } => {
            indexer.index_event_account_id(from, block_number, event_index);
            indexer.index_event_account_id(to, block_number, event_index);
        }
        BalancesEvent::Deposit { who, .. } => {
            indexer.index_event_account_id(who, block_number, event_index);
        }
        BalancesEvent::Withdraw { who, .. } => {
            indexer.index_event_account_id(who, block_number, event_index);
        }
        BalancesEvent::Slashed { who, .. } => {
            indexer.index_event_account_id(who, block_number, event_index);
        }
        BalancesEvent::Minted { who, .. } => {
            indexer.index_event_account_id(who, block_number, event_index);
        }
        BalancesEvent::Burned { who, .. } => {
            indexer.index_event_account_id(who, block_number, event_index);
        }
        BalancesEvent::Suspended { who, .. } => {
            indexer.index_event_account_id(who, block_number, event_index);
        }
        BalancesEvent::Restored { who, .. } => {
            indexer.index_event_account_id(who, block_number, event_index);
        }
        BalancesEvent::Upgraded { who } => {
            indexer.index_event_account_id(who, block_number, event_index);
        }
        BalancesEvent::Issued { .. } => {}
        BalancesEvent::Rescinded { .. } => {}
        BalancesEvent::Locked { who, .. } => {
            indexer.index_event_account_id(who, block_number, event_index);
        }
        BalancesEvent::Unlocked { who, .. } => {
            indexer.index_event_account_id(who, block_number, event_index);
        }
        BalancesEvent::Frozen { who, .. } => {
            indexer.index_event_account_id(who, block_number, event_index);
        }
        BalancesEvent::Thawed { who, .. } => {
            indexer.index_event_account_id(who, block_number, event_index);
        }
    }
}

pub fn transaction_payment_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: TransactionPaymentEvent,
) {
    match event {
        TransactionPaymentEvent::TransactionFeePaid { who, .. } => {
            indexer.index_event_account_id(who, block_number, event_index);
        }
    }
}

pub fn staking_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: StakingEvent,
) {
    match event {
        StakingEvent::EraPaid { era_index, .. } => {
            indexer.index_event_era_index(era_index, block_number, event_index);
        }
        StakingEvent::Rewarded { stash, .. } => {
            indexer.index_event_account_id(stash, block_number, event_index);
        }
        StakingEvent::Slashed { staker, .. } => {
            indexer.index_event_account_id(staker, block_number, event_index);
        }
        StakingEvent::SlashReported {
            validator,
            fraction: _,
            slash_era,
        } => {
            indexer.index_event_account_id(validator, block_number, event_index);
            indexer.index_event_era_index(slash_era, block_number, event_index);
        }
        StakingEvent::OldSlashingReportDiscarded { session_index } => {
            indexer.index_event_session_index(session_index, block_number, event_index);
        }
        StakingEvent::StakersElected => {}
        StakingEvent::Bonded { stash, .. } => {
            indexer.index_event_account_id(stash, block_number, event_index);
        }
        StakingEvent::Unbonded { stash, .. } => {
            indexer.index_event_account_id(stash, block_number, event_index);
        }
        StakingEvent::Withdrawn { stash, .. } => {
            indexer.index_event_account_id(stash, block_number, event_index);
        }
        StakingEvent::Kicked { nominator, stash } => {
            indexer.index_event_account_id(nominator, block_number, event_index);
            indexer.index_event_account_id(stash, block_number, event_index);
        }
        StakingEvent::StakingElectionFailed => {}
        StakingEvent::Chilled { stash } => {
            indexer.index_event_account_id(stash, block_number, event_index);
        }
        StakingEvent::PayoutStarted {
            era_index,
            validator_stash,
        } => {
            indexer.index_event_era_index(era_index, block_number, event_index);
            indexer.index_event_account_id(validator_stash, block_number, event_index);
        }
        StakingEvent::ValidatorPrefsSet { stash, .. } => {
            indexer.index_event_account_id(stash, block_number, event_index);
        }
        StakingEvent::ForceEra { .. } => {}
    }
}

pub fn session_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: SessionEvent,
) {
    match event {
        SessionEvent::NewSession { session_index } => {
            indexer.index_event_session_index(session_index, block_number, event_index);
        }
    }
}

pub fn democracy_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: DemocracyEvent,
) {
    match event {
        DemocracyEvent::Proposed { proposal_index, .. } => {
            indexer.index_event_proposal_index(proposal_index, block_number, event_index);
        }
        DemocracyEvent::Tabled { proposal_index, .. } => {
            indexer.index_event_proposal_index(proposal_index, block_number, event_index);
        }
        DemocracyEvent::ExternalTabled => {}
        DemocracyEvent::Started { ref_index, .. } => {
            indexer.index_event_ref_index(ref_index, block_number, event_index);
        }
        DemocracyEvent::Passed { ref_index } => {
            indexer.index_event_ref_index(ref_index, block_number, event_index);
        }
        DemocracyEvent::NotPassed { ref_index } => {
            indexer.index_event_ref_index(ref_index, block_number, event_index);
        }
        DemocracyEvent::Cancelled { ref_index } => {
            indexer.index_event_ref_index(ref_index, block_number, event_index);
        }
        DemocracyEvent::Delegated { who, target } => {
            indexer.index_event_account_id(who, block_number, event_index);
            indexer.index_event_account_id(target, block_number, event_index);
        }
        DemocracyEvent::Undelegated { account } => {
            indexer.index_event_account_id(account, block_number, event_index);
        }
        DemocracyEvent::Vetoed {
            who, proposal_hash, ..
        } => {
            indexer.index_event_account_id(who, block_number, event_index);
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        DemocracyEvent::Blacklisted { proposal_hash } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        DemocracyEvent::Voted {
            voter, ref_index, ..
        } => {
            indexer.index_event_account_id(voter, block_number, event_index);
            indexer.index_event_ref_index(ref_index, block_number, event_index);
        }
        DemocracyEvent::Seconded {
            seconder,
            prop_index,
        } => {
            indexer.index_event_account_id(seconder, block_number, event_index);
            indexer.index_event_proposal_index(prop_index, block_number, event_index);
        }
        DemocracyEvent::ProposalCanceled { prop_index } => {
            indexer.index_event_proposal_index(prop_index, block_number, event_index);
        }
        DemocracyEvent::MetadataSet { owner: _, hash } => {
            indexer.index_event_preimage_hash(hash.into(), block_number, event_index);
        }
        DemocracyEvent::MetadataCleared { owner: _, hash } => {
            indexer.index_event_preimage_hash(hash.into(), block_number, event_index);
        }
        DemocracyEvent::MetadataTransferred {
            prev_owner: _,
            owner: _,
            hash,
        } => {
            indexer.index_event_preimage_hash(hash.into(), block_number, event_index);
        }
    }
}

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

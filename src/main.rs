#![feature(more_qualified_paths)]
#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}
use polkadot::Event;

use crate::polkadot::runtime_types::{
    frame_system::pallet::Event as SystemEvent,
    pallet_bags_list::pallet::Event as BagsListEvent,
    pallet_balances::pallet::Event as BalancesEvent,
    pallet_bounties::pallet::Event as BountiesEvent,
    pallet_child_bounties::pallet::Event as ChildBountiesEvent,
    pallet_collective::pallet::{Event as CollectiveEvent, Event2 as CollectiveEvent2},
    pallet_democracy::pallet::Event as DemocracyEvent,
    pallet_election_provider_multi_phase::pallet::Event as ElectionProviderMultiPhaseEvent,
    pallet_elections_phragmen::pallet::Event as ElectionsPhragmenEvent,
    pallet_fast_unstake::pallet::Event as FastUnstakeEvent,
    pallet_identity::pallet::Event as IdentityEvent,
    pallet_indices::pallet::Event as IndicesEvent,
    pallet_multisig::pallet::Event as MultisigEvent,
    pallet_nomination_pools::pallet::Event as NominationPoolsEvent,
    pallet_preimage::pallet::Event as PreimageEvent,
    pallet_proxy::pallet::Event as ProxyEvent,
    pallet_session::pallet::Event as SessionEvent,
    pallet_staking::pallet::pallet::Event as StakingEvent,
    pallet_tips::pallet::Event as TipsEvent,
    pallet_transaction_payment::pallet::Event as TransactionPaymentEvent,
    pallet_treasury::pallet::Event as TreasuryEvent,
    pallet_vesting::pallet::Event as VestingEvent,
    polkadot_runtime_common::{
        claims::pallet::Event as ClaimsEvent, paras_registrar::pallet::Event as ParasRegistrarEvent,
    },
    polkadot_runtime_parachains::{
        disputes::pallet::Event as DisputesEvent, hrmp::pallet::Event as HrmpEvent,
        paras::pallet::Event as ParasEvent,
    },
};

struct PolkadotIndexer;

use hybrid_indexer::*;

mod pallets;
use pallets::*;

impl hybrid_indexer::shared::RuntimeIndexer for PolkadotIndexer {
    type RuntimeConfig = subxt::PolkadotConfig;

    fn process_event(
        indexer: &hybrid_indexer::substrate::Indexer<Self>,
        block_number: u32,
        event_index: u32,
        event: subxt::events::EventDetails<Self::RuntimeConfig>,
    ) -> Result<(), subxt::Error> {
        match event.as_root_event::<Event>()? {
            // Substrate pallets.
            Event::System(event) => {
                index_system_event![SystemEvent, event, indexer, block_number, event_index]
            }
            Event::Preimage(event) => {
                index_preimage_event![PreimageEvent, event, indexer, block_number, event_index]
            }
            Event::Indices(event) => {
                index_indices_event![IndicesEvent, event, indexer, block_number, event_index]
            }
            Event::Balances(event) => {
                index_balances_event![BalancesEvent, event, indexer, block_number, event_index]
            }
            Event::TransactionPayment(event) => {
                index_transaction_payment_event![
                    TransactionPaymentEvent,
                    event,
                    indexer,
                    block_number,
                    event_index
                ]
            }
            Event::Staking(event) => {
                index_staking_event![StakingEvent, event, indexer, block_number, event_index]
            }
            Event::Session(event) => {
                index_session_event![SessionEvent, event, indexer, block_number, event_index]
            }
            Event::Democracy(event) => {
                index_democracy_event![DemocracyEvent, event, indexer, block_number, event_index]
            }
            Event::Council(event) => {
                index_collective_event![CollectiveEvent, event, indexer, block_number, event_index]
            }
            Event::TechnicalCommittee(event) => {
                index_collective_event![CollectiveEvent2, event, indexer, block_number, event_index]
            }
            Event::PhragmenElection(event) => {
                index_elections_phragmen_event![
                    ElectionsPhragmenEvent,
                    event,
                    indexer,
                    block_number,
                    event_index
                ]
            }
            Event::Treasury(event) => {
                index_treasury_event![TreasuryEvent, event, indexer, block_number, event_index]
            }
            Event::Vesting(event) => {
                index_vesting_event![VestingEvent, event, indexer, block_number, event_index]
            }
            Event::Identity(event) => {
                index_identity_event![IdentityEvent, event, indexer, block_number, event_index]
            }
            Event::Proxy(event) => {
                index_proxy_event![ProxyEvent, event, indexer, block_number, event_index]
            }
            Event::Multisig(event) => {
                index_multisig_event![MultisigEvent, event, indexer, block_number, event_index]
            }
            Event::Bounties(event) => {
                index_bounties_event![BountiesEvent, event, indexer, block_number, event_index]
            }
            Event::ChildBounties(event) => {
                index_child_bounties_event![
                    ChildBountiesEvent,
                    event,
                    indexer,
                    block_number,
                    event_index
                ]
            }
            Event::Tips(event) => {
                index_tips_event![TipsEvent, event, indexer, block_number, event_index]
            }
            Event::ElectionProviderMultiPhase(event) => {
                index_election_provider_multi_phase_event![
                    ElectionProviderMultiPhaseEvent,
                    event,
                    indexer,
                    block_number,
                    event_index
                ]
            }
            Event::VoterList(event) => {
                index_bags_list_event![BagsListEvent, event, indexer, block_number, event_index]
            }
            Event::NominationPools(event) => {
                index_nomination_pools_event![
                    NominationPoolsEvent,
                    event,
                    indexer,
                    block_number,
                    event_index
                ]
            }
            Event::FastUnstake(event) => {
                index_fast_unstake_event![
                    FastUnstakeEvent,
                    event,
                    indexer,
                    block_number,
                    event_index
                ]
            }
            // Polkadot pallets.
            Event::Claims(event) => {
                index_claims_event![ClaimsEvent, event, indexer, block_number, event_index]
            }
            Event::Paras(event) => {
                index_paras_event![ParasEvent, event, indexer, block_number, event_index]
            }
            Event::Hrmp(event) => {
                index_hrmp_event![HrmpEvent, event, indexer, block_number, event_index]
            }
            Event::ParasDisputes(event) => {
                index_disputes_event![DisputesEvent, event, indexer, block_number, event_index]
            }
            Event::Registrar(event) => {
                index_paras_registrar_event![
                    ParasRegistrarEvent,
                    event,
                    indexer,
                    block_number,
                    event_index
                ]
            }
            Event::Slots(event) => {}
            Event::Auctions(event) => {}
            Event::Crowdloan(event) => {}
            Event::XcmPallet(event) => {}
            _ => {}
        };
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let _ = hybrid_indexer::start::<PolkadotIndexer>().await;
}

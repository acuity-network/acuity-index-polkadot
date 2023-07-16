#![feature(more_qualified_paths)]
#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}
use polkadot::Event;

use crate::polkadot::runtime_types::{
    frame_system::pallet::Event as SystemEvent,
    pallet_balances::pallet::Event as BalancesEvent,
    pallet_collective::pallet::{Event as CollectiveEvent, Event2 as CollectiveEvent2},
    pallet_democracy::pallet::Event as DemocracyEvent,
    pallet_elections_phragmen::pallet::Event as ElectionsPhragmenEvent,
    pallet_indices::pallet::Event as IndicesEvent,
    pallet_preimage::pallet::Event as PreimageEvent,
    pallet_session::pallet::Event as SessionEvent,
    pallet_staking::pallet::pallet::Event as StakingEvent,
    pallet_transaction_payment::pallet::Event as TransactionPaymentEvent,
    pallet_treasury::pallet::Event as TreasuryEvent,
    pallet_vesting::pallet::Event as VestingEvent,
};

struct PolkadotIndexer;

use hybrid_indexer::*;

impl hybrid_indexer::shared::RuntimeIndexer for PolkadotIndexer {
    type RuntimeConfig = subxt::PolkadotConfig;

    fn process_event(
        indexer: &hybrid_indexer::substrate::Indexer<Self>,
        block_number: u32,
        event_index: u32,
        event: subxt::events::EventDetails<Self::RuntimeConfig>,
    ) -> Result<(), subxt::Error> {
        match event.as_root_event::<Event>()? {
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
            Event::Claims(event) => {}
            Event::Utility(event) => {}
            Event::Identity(event) => {}
            Event::Proxy(event) => {}
            Event::Multisig(event) => {}
            Event::Bounties(event) => {}
            Event::ChildBounties(event) => {}
            Event::Tips(event) => {}
            Event::ElectionProviderMultiPhase(event) => {}
            Event::VoterList(event) => {}
            Event::NominationPools(event) => {}
            Event::FastUnstake(event) => {}
            Event::ParaInclusion(event) => {}
            Event::Paras(event) => {}
            Event::Hrmp(event) => {}
            Event::ParasDisputes(event) => {}
            Event::Registrar(event) => {}
            Event::Slots(event) => {}
            Event::Auctions(event) => {}
            Event::Crowdloan(event) => {}
            Event::XcmPallet(event) => {}
            Event::Ump(event) => {}
            _ => {}
        };
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let _ = hybrid_indexer::start::<PolkadotIndexer>().await;
}

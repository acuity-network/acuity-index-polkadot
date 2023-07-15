#![feature(more_qualified_paths)]
#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}
use polkadot::runtime_types::frame_system::pallet::Event as SystemEvent;
use polkadot::Event;

struct PolkadotIndexer;
pub mod substrate;
use crate::substrate::*;

use hybrid_indexer::*;

impl hybrid_indexer::shared::RuntimeIndexer for PolkadotIndexer {
    type RuntimeConfig = subxt::PolkadotConfig;

    fn process_event(
        indexer: &hybrid_indexer::substrate::Indexer<Self>,
        block_number: u32,
        event_index: u32,
        event: subxt::events::EventDetails<Self::RuntimeConfig>,
    ) {
        let event = event.as_root_event::<Event>();
        match event {
            Ok(event) => {
                //println!("Event: {:?}", event);
                match event {
                    Event::System(event) => {
                        index_system_event![SystemEvent, event, indexer, block_number, event_index]
                    }
                    Event::Scheduler(event) => {
                        scheduler_index_event(indexer, block_number, event_index, event);
                    }
                    Event::Preimage(event) => {
                        preimage_index_event(indexer, block_number, event_index, event);
                    }
                    Event::Indices(event) => {
                        indices_index_event(indexer, block_number, event_index, event);
                    }
                    Event::Balances(event) => {
                        balances_index_event(indexer, block_number, event_index, event);
                    }
                    Event::TransactionPayment(event) => {
                        transaction_payment_index_event(indexer, block_number, event_index, event);
                    }
                    Event::Staking(event) => {
                        staking_index_event(indexer, block_number, event_index, event);
                    }
                    Event::Offences(_) => {}
                    Event::Session(event) => {
                        session_index_event(indexer, block_number, event_index, event);
                    }
                    Event::Grandpa(_) => {}
                    Event::ImOnline(_) => {}
                    Event::Democracy(event) => {
                        democracy_index_event(indexer, block_number, event_index, event);
                    }
                    Event::Council(event) => {
                        collective_index_event(indexer, block_number, event_index, event);
                    }
                    Event::TechnicalCommittee(event) => {
                        collective2_index_event(indexer, block_number, event_index, event);
                    }
                    Event::PhragmenElection(event) => {
                        elections_phragmen_index_event(indexer, block_number, event_index, event);
                    }
                    Event::TechnicalMembership(event) => {}
                    Event::Treasury(event) => {}
                    Event::ConvictionVoting(event) => {}
                    Event::Referenda(event) => {}
                    Event::Whitelist(event) => {}
                    Event::Claims(event) => {}
                    Event::Vesting(event) => {}
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
                }
            }
            Err(_) => {}
        };
    }
}

#[tokio::main]
async fn main() {
    let _ = hybrid_indexer::start::<PolkadotIndexer>().await;
}

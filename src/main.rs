#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}
use polkadot::Event;

struct PolkadotIndexer;

pub mod substrate;
use crate::substrate::*;

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
                    Event::System(e) => {
                        system_index_event(indexer, block_number, event_index, e);
                    }
                    Event::Scheduler(e) => {
                        scheduler_index_event(indexer, block_number, event_index, e);
                    }
                    Event::Preimage(e) => {
                        preimage_index_event(indexer, block_number, event_index, e);
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
                    Event::Offences(event) => {}
                    Event::Session(event) => {
                        session_index_event(indexer, block_number, event_index, event);
                    }
                    Event::Grandpa(event) => {}
                    Event::ImOnline(event) => {}
                    Event::Democracy(event) => {}
                    Event::Council(event) => {}
                    Event::TechnicalCommittee(event) => {}
                    Event::PhragmenElection(event) => {}
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
    hybrid_indexer::start::<PolkadotIndexer>().await;

    /*
        let p = Pallets{
            bags_list: polkadot::voter_list::events::Rebagged{},
        };
    */
    //    test::<Pallets>();
}

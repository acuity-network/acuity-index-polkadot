#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}

struct PolkadotIndexer;

impl hybrid_indexer::shared::RuntimeIndexer for PolkadotIndexer {
    type RuntimeConfig = subxt::PolkadotConfig;

    fn process_event(
        indexer: &hybrid_indexer::substrate::Indexer<Self>,
        block_number: u32,
        event_index: u32,
        event: subxt::events::EventDetails<Self::RuntimeConfig>,
    ) {
        let event = event.as_root_event::<polkadot::Event>();
        match event {
            Ok(event) => {
                //               println!("Event: {:?}", event);
                match event {
                    polkadot::Event::System(e) => {}
                    polkadot::Event::Scheduler(e) => {}
                    polkadot::Event::Preimage(e) => {}
                    polkadot::Event::VoterList(e) => {
                        match e {
                            polkadot::runtime_types::pallet_bags_list::pallet::Event::Rebagged{who, from, to} => {
                                indexer.index_event_account_id(who, block_number, event_index);
                            }
                            polkadot::runtime_types::pallet_bags_list::pallet::Event::ScoreUpdated{who, new_score} => {
                                indexer.index_event_account_id(who, block_number, event_index);
                            }
                        }
                        /*
                        hybrid_indexer::pallets::bags_list::bags_list_index_event::<
                            Self,
                            polkadot::runtime_types::pallet_bags_list::pallet::Event,
                        >(indexer, block_number, event_index, e);
                        */
                    }
                    _ => (),
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

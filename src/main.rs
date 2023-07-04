#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}

mod substrate;
use crate::substrate::voter_list::*;

#[repr(u8)]
enum Runtime {
    VoterList(VoterListEvent) = 37,
}

struct PolkadotIndexer;

impl hybrid_indexer::shared::RuntimeIndexer for PolkadotIndexer {
    fn process_event(
        block_number: u32,
        event_index: u32,
        event: subxt::events::EventDetails<subxt::PolkadotConfig>,
    ) {
        let event = event.as_root_event::<polkadot::Event>();
        match event {
            Ok(event) => {
                println!("Event: {:?}", event);
                match event {
                    System => {}
                    Scheduler => {}
                    Preimage => {}
                    VoterList => {}
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

fn test<T>() {}

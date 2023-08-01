#![feature(more_qualified_paths)]
use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Chain {
    Polkadot,
    Kusama,
    Rococo,
    Westend,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_enum, default_value_t = Chain::Polkadot)]
    pub chain: Chain,
    /// URL of Substrate node to connect to.
    #[arg(short, long)]
    pub url: Option<String>,
    /// Block number to start indexing from.
    #[arg(short, long)]
    pub block_number: Option<u32>,
    /// How many blocks to query at the same time
    #[arg(short, long, default_value_t = 64)]
    pub async_blocks: u32,
    /// Port to open for WebSocket queries.
    #[arg(short, long, default_value_t = 8172)]
    pub port: u16,
}

mod polkadot;
use polkadot::PolkadotIndexer;
mod kusama;
use kusama::KusamaIndexer;
mod rococo;
use rococo::RococoIndexer;
mod westend;
use westend::WestendIndexer;
mod pallets;

#[tokio::main]
async fn main() {
    // Check command line parameters.
    let args = Args::parse();
    // Start the indexer.
    let _ = match args.chain {
        Chain::Polkadot => {
            hybrid_indexer::start::<PolkadotIndexer>(
                args.url,
                args.block_number,
                args.async_blocks,
                args.port,
            )
            .await
        }
        Chain::Kusama => {
            hybrid_indexer::start::<KusamaIndexer>(
                args.url,
                args.block_number,
                args.async_blocks,
                args.port,
            )
            .await
        }
        Chain::Rococo => {
            hybrid_indexer::start::<RococoIndexer>(
                args.url,
                args.block_number,
                args.async_blocks,
                args.port,
            )
            .await
        }
        Chain::Westend => {
            hybrid_indexer::start::<WestendIndexer>(
                args.url,
                args.block_number,
                args.async_blocks,
                args.port,
            )
            .await
        }
    };
}

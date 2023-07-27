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
    /// How many blocks to query at the same time [128]
    #[arg(short, long)]
    pub async_blocks: Option<u32>,
}

mod polkadot;
use polkadot::PolkadotIndexer;
mod pallets;

#[tokio::main]
async fn main() {
    // Check command line parameters.
    let args = Args::parse();
    let url = args
        .url
        .clone()
        .unwrap_or_else(|| "wss://rpc.polkadot.io:443".to_string());
    // Start the indexer.
    let _ =
        hybrid_indexer::start::<PolkadotIndexer>(url, args.block_number, args.async_blocks).await;
}

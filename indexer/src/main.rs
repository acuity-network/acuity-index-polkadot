#![feature(more_qualified_paths)]
use byte_unit::Byte;
use clap::{Parser, ValueEnum};
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Chain {
    Polkadot,
    Kusama,
    Rococo,
    Westend,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum DbMode {
    LowSpace,
    HighThroughput,
}

impl Into<sled::Mode> for DbMode {
    fn into(self) -> sled::Mode {
        match self {
            DbMode::LowSpace => sled::Mode::LowSpace,
            DbMode::HighThroughput => sled::Mode::HighThroughput,
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Chain to index
    #[arg(short, long, value_enum, default_value_t = Chain::Polkadot)]
    pub chain: Chain,
    /// Database path
    #[arg(short, long)]
    pub db_path: Option<String>,
    /// Database mode
    #[arg(long, value_enum, default_value_t = DbMode::LowSpace)]
    pub db_mode: DbMode,
    /// Maximum size in bytes for the system page cache
    #[arg(long, default_value = "1024.00 MiB")]
    pub db_cache_capacity: String,
    /// URL of Substrate node to connect to
    #[arg(short, long)]
    pub url: Option<String>,
    /// Maximum number of concurrent requests to the chain
    #[arg(long, default_value_t = 64)]
    pub queue_depth: u8,
    /// Port to open for WebSocket queries
    #[arg(short, long, default_value_t = 8172)]
    pub port: u16,
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
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
    let db_cache_capacity = Byte::from_str(args.db_cache_capacity)
        .unwrap()
        .get_bytes()
        .try_into()
        .unwrap();
    let log_level = args.verbose.log_level_filter();
    // Start the indexer.
    match args.chain {
        Chain::Polkadot => {
            hybrid_indexer::start::<PolkadotIndexer>(
                args.db_path,
                args.db_mode.into(),
                db_cache_capacity,
                args.url,
                args.queue_depth,
                args.port,
                log_level,
            )
            .await
        }
        Chain::Kusama => {
            hybrid_indexer::start::<KusamaIndexer>(
                args.db_path,
                args.db_mode.into(),
                db_cache_capacity,
                args.url,
                args.queue_depth,
                args.port,
                log_level,
            )
            .await
        }
        Chain::Rococo => {
            hybrid_indexer::start::<RococoIndexer>(
                args.db_path,
                args.db_mode.into(),
                db_cache_capacity,
                args.url,
                args.queue_depth,
                args.port,
                log_level,
            )
            .await
        }
        Chain::Westend => {
            hybrid_indexer::start::<WestendIndexer>(
                args.db_path,
                args.db_mode.into(),
                db_cache_capacity,
                args.url,
                args.queue_depth,
                args.port,
                log_level,
            )
            .await
        }
    };
}

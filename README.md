# Polkadot Indexer

Event indexer for Polkadot, Kusama, Westend and Paseo blockchains.

## Architecture

![Hybrid Architecture](https://raw.githubusercontent.com/ethernomad/hybrid-diagram/main/hybrid.png)

Polkadot Indexer uses the [Hybrid Indexer](https://github.com/hybrid-explorer/hybrid-indexer) Rust library. It can be accessed using [Hybrid Dapp](https://github.com/hybrid-explorer/hybrid-dapp).

## Building

Polkadot Indexer can be built using `cargo build`, however it is necessary to use the nightly `rustc`.

```sh
rustup default nightly
cargo build --release
```

Compiling `metadata` can take a very long time.

## Running

```
Usage: polkadot-indexer [OPTIONS]

Options:
  -c, --chain <CHAIN>                Chain to index [default: polkadot] [possible values: polkadot, kusama, westend, paseo]
  -d, --db-path <DB_PATH>            Database path
  -u, --url <URL>                    URL of Substrate node to connect to
  -b, --block-number <BLOCK_NUMBER>  Block number to start indexing from
      --queue-depth <QUEUE_DEPTH>    Maximum number of concurrent requests to the chain [default: 64]
  -p, --port <PORT>                  Port to open for WebSocket queries [default: 8172]
  -v, --verbose...                   More output per occurrence
  -q, --quiet...                     Less output per occurrence
  -h, --help                         Print help
  -V, --version                      Print version
```

## Docker

First build the docker image:

```sh
docker build .
```

Run the docker image for each chain in a separate tab (replace `[image_hash]` with the hash of the docker image displayed at the end of the build):

```sh
docker run --rm -p 8172:8172 [image_hash] -c polkadot -b 16730000 -p 8172
```

```sh
docker run --rm -p 8173:8173 [image_hash] -c kusama -b 19120000 -p 8173
```
  
```sh
docker run --rm -p 8175:8175 [image_hash] -c westend -b 16940000 -p 8175
```

```sh
docker run --rm -p 8174:8174 [image_hash] -c paseo -b 6550000 -p 8174
```

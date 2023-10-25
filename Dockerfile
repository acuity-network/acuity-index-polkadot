FROM rust:slim

WORKDIR /usr/src/polkadot-indexer

COPY . .

RUN rustup default nightly

RUN cargo build --release -j 1

EXPOSE 8172 8173 8174 8175

#ENTRYPOINT ["/usr/src/polkadot-indexer/target/release/polkadot-indexer"]

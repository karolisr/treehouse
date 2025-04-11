#!/usr/bin/env bash

export RUSTFLAGS='-C target-cpu=native'

# cargo install --locked cargo-zigbuild
cargo install cargo-bundle

cargo fmt && \

cargo check --profile dev && \
cargo clippy --profile dev && \
cargo build --profile dev && \

cargo check --profile release && \
cargo clippy --profile release && \
cargo build --profile release && \

# cargo zigbuild --profile release --target x86_64-unknown-linux-gnu.2.32
cargo-bundle bundle --profile release

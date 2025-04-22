#!/usr/bin/env bash

# rustc --print=target-cpus
# rustc --print cfg
# rustc -Ctarget-cpu=generic --print cfg
# rustc -Ctarget-cpu=native --print cfg

# RUSTFLAGS="-Ctarget-cpu=generic"
RUSTFLAGS="-Ctarget-cpu=native"
export RUSTFLAGS

cargo install cargo-bundle

cargo fmt && \

cargo check --profile dev && \
cargo clippy --profile dev && \
cargo build --profile dev && \

cargo check --profile release && \
cargo clippy --profile release && \
cargo build --profile release

# cargo-bundle bundle --profile release

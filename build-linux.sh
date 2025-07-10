#!/usr/bin/env bash

# -------------------------------------------
# Suppress printing of error messages
# exec 2>/dev/null

# Stop on first error
set -o errexit
# Set trap on ERR to be inherited by shell functions
set -o errtrace

# Trap errors
trap 'echo Error at line: $LINENO' ERR
# -------------------------------------------

# -------------------------------------------
cargo fmt
# -------------------------------------------
cargo check --profile dev
cargo clippy --profile dev
cargo build --profile dev --bin treehouse
# -------------------------------------------
cargo check --profile release
cargo clippy --profile release
cargo build --profile release --bin treehouse
# -------------------------------------------

# -------------------------------------------
cargo doc --profile release --document-private-items --no-deps --workspace
# -------------------------------------------

# -------------------------------------------
cargo install cargo-bundle 2>/dev/null
cargo-bundle bundle --profile release --bin treehouse
# -------------------------------------------

# -------------------------------------------
# cargo install --locked cargo-zigbuild 2>/dev/null
# RUSTFLAGS="-L/usr/lib64"
# export RUSTFLAGS
# cargo zigbuild --profile release --bin treehouse --target x86_64-unknown-linux-gnu.2.32
# -------------------------------------------

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
cargo check --all-targets --profile dev
cargo clippy --all-targets --profile dev
# cargo build --all-targets --profile dev
cargo build --profile dev --bin treehouse
# -------------------------------------------
# cargo check --all-targets --profile release
# cargo clippy --all-targets --profile release
# cargo build --all-targets --profile release
cargo build --profile release --bin treehouse
# -------------------------------------------

# -------------------------------------------
cargo doc --profile release --document-private-items --no-deps --workspace
# -------------------------------------------

# -------------------------------------------
cargo install cargo-bundle 2>/dev/null
cargo-bundle bundle --profile release --package treehouse --bin treehouse
# -------------------------------------------

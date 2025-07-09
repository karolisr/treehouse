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
cargo install cargo-bundle 2>/dev/null
cargo-bundle bundle --profile release --bin treehouse
# -------------------------------------------

# -------------------------------------------
BUNDLE_PATH="./target/release/bundle/osx/TreeHouse.app/Contents"
FILE="${BUNDLE_PATH}/Info.plist"
TOTAL_LINES=$(wc -l <"${FILE}")
head -n $((TOTAL_LINES - 2)) "${FILE}" >"Info.plist.tmp" && mv "Info.plist.tmp" "${FILE}"
cat "./resources/macos/info_plist_tail.txt" >>"${FILE}"
cp "./resources/macos/"*".icns" "${BUNDLE_PATH}/Resources/"
# -------------------------------------------
xattr -rc "./target/release/bundle/osx/TreeHouse.app" &&
    if [[ -n "${SIGNING_IDENTITY}" ]]; then
        echo -e "\nSigning identity: ${SIGNING_IDENTITY}"
        codesign --sign "${SIGNING_IDENTITY}" --options runtime "./target/release/bundle/osx/TreeHouse.app"
    else
        codesign --deep --force --sign - "./target/release/bundle/osx/TreeHouse.app"
    fi
# -------------------------------------------

#!/usr/bin/env bash

# rustc --print=target-cpus
# rustc --print cfg
# rustc -Ctarget-cpu=generic --print cfg
# rustc -Ctarget-cpu=native --print cfg

# RUSTFLAGS="-Ctarget-cpu=generic"
RUSTFLAGS="-Ctarget-cpu=native"
export RUSTFLAGS

# cargo install cargo-bundle
# cargo install --locked --git https://github.com/iced-rs/comet.git

cargo fmt && \

cargo check --profile dev && \
cargo clippy --profile dev && \
cargo build --profile dev && \

cargo check --profile release && \
cargo clippy --profile release && \
cargo build --profile release && \

cargo-bundle bundle --profile release && \

BUNDLE_PATH="./target/release/bundle/osx/TreeHouse.app/Contents"
FILE="${BUNDLE_PATH}/Info.plist"
TOTAL_LINES=$(wc -l < "${FILE}")
head -n $((TOTAL_LINES - 2)) "${FILE}" > "Info.plist.tmp" && mv "Info.plist.tmp" "${FILE}"
cat "./resources/macos/info_plist_tail.txt" >> "${FILE}"
cp "./resources/macos/"*".icns" "${BUNDLE_PATH}/Resources/"

xattr -rc "./target/release/bundle/osx/TreeHouse.app" && \

if [[ -n "${SIGNING_IDENTITY}" ]]; then
    echo -e "\nSigning identity: ${SIGNING_IDENTITY}"
    codesign --sign "${SIGNING_IDENTITY}" "./target/release/bundle/osx/TreeHouse.app"
else
    codesign --deep --force --sign - "./target/release/bundle/osx/TreeHouse.app"
fi

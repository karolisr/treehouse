#!/usr/bin/env bash

export RUSTFLAGS='-C target-cpu=native'

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
codesign --deep --force --sign - "./target/release/bundle/osx/TreeHouse.app"

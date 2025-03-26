cargo fmt && \
cargo check --profile release && \
cargo clippy --profile release && \
cargo build --profile release && \
cargo-bundle bundle --profile release --bin treehouse && \

cp ./resources/macos/Info.plist ./target/release/bundle/osx/TreeHouse.app/Contents/
cp ./resources/macos/*.icns ./target/release/bundle/osx/TreeHouse.app/Contents/Resources/

xattr -rc ./target/release/bundle/osx/TreeHouse.app && \
codesign --deep --force --sign - ./target/release/bundle/osx/TreeHouse.app

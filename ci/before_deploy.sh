#!/bin/bash
set -e

CUR_DIR="$(pwd)"
TARGET='x86_64-unknown-linux-musl'
TARGET_DIR="$CUR_DIR/target/$TARGET/release/"
LOCAL_USER="$(id -u):$(id -g)"
docker run \
    -v "$CUR_DIR":/home/rust/src \
    -v "$HOME/.cargo/registry":/home/rust/.cargo/registry \
    --rm \
    ekidd/rust-musl-builder \
    /bin/bash -c \
    "set -e && mkdir -p target/ \
        && sudo chown -R rust:rust target/ /home/rust/.cargo/registry/ \
        && rustup update stable \
        && cargo build --target=$TARGET --release \
        && sudo chown $LOCAL_USER target/$TARGET/release/$BIN_NAME"

cd "$TARGET_DIR" || exit 1

tar -czf "$CUR_DIR/$BIN_NAME-$TRAVIS_TAG.$TARGET.tar.gz" "$BIN_NAME"

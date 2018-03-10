#!/bin/bash
set -e

CUR_DIR="$(pwd)"
TARGET='x86_64-unknown-linux-musl'
LOCAL_USER="$(id -u):$(id -g)"
REGISTRY_DIR="$CUR_DIR/target/registry/"

mkdir -p "$REGISTRY_DIR"

docker run \
    -v "$CUR_DIR":/home/rust/src \
    -v "$REGISTRY_DIR":/home/rust/.cargo/registry \
    --rm \
    ekidd/rust-musl-builder:stable \
    /bin/bash -c \
    "set -e && sudo chown rust:rust /home/rust/.cargo/registry/ target/ \
        && cargo build --target=$TARGET --release \
        && sudo chown $LOCAL_USER target/$TARGET/release/$BIN_NAME"

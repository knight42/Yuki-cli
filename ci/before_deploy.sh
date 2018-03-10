#!/bin/bash
set -e

CUR_DIR="$(pwd)"
TARGET='x86_64-unknown-linux-musl'
TARGET_DIR="$CUR_DIR/target/$TARGET/release/"

cd "$TARGET_DIR" || exit 1

tar -czf "$CUR_DIR/$BIN_NAME-$TRAVIS_TAG.$TARGET.tar.gz" "$BIN_NAME"

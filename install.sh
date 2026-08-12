#!/bin/bash

set -e

cd "$(dirname "$0")"

PROJECT_NAME="ltop"

echo "=== Build project ==="
cargo build --release

echo "=== Create symlink ==="
sudo ln -sf "$(pwd)/target/release/$PROJECT_NAME" /usr/local/bin/$PROJECT_NAME

echo "=== Installation complete ==="
echo "Now you can run the program using - $PROJECT_NAME"
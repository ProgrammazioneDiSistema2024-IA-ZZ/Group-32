#!/bin/bash

rm -rf "release/linux"

# Build the project with cargo in release mode for the entire workspace
cargo build --release --workspace

# Create the release/linux directory if it does not exist
if [ ! -d "release/linux" ]; then
    mkdir -p release/linux
fi

# Copy the executables to the release/linux directory
cp target/release/progetto_g32 release/linux/progetto_g32
cp target/release/setup release/linux/setup
cp target/release/uninstall release/linux/uninstall

echo "Build and copy process completed successfully."
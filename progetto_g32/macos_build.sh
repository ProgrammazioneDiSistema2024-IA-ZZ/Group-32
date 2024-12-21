#!/bin/bash

rm -rf "release/macos"

# Build the project with cargo in release mode for the entire workspace
cargo build --release --workspace

# Create the release/macos directory if it does not exist
if [ ! -d "release/macos" ]; then
    mkdir -p release/macos
fi

# Copy the executables to the release/macos directory
if [ -f "target/release/progetto_g32" ]; then
    cp target/release/progetto_g32 release/macos/progetto_g32
fi

if [ -f "target/release/setup" ]; then
    cp target/release/setup release/macos/setup
fi

if [ -f "target/release/uninstall" ]; then
    cp target/release/uninstall release/macos/uninstall
fi


echo "Build and copy process completed successfully."
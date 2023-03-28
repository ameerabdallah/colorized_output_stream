#!/bin/bash

# Build and install the package
cargo build --release
sudo cp target/release/colorize_output_streams /usr/local/bin/clos

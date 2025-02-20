#!/bin/bash
cd "$(dirname "$BASH_SOURCE")/../"

cargo build --release --manifest-path assembler/Cargo.toml
cpu/scripts/build_all.sh

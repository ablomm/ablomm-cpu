#!/bin/bash

assembler_path="$(dirname "$BASH_SOURCE")/../assembler/target/release/ablomm_asm"
./$assembler_path "$@"

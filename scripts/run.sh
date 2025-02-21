#!/bin/bash

script_dir="$(dirname "$BASH_SOURCE")"
./$script_dir/simulate.sh +src=<(./$script_dir/assemble.sh "$1" || kill $$) +verilator+quiet

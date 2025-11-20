#!/bin/bash
cd "$(dirname "$BASH_SOURCE")"

./verilator/build_test.sh
./iverilog/build_test.sh

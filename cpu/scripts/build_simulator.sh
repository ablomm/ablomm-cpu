#!/bin/bash
cd "$(dirname "$BASH_SOURCE")"

./verilator/build_simulator.sh
./iverilog/build_simulator.sh

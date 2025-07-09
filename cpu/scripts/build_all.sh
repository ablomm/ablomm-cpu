#!/bin/bash
cd "$(dirname "$BASH_SOURCE")"

verilator/simulation.sh
verilator/test.sh
iverilog/simulation.sh
iverilog/test.sh

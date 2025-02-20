#!/bin/bash
cd "$(dirname "$BASH_SOURCE")"

verilator/simulation.sh
verilator/test.sh
icarus/simulation.sh
icarus/test.sh

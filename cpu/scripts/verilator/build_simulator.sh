#!/bin/bash
cd "$(dirname "$BASH_SOURCE")/../../"

verilator -f scripts/verilator/simulator.f

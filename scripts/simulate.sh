#!/bin/bash

simulator_path="$(dirname "$BASH_SOURCE")/../cpu/build/verilator/Vsimulator"
./$simulator_path "$@"

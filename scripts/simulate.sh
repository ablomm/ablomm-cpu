#!/bin/bash

simulator_path="$(dirname "$BASH_SOURCE")/../cpu/build/icarus/Vsimulator"
./$simulator_path "$@"

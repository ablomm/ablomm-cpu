#!/bin/bash

simulator_path="$(dirname "$BASH_SOURCE")/../cpu/build/$1/Vsimulator"
./$simulator_path ${@:2}

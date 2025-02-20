#!/bin/bash
cd "$(dirname "$BASH_SOURCE")/../../"

iverilog -o build/icarus/Vsimulation -c scripts/file_list/simulation.txt -g2012 

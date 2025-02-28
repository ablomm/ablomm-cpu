#!/bin/bash
cd "$(dirname "$BASH_SOURCE")/../../"

iverilog -o build/icarus/Vsimulator -c scripts/file_list/simulation.txt -g2012 

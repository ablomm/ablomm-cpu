#!/bin/bash
cd "$(dirname "$BASH_SOURCE")/../../"

iverilog -o build/iverilog/Vsimulator -c scripts/file_list/simulation.txt -g2012 

#!/bin/bash
cd "$(dirname "$BASH_SOURCE")/../../"

iverilog -o build/icarus/Vtest -c scripts/file_list/test.txt -g2012 

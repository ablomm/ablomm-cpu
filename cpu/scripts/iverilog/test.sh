#!/bin/bash
cd "$(dirname "$BASH_SOURCE")/../../"

iverilog -o build/iverilog/Vtest -c scripts/file_list/test.txt -g2012 

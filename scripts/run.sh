#!/bin/bash

script_dir="$(dirname "$BASH_SOURCE")"

extra_options=()

case "$1" in
	"verilator")
		extra_options+=( "+verilator+quiet" )
		;;
			
	"iverilog")
		;;
esac

./$script_dir/simulate.sh "$1" +src=<(./$script_dir/assemble.sh "$2") ${extra_options[@]} 

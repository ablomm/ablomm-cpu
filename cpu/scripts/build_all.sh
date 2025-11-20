#!/bin/bash
cd "$(dirname "$BASH_SOURCE")"

./build_simulator.sh
./build_test.sh

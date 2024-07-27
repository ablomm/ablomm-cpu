mkdir -p build
file_list=${1:-"file_list.txt"}
iverilog -o build/out -c ${file_list} -g2012 

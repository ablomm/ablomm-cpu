# Building

To build the application, you must have installed the following:
- [Rust](https://www.rust-lang.org/)
- Either [Verilator](https://www.veripool.org/verilator/) and/or [Icarus Verilog](https://steveicarus.github.io/iverilog/) (I recommend Verilator, as it is much faster)
- Bash (to use the scripts, otherwise you will need to run the commands / write scripts manually for your shell)
  - For Windows, you can get Bash through [Mingw](https://www.mingw-w64.org/)
 
Included in the repo is a [script to build everything](../scripts/build_all.sh).

To build, simply run from the project directory:

```bash
$ ./scripts/build_all.sh
```

# Running

Included in the repo is a [script to assemble and run a program](../scripts/run.sh).

To run the included hello_world program, simply run from the project directory:

```bash
$ ./scripts/run.sh verilator programs/hello_world.asm
```

or, using Icarus Verilog:

```bash
$ ./scripts/run.sh iverilog programs/hello_world.asm
```

## Assemble

Usually it's fine to just use the `run.sh` script, which will assemble and run the application all in one, but you may want to only assemble a program.

Included in the repo is a [script to assemble a program](../scripts/assemble.sh).

To assemble the included hello_world program, simply run from the project directory:

```bash
$ ./scripts/assmeble.sh programs/hello_world.asm
```

> [!NOTE]  
> By default, the assembler will print the machine code to stdout. You can optionally write the output to a file using redirection or using the `-o <OUTPUT>` option. For a full list of options the assembler supports, use the `-h` option.

## Simulate

The simulator allows you to run a program by passing in the machine code for that program.

Included in the repo is a [script to run the simulator](../scripts/simulate.sh).

To simulate a program `hello_world` (which contains the machine code), simply run from the project directory:

```bash
$ ./scripts/simulate.sh verilator +src=hello_world
```

or, using Icarus Verilog:

```bash
$ ./scripts/simulate.sh iverilog +src=hello_world
```

> [!NOTE]  
> The scripts will simply delegate all inputs after the first to the simulator. Therefore, all Verilator or Icarus Verilog options can also be passed through this script.

## Binaries

Binaries can found in the [releases page](https://github.com/ablomm/ablomm-cpu/releases). These binaries will not work with the scripts.

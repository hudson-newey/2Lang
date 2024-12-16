# 2Lang Compiler

## Commands

- `build` Build a 2Lang application
- `mod docgen` Build 2Lang documentation

## Command Line Options

- `--run` automatically runs the program after compilation

## Compiler Options

- `-b`, `--generate-intermediate` takes an intermediary output (_.bin_) file produced by the pre-processor
- `-d`, `--debug` Run in debug mode (log file reads, etc...)
- `-s`, `--stdout` Output the final file contents to stdout
- `-o`, `--output` Specify the output file name

## Pre-Processor Options

- `-p`, `--preserve-intermediate` preserve the output produced by the pre-processor
- `--preserve-linked` Preserves the statically linked file (without macros expanded)

## Optimizer Options

- `--skip-optimizer` Does not run the optimizer on the output source code
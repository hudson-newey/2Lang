#!/usr/bin/env bash
# we do not fail on compilation here because I expect some source files to be invalid

declare COMPILER="target/debug/two_lang"
declare INPUT_DIR="tests/input"
declare OUTPUT_DIR="tests/output"

for file in $INPUT_DIR/*.2; do
    echo "Compiling $file"

    declare OUTPUT_FILE="$OUTPUT_DIR/$(basename $file).out"
    declare COMPILER_CMD="$COMPILER $file -o $OUTPUT_FILE"

    $COMPILER_CMD
done

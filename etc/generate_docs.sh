#!/usr/bin/env bash
set -euo pipefail

declare COMPILER="target/debug/two_lang"

for file in lib/**/*.2; do
    declare COMPILER_CMD="$COMPILER mod docgen $file"

    $COMPILER_CMD
done

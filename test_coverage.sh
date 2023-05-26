#!/usr/bin/env bash

RUSTFLAGS="-C instrument-coverage" \
    cargo test --tests

llvm-profdata merge -sparse default_*.profraw -o json5format.profdata


llvm-cov report \
     --use-color \
     --ignore-filename-regex='/.cargo/registry' \
     --instr-profile=json5format.profdata \
     --object target/debug/deps/py_serial_rs-73530a03b3d7dc3b

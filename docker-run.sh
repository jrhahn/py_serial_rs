#!/bin/bash

docker run --rm -v $(pwd):/io py-serial-rs-builder build --release  # or other maturin arguments

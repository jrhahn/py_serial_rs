#!/bin/bash

docker run --rm -v $(pwd):/io py-serial-rs-builder publish  # or other maturin arguments

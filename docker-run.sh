#!/bin/bash

docker run --rm -it -v $(pwd):/io py-serial-rs-builder   # or other maturin arguments

#!/bin/bash

# Compile as release and then run the program. We do this because the optimized
# code is MUCH faster than the unoptomized, and it is well worth the optimized
# compile time.

cargo build --release

time ./target/release/ray-tracer > img.ppm && xdg-open img.ppm

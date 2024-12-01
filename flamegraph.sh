#!/bin/sh
set -e

cargo flamegraph --profile flamegraph --root --package $1 --image-width 800 -o flamegraph.svg
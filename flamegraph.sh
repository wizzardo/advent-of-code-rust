#!/bin/sh
set -e

cargo flamegraph --profile flamegraph --root --package $1 -o flamegraph.svg
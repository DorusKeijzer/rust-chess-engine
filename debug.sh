#!/bin/sh

# Check if arguments are provided
if [ $# -lt 2 ]; then
    echo "Usage: ./debug.sh <depth> <fen> [moves]"
    exit 1
fi
# echo "Debug: Received args: $@" >&2
# build most recent chess engine
cargo build --release -q

# Fix: Enclose $fen in quotes to handle spaces properly
target/release/chess debug "$2" $1
# Check if arguments are provided

#!/bin/bash

# Check if argument is provided
if [ $# -eq 0 ]; then
    echo "Usage: ./compare_outputs.sh <argument>"
    exit 1
fi

argument="$1"

echo "Building latest version of chess engine..."
# builds rust program
cargo build --release -q

# Define the paths to the Python and Rust executables
pythonScriptPath="pythontest.py"
rustExecutablePath="target/release/chess"

# Run the Python script and capture its output
pythonOutput=$(python "$pythonScriptPath" "$argument")

# Run the Rust executable and capture its output
rustOutput=$("$rustExecutablePath" script "$argument")

# Get the last line of each output
lastPythonLine=$(echo "$pythonOutput" | tail -n 1)
lastRustLine=$(echo "$rustOutput" | tail -n 1)

if [ "$lastPythonLine" = "$lastRustLine" ]; then
    echo "Same number of moves"
else
    # Find lines that are unique to each output
    echo "Unique lines in Python output:"
    diff <(echo "$pythonOutput") <(echo "$rustOutput") | grep "^<" | cut -c 3-

    echo "Unique lines in Rust output:"
    diff <(echo "$pythonOutput") <(echo "$rustOutput") | grep "^>" | cut -c 3-
fi

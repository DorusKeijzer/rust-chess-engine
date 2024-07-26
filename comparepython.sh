#!/bin/bash

# Check if argument is provided
if [ $# -eq 0 ]; then
    echo "Usage: ./compare_outputs.sh <argument>"
    exit 1
fi

argument="$1"
norebuild=false
if [ "$2" = "--norebuild" ]; then
    norebuild=true
fi

if ! $norebuild; then 
    echo "Building latest version of chess engine..."
# builds rust program
    cargo build --release -q
fi
# Define the paths to the Python and Rust executables
pythonScriptPath="pythontest.py"
rustExecutablePath="target/release/chess"

$rustExecutablePath draw "$argument"

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
    # Use comm to find lines unique to each output
    echo "Unique lines in Python output:"
    comm -23 <(echo "$pythonOutput" | sort) <(echo "$rustOutput" | sort)

    echo "Unique lines in Rust output:"
    comm -13 <(echo "$pythonOutput" | sort) <(echo "$rustOutput" | sort)
fi

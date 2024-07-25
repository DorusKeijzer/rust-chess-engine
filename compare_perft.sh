#!/bin/bash

# Check if argument is provided
if [ $# -eq 0 ]; then
    echo "Usage: ./find_problem_positions.sh <fen> <depth>"
    exit 1
fi

fen="$1"
depth="$2"

echo "Building latest version of chess engine..."
cargo build --release -q

pythonScriptPath="quietperft.py"
rustExecutablePath="target/release/chess"

find_problem_position() {
    local current_fen="$1"
    local current_depth="$2"

    echo "Analyzing position: $current_fen at depth $current_depth"

    python_output=$(python "$pythonScriptPath" "$current_fen" "$current_depth")
    rust_output=$("$rustExecutablePath" quiet "$current_fen" "$current_depth")
    
     echo $python_output
     echo $rust_output 

    python_count=$(echo "$python_output" | tail -n 1 | awk '{print $NF}')
    rust_count=$(echo "$rust_output" | tail -n 1 | awk '{print $NF}')

    if [ "$python_count" = "$rust_count" ]; then
        echo "No problem at this position and depth"
        return
    fi

    if [ "$current_depth" = "1" ]; then
        echo "Problematic position found: $current_fen"
        echo "Python count: $python_count"
        echo "Rust count: $rust_count"
        return
    fi

    # Get move list from Python (assuming it's correct)
    moves=$(python "$pythonScriptPath" "$current_fen" "alg_moves")

    # Recursively check each move
    while IFS= read -r move; do
        new_fen=$(python "$pythonScriptPath" "$current_fen" "make_move" "$move")
        find_problem_position "$new_fen" $((current_depth - 1))
    done <<< "$moves"
}

find_problem_position "$fen" "$depth"

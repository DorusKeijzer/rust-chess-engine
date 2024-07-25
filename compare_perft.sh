
# Check if arguments are provided
if [ $# -lt 2 ]; then
    echo "Usage: ./find_problem_positions.sh <fen> <depth> [--verbose]"
    exit 1
fi

fen="$1"
depth="$2"
verbose=false

# Check for verbose flag
if [ "$3" = "--verbose" ]; then
    verbose=true
fi

# Build the latest version of chess engine
if $verbose; then
    echo "Building latest version of chess engine..."
fi
cargo build --release -q

pythonScriptPath="quietperft.py"
rustExecutablePath="target/release/chess"

find_problem_position() {
    local current_fen="$1"
    local current_depth="$2"
    
    if $verbose; then
        echo "Analyzing position: $current_fen at depth $current_depth"
    fi
    
    python_output=$(python "$pythonScriptPath" "$current_fen" "$current_depth")
    rust_output=$("$rustExecutablePath" quiet "$current_fen" "$current_depth")
    
    if $verbose; then
        echo $python_output
        echo $rust_output 
    fi
    
    python_count=$(echo "$python_output" | tail -n 1 | awk '{print $NF}')
    rust_count=$(echo "$rust_output" | tail -n 1 | awk '{print $NF}')
    
    if [ "$python_count" = "$rust_count" ]; then
        if $verbose; then
            echo "No problem at this position and depth"
        fi
        return
    fi
    
    if [ "$current_depth" = "1" ]; then
        echo "problem at depth $current_depth"
        echo $current_fen
        echo "python count: $python_count"
        echo "rust count:   $rust_count"
        ./comparepython.sh "$current_fen"
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

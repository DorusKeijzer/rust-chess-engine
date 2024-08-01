
#!/bin/bash
fen="$1"
depth="$2"
echo "$fen"
echo "$depth"
cargo build -r -q
echo debug "$fen" $depth

# Process Rust output
rustoutput=$(./target/release/chess debug "$fen" $depth | sort | tr '\n' '\0' | xargs -0 -n1 echo)

# Capture and process Stockfish output
stockfishoutput=$(/usr/local/bin/stockfish <<EOD
position fen $fen
go perft $depth
EOD
)

# Sort Stockfish output, excluding the last line (which is typically a summary)
sorted_stockfishoutput=$(echo "$stockfishoutput" | sed '$d' | sort | tr '\n' '\0' | xargs -0 -n1 echo)

# Save outputs to files
echo "$rustoutput" > rust_output.txt
echo "$sorted_stockfishoutput" > stockfish_output.txt

# Compare the outputs
comm -3 rust_output.txt stockfish_output.txt

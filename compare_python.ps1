param (
    [string]$argument
)
Write-Host "Building latest version of chess engine..."
# builds rust program
cargo build --release -q

# Define the paths to the Python and Rust executables
$pythonScriptPath = "C:\Users\dorus\rust_projects\chess\pythontest.py"
$rustExecutablePath = "C:\Users\dorus\rust_projects\chess\target\release\chess.exe"

# Check if argument is provided
if (-not $argument) {
    Write-Host "Usage: .\compare-outputs.ps1 -argument <argument>"
    exit 1
}

# Run the Python script and capture its output
# Write-Host "Running Python script with argument: $argument"
$pythonOutput = & python $pythonScriptPath $argument
# Write-Host "Python Output:`n$pythonOutput"

# Run the Rust executable and capture its output
# Write-Host "Running Rust executable with argument: $argument"
$rustOutput = & $rustExecutablePath script $argument 
# Write-Host "Rust Output:`n$rustOutput"


# Split the outputs into arrays of lines
$pythonLines = $pythonOutput -split "`n"
$rustLines = $rustOutput -split "`n"

# Get the last line of each output
$lastPythonLine = $pythonLines[-1]
$lastRustLine = $rustLines[-1]

if ($lastPythonLine -eq $lastRustLine)
{
Write-Host "Same number of moves"
}
else
{
# Find lines that are unique to each output
$uniquePythonLines = $pythonLines | Where-Object { $_ -notin $rustLines }
$uniqueRustLines = $rustLines | Where-Object { $_ -notin $pythonLines }
# Display the unique lines
Write-Host "Unique lines in Python output:"
$uniquePythonLines

Write-Host "Unique lines in Rust output:"
$uniqueRustLines
}

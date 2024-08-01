import chess
import chess.engine
from sys import argv

fen = argv[1]
# Path to your Stockfish executable
STOCKFISH_PATH = r"/usr/local/bin/stockfish"

# Initialize the chess board with a given FEN position or use the starting position
# or use chess.Board(fen_string) for a specific position
board = chess.Board(fen)

# Initialize Stockfish engine
with chess.engine.SimpleEngine.popen_uci(STOCKFISH_PATH) as engine:
    # Get all legal moves in the current position
    legal_moves = list(board.legal_moves)
    # Print the legal moves
    for move in legal_moves:
        print(f"{board.piece_at(move.from_square)}".upper(
        ) + f" from {chess.square_name(move.from_square)} to {chess.square_name(move.to_square)}")
    # Close the engine
    engine.quit()

import chess
import chess.engine
from sys import argv

fen = argv[1]
# Path to your Stockfish executable
STOCKFISH_PATH = r"C:\Users\dorus\OneDrive\Bureaublad\stockfish-windows-x86-64\stockfish\stockfish-windows-x86-64.exe"

# Initialize the chess board with a given FEN position or use the starting position
board = chess.Board(fen)  # or use chess.Board(fen_string) for a specific position

# Initialize Stockfish engine
with chess.engine.SimpleEngine.popen_uci(STOCKFISH_PATH) as engine:
    # Get all legal moves in the current position
    legal_moves = list(board.legal_moves)
    # Print the legal moves
    for move in legal_moves:
        print(f"{board.piece_at(move.from_square)} from {chess.square_name(move.from_square)} to {chess.square_name(move.to_square)}")
    print(len(legal_moves))

    # Optionally, you can also evaluate a position or get the best move
    # info = engine.analyse(board, chess.engine.Limit(time=0.1))
    # print("Score:", info["score"])
    # best_move = engine.play(board, chess.engine.Limit(time=0.1))
    # print("Best move:", best_move.move)

    # Close the engine
    engine.quit()

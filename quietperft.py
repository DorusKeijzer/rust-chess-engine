
import chess
import chess.engine
from sys import argv

fen = argv[1]
# path to your stockfish executable
STOCKFISH_PATH = r"/usr/bin/stockfish"

# Initialize the chess board with a given FEN position or use the starting position
# or use chess.Board(fen_string) for a specific position
board = chess.Board(fen)


def perft(board, depth):
    total = 0
    legal_moves = list(board.legal_moves)
    if depth == 1:
        return len(legal_moves)
    for move in legal_moves:
        board.push(move)
        total += perft(board, depth - 1)
        board.pop()
    return total


if len(argv) > 2:
    if argv[2].isnumeric():
        # performs perft on the specfied depth
        depth = int(argv[2])

        # Initialize Stockfish engine
        print(perft(board, depth))
    elif argv[2] == "moves":
        # lists all legal moves in the same notatio nas the rust program
        legal_moves = list(board.legal_moves)
        for move in legal_moves:
            print(f"{board.piece_at(move.from_square)} from {chess.square_name(
                move.from_square)} to {chess.square_name(move.to_square)}")
    elif argv[2] == "alg_moves":
        # lists all legal moves in algebraic notation
        legal_moves = list(board.legal_moves)
        for move in legal_moves:
            print(move)
    elif argv[2] == "make_move":
        # makes move and prints the resulting board state.
        move = argv[3]
        board.push_san(move)
        print(board.fen())

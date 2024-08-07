# To do list:
* finish PERFT
* track PERFT time per commit
* optimize move generation 
  * magic bitboards
  * pawn generation through bitboards
  * store moves as array not vec
  * inlining commonly used functions
  * faster check detection
  * parallelization
* search
  * alpha beta pruning
  * move ordering
  * zobrist hashing
* evaluation:
  * neural network as evaluation function
* opening moves

## Perft Results

Last updated: 2024-08-07 10:09:50

Position 0: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -
| Depth | Perft score | time | correct  |
|-------|-------------|------|----------|
| 1 | 20/20 | 20.76µs | true |
| 2 | 400/400 | 112.64µs | true |
| 3 | 8902/8902 | 2.20ms | true |
| 4 | 197281/197281 | 50.96ms | true |
| 5 | 4865609/4865609 | 1.36s | true |

Position 1: r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 
| Depth | Perft score | time | correct  |
|-------|-------------|------|----------|
| 1 | 48/48 | 22.32µs | true |
| 2 | 2039/2039 | 717.47µs | true |
| 3 | 97729/97862 | 26.45ms | false |
| 4 | -- | -- | false |
| 5 | -- | -- | false |

Position 2: 8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -  
| Depth | Perft score | time | correct  |
|-------|-------------|------|----------|
| 1 | 14/14 | 4.37µs | true |
| 2 | 191/191 | 46.06µs | true |
| 3 | 2812/2812 | 561.27µs | true |
| 4 | 43238/43238 | 8.94ms | true |
| 5 | 674624/674624 | 132.50ms | true |

Position 3: r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1
| Depth | Perft score | time | correct  |
|-------|-------------|------|----------|
| 1 | 6/6 | 12.22µs | true |
| 2 | 264/264 | 93.19µs | true |
| 3 | 9467/9467 | 2.84ms | true |
| 4 | 421897/422333 | 119.07ms | false |
| 5 | -- | -- | false |

Position 4: rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  
| Depth | Perft score | time | correct  |
|-------|-------------|------|----------|
| 1 | 44/44 | 15.24µs | true |
| 2 | 1486/1486 | 424.68µs | true |
| 3 | 62379/62379 | 19.11ms | true |
| 4 | 2103487/2103487 | 587.99ms | true |
| 5 | 89945753/89941194 | 23.37s | false |

Position 5: r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10
| Depth | Perft score | time | correct  |
|-------|-------------|------|----------|
| 1 | 46/46 | 11.72µs | true |
| 2 | 2079/2079 | 408.78µs | true |
| 3 | 89890/89890 | 18.36ms | true |
| 4 | 3894594/3894594 | 774.97ms | true |
| 5 | 164075551/164075551 | 35.23s | true |
<!-- End of Perft Results -->
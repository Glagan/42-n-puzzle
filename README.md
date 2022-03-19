# n-puzzle

## Usage

```bash
cargo run -- {options} {puzzle_path...}
Options:
--heuristic     linear-conflict (default, best)
                manhattan
                euclidean
                hamming (worst)
--solution-type snail (default)
                first
                last
--mode          normal (default)
                greedy (skip g(n))
                uniform (skip h(n))
--amount        number (amount of puzzle to generate)
--size          number (size of the generated puzzles)
--unsolvable    (make generated puzzle unsolvable)
```

## Resources

* Rust Book
    * https://doc.rust-lang.org/book/
* Binary Heap
    * https://doc.rust-lang.org/std/collections/binary_heap/index.html
* n-puzzle
    * https://en.wikipedia.org/wiki/15_puzzle#Solvability
* A*
    * https://en.wikipedia.org/wiki/A*_search_algorithm
    * https://algorithmsinsight.wordpress.com/graph-theory-2/a-star-in-general/
    * https://www.aaai.org/Papers/AAAI/1996/AAAI96-178.pdf

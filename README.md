# n-puzzle

This project is an [n-puzzle](https://en.wikipedia.org/wiki/15_puzzle) solver using different algorithms and heuristics.  
You can solve maps by adding them as the last arguments, or they can be directly generated if no maps are given.

This solver can easily solve up to 6x6 maps (in **greedy** mode) and *may* solve easy 7x7 maps.

## Usage

```bash
cargo run -- {options} {puzzle_path...}
Options:
--variant       ida* (default, memory efficient)
                a* (use a lot of memory)
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
--unsolvable    true (make generated puzzle unsolvable)
```

## Resources

* Rust Book
    * https://doc.rust-lang.org/book/
* Binary Heap
    * https://doc.rust-lang.org/std/collections/binary_heap/index.html
* n-puzzle
    * https://en.wikipedia.org/wiki/15_puzzle#Solvability
* A* and heuristics
    * https://en.wikipedia.org/wiki/A*_search_algorithm
    * https://algorithmsinsight.wordpress.com/graph-theory-2/a-star-in-general/
    * https://www.aaai.org/Papers/AAAI/1996/AAAI96-178.pdf
    * https://arxiv.org/pdf/1107.0050.pdf

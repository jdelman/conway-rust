# conway-rust

Using this to teach myself some Rust basics.

This implementation is bounded and uses Vectors as a storage mechanism. This is naive compared to more advanced algorithms like [HashLife](https://en.wikipedia.org/wiki/Hashlife) and [QuickLife](http://golly.sourceforge.net/Help/Algorithms/QuickLife.html), but it requires a very low memory footprint, which was a goal of this project. I do not claim that this is a great implementation of the Game of Life. For pete's sake, the inventor of the [Glider Gun](http://www.conwaylife.com/wiki/Gosper_glider_gun) made HashLife in the 80s.

The grid is stored using a struct that keeps track of its dimensions and the current state. State is represented using a one-dimensional vector made up of `usize` (=uint64 on my MacBook Pro) integers, where each cell is represented by a single bit. I use bitwise operations to read and write to an individual integer in the vector. Thus a 128x128 board requires exactly 1k of memory to store.

The basic algorithm for this implementation (needs more detail):

 - Get a new empty board.
 - For each cell on the last board,
    - Get the valid neighbors.
    - Count the number of living neighbors.
    - Based on B3/S23 rule, decide whether the cell lives or dies in the next generation.
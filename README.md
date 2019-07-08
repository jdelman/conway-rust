# conway-rust

### Disclaimer

Using this to teach myself some Rust basics.

This implementation is bounded and uses Vectors as a storage mechanism. This is naive compared to more advanced algorithms like [HashLife](https://en.wikipedia.org/wiki/Hashlife) and [QuickLife](http://golly.sourceforge.net/Help/Algorithms/QuickLife.html), but it requires a very low memory footprint, which was a goal of this project. I do not claim that this is a great implementation of the Game of Life. For pete's sake, the inventor of the [Glider Gun](http://www.conwaylife.com/wiki/Gosper_glider_gun) made HashLife in the 80s.

### How it works

The grid is stored using a struct that keeps track of its dimensions and the current state. State is represented using a one-dimensional vector made up of `usize` (=uint64 on my MacBook Pro) integers, where each cell is represented by a single bit. I use bitwise operations to read and write to an individual integer in the vector. Thus a 128x128 board requires exactly 1k of memory to store.

The basic algorithm for this implementation (needs more detail):

 - Get a new empty board.
 - For each cell on the last board,
    - Get the valid neighbors.
    - Count the number of living neighbors.
    - Based on B3/S23 rule, decide whether the cell lives or dies in the next generation.

### Building & running

To do a release build, run `cargo build --release`. The binary will be written to `target/release/conway-rust`. When you run it, you can add two command line arguments corresponding to width and height. If you do not specify these arguments a 32x32 board will be created.

So, if you want to create a 1024x1024 board, run `conway-rust 1024 1024`

### License

Copyright 2019 Josh Delman

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
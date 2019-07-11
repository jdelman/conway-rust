use std::env;
use std::sync::mpsc;
use std::time::Instant;
use std::thread;

mod grid;
mod conway;
mod octets;

use grid::{Grid, GridSize};
use conway::{Conway};

fn main() {
  // try to pull dimensions from command line
  let args: Vec<String> = env::args().collect();

  let mut height: usize = 32;
  let mut width: usize = 32;

  if (args.len() == 3) {
    width = args[1].parse::<usize>().unwrap();
    height = args[2].parse::<usize>().unwrap();
  }
  else if (args.len() > 1) {
    panic!("{:?}", "please provide 2 dimensions or no arguments");
  }

  let dimensions = grid::GridSize {
    height,
    width
  };

  let mut grid = grid::Grid::with_dimensions(dimensions);

  // debug - get all neighbors
  // for i in 0..grid.dimensions.area() {
  //   println!("for index={}, left_wall={}, right_wall={}, neighbors are={:?}", i, grid.is_left_wall(i), grid.is_right_wall(i), grid.get_neighbors_indexes(i));
  // }

  grid.randomize_grid();

  // get the randomized grid as base64
  let encoded = grid.to_base64();
  println!("encoded: {}", encoded);

  let mut game = Conway::with_grid(grid);
  let t0 = Instant::now();

  loop {
    let elapsed = t0.elapsed().as_millis();
    let steps_per_sec = game.steps as f64 / (elapsed as f64 / 1000.);
    println!("{}", game.grid);
    println!("steps: {}; elapsed: {}ms, steps/sec: {}", game.steps, elapsed, steps_per_sec);
    game.step();
  }
}
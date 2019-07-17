use std::env;
use std::sync::mpsc;
use std::time::Instant;
use std::thread;
use std::fs;

#[macro_use]
extern crate clap;
use clap::App;


mod grid;
mod conway;
mod octets;

pub const SEGMENT_SIZE: usize = size_of::<usize>();


use grid::{Grid, GridSize};
use conway::{Conway};

fn main() {
  let yaml = load_yaml!("cli.yaml");
  let matches = App::from_yaml(yaml).get_matches();

  let height = matches.value_of("height")
    .map_or(32, |val| val.parse::<usize>().unwrap());
  let width = matches.value_of("width")
    .map_or(32, |val| val.parse::<usize>().unwrap());
  let file = matches.value_of("file").unwrap_or("");
  let max_steps = matches.value_of("steps")
    .map_or(0, |val| val.parse::<usize>().unwrap());

  let dimensions = GridSize {
    height,
    width
  };

  // TODO: there's probably a 'lazier' way to do this,
  // only doing the default work if a file isn't specified
  let mut grid = Grid::with_dimensions(dimensions);
  grid.randomize_grid();

  if file.len() > 0 {
    // try to read file

    let b64_data = fs::read_to_string(file)
      .expect("Something went wrong reading your file.");
    grid = Grid::from_base64(&b64_data);
  }

  // TODO: register a handler to print the grid as base64 to stderr on signal

  let mut game = Conway::with_grid(grid);
  let t0 = Instant::now();

  loop {
    if max_steps > 0 && game.steps == max_steps {
      break;
    }

    let elapsed = t0.elapsed().as_millis();
    let steps_per_sec = game.steps as f64 / (elapsed as f64 / 1000.);
    println!("{}", game.grid);
    println!("steps: {}; elapsed: {}ms, steps/sec: {}", game.steps, elapsed, steps_per_sec);
    game.step();
  }
}
use std::fs;
use std::time::{Instant, Duration};
use std::thread::sleep;
use core::mem::size_of;

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
  let grid_file = matches.value_of("grid").unwrap_or("");
  let delay = matches.value_of("delay")
    .map_or(0, |val| val.parse::<u64>().unwrap());

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
  else if grid_file.len() > 0 {
    let grid_data = fs::read_to_string(grid_file)
      .expect("Something went wrong reading your grid file.");
    grid = Grid::from_grid_str(&grid_data);
  }

  // TODO: register a handler to print the grid as base64 to stderr on signal

  let mut game = Conway::with_grid(grid);
  let t0 = Instant::now();
  let delay_duration = Duration::from_millis(delay);

  loop {
    if max_steps > 0 && game.steps == max_steps {
      break;
    }

    let elapsed = t0.elapsed().as_millis();
    let steps_per_sec = game.steps as f64 / (elapsed as f64 / 1000.);
    print!("{}[2J", 27 as char);
    println!("{}", game.grid);
    println!("steps: {}; elapsed: {}ms, steps/sec: {}", game.steps, elapsed, steps_per_sec);
    game.step();

    if (delay > 0) {
      sleep(delay_duration);
    }
  }
}
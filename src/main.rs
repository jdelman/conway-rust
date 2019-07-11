use std::env;
use std::mem::size_of;
use rand::Rng;
use std::time::Instant;
use std::fmt;

const SEGMENT_SIZE: usize = size_of::<usize>();

#[derive(Debug, Copy, Clone)]
struct GridSize {
  height: usize,
  width: usize
}

impl GridSize {
  fn area(&self) -> usize {
    self.height * self.width
  }
}

struct Grid {
  dimensions: GridSize,
  grid: Vec<usize>
}

impl Grid {
  fn with_dimensions(dimensions: GridSize) -> Grid {
    let grid = Grid::zero_grid(&dimensions);

    Grid {
      dimensions,
      grid
    }
  }

  fn new_zero_grid(&self) -> Grid {
    let next_dimensions = self.dimensions;
    let grid = Grid::zero_grid(&next_dimensions);

    Grid {
      dimensions: next_dimensions,
      grid
    }
  }

  fn zero_grid(dimensions: &GridSize) -> Vec<usize> {
    // this isn't perfect, it won't work for boards smaller than (8 * usize) in area
    let mut required_segments = dimensions.area() / 8 / SEGMENT_SIZE;

    // if the area isn't a power of 2, add one, since integer devision will round down 
    if !required_segments.is_power_of_two() {
      required_segments += 1;
    }

    return vec![0; required_segments];
  }

  fn randomize_grid(&mut self) {
    let mut rng = rand::thread_rng();
    for i in 0..self.dimensions.area() {
      if rng.gen() {
        self.assign_value(i, true);
      }
      else {
        self.assign_value(i, false);
      }
    }
  }

  fn assign_value(&mut self, index: usize, value: bool) {
    let segment = index / (8 * SEGMENT_SIZE);
    let offset = index % (8 * SEGMENT_SIZE);

    if value {
      self.grid[segment] |= 1 << offset;
    }
    else {
      self.grid[segment] &= !(1 << offset);
    }
  }

  fn get_value(&self, index: usize) -> bool {
    let segment = index / (8 * SEGMENT_SIZE);
    let offset = index % (8 * SEGMENT_SIZE);
    return self.grid[segment] & (1 << offset) != 0;
  }

  fn is_left_wall(&self, index: usize) -> bool {
    index % self.dimensions.width == 0
  }

  fn is_right_wall(&self, index: usize) -> bool {
    index != 0 && (index + 1) % self.dimensions.width == 0
  }

  fn get_neighbors_indexes(&self, index: usize) -> Vec<usize> {
    let possibles = vec![
      index.checked_sub(self.dimensions.width - 1),
      index.checked_sub(self.dimensions.width),
      index.checked_sub(self.dimensions.width + 1),
      index.checked_sub(1),
      index.checked_add(1),
      index.checked_add(self.dimensions.width - 1),
      index.checked_add(self.dimensions.width),
      index.checked_add(self.dimensions.width + 1)
    ];

    let mut neighbors: Vec<usize> = Vec::with_capacity(8);

    for possible in possibles {
      if let Some(value) = possible {
        // don't add indexes from opposite walls
        let should_skip_value =
          value >= self.dimensions.area() ||
          (self.is_left_wall(index) && self.is_right_wall(value)) ||
          (self.is_right_wall(index) && self.is_left_wall(value))
        ;

        if !should_skip_value {
          neighbors.push(value);
        }
      }
    }

    return neighbors;
  }

  fn print_indexes(&self) {
    let mut gstr = String::new();

    for i in 0..self.dimensions.area() {
      if i > 0 && i % self.dimensions.width == 0 {
        gstr += "\n";
      }

      let digit_str = format!("{: >5} ", i);
      gstr += &digit_str;
    }

    println!("{}", gstr);
  }
      }
    }

  }

impl fmt::Display for Grid {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // total size is equal to dimensions plus a new line for each line

    let string_size = (self.dimensions.width + 1) * self.dimensions.height;
    let mut gstr = String::with_capacity(string_size);

    for i in 0..self.dimensions.area() {
      if i != 0 && i % self.dimensions.width == 0 {
        gstr += "\n";
      }

      gstr += match self.get_value(i) {
        true => "█",
        false => "·"
      }
    }

    write!(f, "{}", gstr)
  }
}

struct Conway {
  grid: Grid,
  steps: usize
}

impl Conway {
  fn with_grid(grid: Grid) -> Conway {
    Conway {
      grid,
      steps: 0
    }
  }

  fn step(&mut self) {
    // create a new grid
    let mut new_grid = self.grid.new_zero_grid();

    for i in 0..self.grid.dimensions.area() {
      let is_alive = self.grid.get_value(i) == true;
      let neighbors = self.grid.get_neighbors_indexes(i);
      let mut alive_neighbors = 0;
      for index in neighbors {
        alive_neighbors += match self.grid.get_value(index) {
          true => 1,
          _ => 0
        };
      }

      // conway rules
      if is_alive && (alive_neighbors == 2 || alive_neighbors == 3) {
        new_grid.assign_value(i, true);
      }
      else if !is_alive && alive_neighbors == 3 {
        new_grid.assign_value(i, true);
      }
    }

    self.grid = new_grid;
    self.steps += 1;
  }

}

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

  let dimensions = GridSize {
    height,
    width
  };

  let mut grid = Grid::with_dimensions(dimensions);

  // debug - get all neighbors
  // for i in 0..grid.dimensions.area() {
  //   println!("for index={}, left_wall={}, right_wall={}, neighbors are={:?}", i, grid.is_left_wall(i), grid.is_right_wall(i), grid.get_neighbors_indexes(i));
  // }

  grid.randomize_grid();

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
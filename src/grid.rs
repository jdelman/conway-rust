use rand::Rng;
use base64;
use std::fmt;
use std::mem::size_of;

use crate::octets as octets;

const SEGMENT_SIZE: usize = size_of::<usize>();

#[derive(Debug, Copy, Clone)]
pub struct GridSize {
  pub height: usize,
  pub width: usize
}

impl GridSize {
  pub fn area(&self) -> usize {
    self.height * self.width
  }
}

pub struct Grid {
  pub dimensions: GridSize,
  pub grid: Vec<usize>
}

impl Grid {
  pub fn with_dimensions(dimensions: GridSize) -> Grid {
    let grid = Grid::zero_grid(&dimensions);

    Grid {
      dimensions,
      grid
    }
  }

  pub fn new_zero_grid(&self) -> Grid {
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

    vec![0; required_segments]
  }

  pub fn randomize_grid(&mut self) {
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

  pub fn assign_value(&mut self, index: usize, value: bool) {
    let segment = index / (8 * SEGMENT_SIZE);
    let offset = index % (8 * SEGMENT_SIZE);

    if value {
      self.grid[segment] |= 1 << offset;
    }
    else {
      self.grid[segment] &= !(1 << offset);
    }
  }

  pub fn get_value(&self, index: usize) -> bool {
    let segment = index / (8 * SEGMENT_SIZE);
    let offset = index % (8 * SEGMENT_SIZE);
    self.grid[segment] & (1 << offset) != 0
  }

  fn is_left_wall(&self, index: usize) -> bool {
    index % self.dimensions.width == 0
  }

  fn is_right_wall(&self, index: usize) -> bool {
    index != 0 && (index + 1) % self.dimensions.width == 0
  }

  pub fn get_neighbors_indexes(&self, index: usize) -> Vec<usize> {
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

    neighbors
  }

  pub fn print_indexes(&self) {
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

  pub fn to_base64(&self) -> String {
    // we have to break up usize integers
    // into u8s should be 8 ints
    let count = (self.dimensions.area() * SEGMENT_SIZE) + 2;
    let mut ints: Vec<u8> = Vec::with_capacity(count);

    // first two values should be the dimensions
    ints.append(&mut (octets::to_octets(self.dimensions.width)));
    ints.append(&mut (octets::to_octets(self.dimensions.height)));

    // now grid values
    for index in 0..self.grid.len() {
      let value = self.grid[index];
      let mut octets = octets::to_octets(value);
      ints.append(&mut octets);
    }

    let str_out = base64::encode_config(&ints, base64::STANDARD);

    str_out
  }

  pub fn from_base64(b64str: &str) -> Grid {
    let octets = base64::decode_config(b64str, base64::STANDARD).unwrap();

    // pull off the dimensions
    let width = octets::from_octets(&octets[0..SEGMENT_SIZE]);
    let height = octets::from_octets(&octets[SEGMENT_SIZE..(2 * SEGMENT_SIZE)]);
    let dimensions = GridSize {
      width,
      height
    };

    let mut new_grid = Grid::with_dimensions(dimensions);

    // we pushed onto the array in order, from least
    // to most significant bits for each usize
    let iters = (octets.len() - 2) / SEGMENT_SIZE;
    let mut index = 0;
    for i in 0..iters {
      // build up an array of SEGMENT_SIZE (8) octets
      let start = (i * SEGMENT_SIZE) + (i + 2);
      let end = (i * SEGMENT_SIZE) + (i + 2) + SEGMENT_SIZE;
      let value = octets::from_octets(&octets[start..end]);
      new_grid.grid[index] = value;
      index += 1;
    }

    new_grid
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
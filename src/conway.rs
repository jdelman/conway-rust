use crate::grid::Grid;

pub struct Conway {
  pub grid: Grid,
  pub steps: usize
}

impl Conway {
  pub fn with_grid(grid: Grid) -> Conway {
    Conway {
      grid,
      steps: 0
    }
  }

  pub fn step_cell(&self, i: usize) -> bool {
    let is_alive = self.grid.get_value(i) == true;
    let neighbors = self.grid.get_neighbors_indexes(i);
    let mut alive_neighbors = 0;
    for index in neighbors {
      alive_neighbors += match self.grid.get_value(index) {
        true => 1,
        _ => 0
      };
    }

    if is_alive && (alive_neighbors == 2 || alive_neighbors == 3) {
      return true;
    }
    else if !is_alive && alive_neighbors == 3 {
      return true;
    }

    return false;
  }

  pub fn step(&mut self) {
    // create a new grid
    let mut new_grid = self.grid.new_zero_grid();

    for i in 0..self.grid.dimensions.area() {
      let is_cell_alive_in_next_gen = self.step_cell(i);
      if is_cell_alive_in_next_gen {
        new_grid.set_value(i, true);
      }
    }

    self.grid = new_grid;
    self.steps += 1;
  }

  // fn step_threaded(&mut self, n: usize) {
  //   // use a multithreaded approach to generate a step
  //   let mut new_grid = self.grid.new_zero_grid();

  //   // create a channel and transfer pool
  //   let (tx, rx) = mspc::channel();
  //   let mut txs = VecDeque<mspc::Sender>::with_capacity(n);
  //   for i in 0..n {
  //     let tx_n = mpsc::Sender::clone(&tx);
  //     txs.push_back(tx_n);
  //   }

  //   // let mut remaining_cells = self.grid.dimensions.area();
  //   let 

  //   while remaining_cells >= 0 {
  //     if let Some(tx_n) = txs.pop_front() {
  //       // spawn a thread
  //       let iself = &self;
  //       thread::spawn(move || {
  //         iself.
  //       });
  //     }
  //     else {
  //       // join on the open threads, set the values, then repeat
        
  //     }
  //   }
  // }

}
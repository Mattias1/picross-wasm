pub struct Puzzle {
  pub name: String,
  answers: Vec<Vec<i32>>,
  values: Vec<Vec<i32>>,
  max_col_count: i32,
}

impl Puzzle {
  pub fn build(name: &str, puzzle: &Vec<Vec<i32>>) -> Puzzle {
    let height = if let Some(v) = puzzle.get(0) { v.len() } else { 0 };
    Puzzle {
      name: String::from(name),
      answers: puzzle.to_vec(),
      values: vec![vec![0; height]; puzzle.len()],
      max_col_count: 0
    }
  }

  pub fn width(&self) -> usize {
    if let Some(v) = self.values.get(0) { v.len() } else { 0 }
  }

  pub fn height(&self) -> usize {
    self.values.len()
  }

  pub fn get(&self, x: usize, y: usize) -> i32 {
    self.values[y][x]
  }

  pub fn get_max_col_count_so_far(&self) -> i32 {
    self.max_col_count
  }

  pub fn get_class_name(&self, x: usize, y: usize) -> String {
    match self.get(x, y) {
      0 => String::from(""),
      1 => String::from("filled"),
      2 => String::from("empty"),
      _ => String::from("unknown")
    }
  }

  pub fn fill_at(&mut self, x: usize, y: usize) {
    self.values[y][x] = 1;
  }

  pub fn empty_at(&mut self, x: usize, y: usize) {
    self.values[y][x] = 2;
  }

  pub fn clear_at(&mut self, x: usize, y: usize) {
    self.values[y][x] = 0;
  }

  pub fn row_nrs(&self, y: usize) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut curr = 0;
    for x in 0..self.width() {
      if self.answers[y][x] == 1 {
        curr += 1;
      } else if curr > 0 {
        result.push(curr);
        curr = 0;
      }
    }
    if curr > 0 {
      result.push(curr);
    }
    result
  }

  pub fn col_nrs(&mut self, x: usize) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut count = 0;
    let mut curr = 0;
    for y in 0..self.height() {
      if self.answers[y][x] == 1 {
        curr += 1;
      } else if curr > 0 {
        result.push(curr);
        count += 1;
        curr = 0;
      }
    }
    if curr > 0 {
      result.push(curr);
      count += 1;
    }
    if count > self.max_col_count {
      self.max_col_count = count;
    }
    result
  }

  pub fn is_solved(&self) -> bool {
    for y in 0..self.height() {
      for x in 0..self.width() {
        if self.answers[y][x] == 0 && self.values[y][x] == 1 {
          return false
        } else if self.answers[y][x] == 1 && self.values[y][x] != 1 {
          return false
        }
      }
    }
    true
  }
}

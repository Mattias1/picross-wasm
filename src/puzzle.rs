pub struct Puzzle {
  pub name: String,
  answers: Vec<Vec<i32>>,
  values: Vec<Vec<i32>>,
}

impl Puzzle {
  pub fn build(name: &str, puzzle: &Vec<Vec<i32>>) -> Puzzle {
    let height = if let Some(v) = puzzle.get(0) { v.len() } else { 0};
    Puzzle {
      name: String::from(name),
      answers: puzzle.to_vec(),
      values: vec![vec![0; height]; puzzle.len()]
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

  pub fn get_class_name(&self, x: usize, y: usize) -> String {
    match self.get(x, y) {
      0 => String::from(""),
      1 => String::from("filled"),
      2 => String::from("empty"),
      _ => String::from("unknown")
    }
  }

  pub fn toggle_filled(&mut self, x: usize, y: usize) {
    self.values[y][x] = if self.values[y][x] == 1 { 0 } else { 1 };
  }

  pub fn toggle_empty(&mut self, x: usize, y: usize) {
    self.values[y][x] = if self.values[y][x] == 2 { 0 } else { 2 };
  }

  pub fn row_nrs(&self, y: usize) -> String {
    let mut result = String::new();
    let mut curr = 0;
    for x in 0..self.width() {
      if self.answers[y][x] == 1 {
        curr += 1;
      } else if curr > 0 {
        result += &curr.to_string();
        result.push(' ');
        curr = 0;
      }
    }
    if curr > 0 {
      result += &curr.to_string();
    }
    result
  }

  pub fn col_nrs(&self, x: usize) -> String {
    let mut result = String::new();
    let mut curr = 0;
    for y in 0..self.height() {
      if self.answers[y][x] == 1 {
        curr += 1;
      } else if curr > 0 {
        result += &curr.to_string();
        result.push('\n');
        curr = 0;
      }
    }
    if curr > 0 {
      result += &curr.to_string();
    }
    result
  }
}

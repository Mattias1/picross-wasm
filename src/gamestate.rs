use super::puzzle::Puzzle;
use super::puzzle_list::PuzzleList;

pub struct GameState {
  current_puzzle: Option<Puzzle>,
  mouse_down: bool,
  ctrl_down: bool,
  flip_ctrl: bool,
}

impl GameState {
  pub fn build() -> GameState {
    GameState {
      current_puzzle: None,
      mouse_down: false,
      ctrl_down: false,
      flip_ctrl: false,
    }
  }

  pub fn load_puzzle(&mut self, name: &str) {
    self.current_puzzle = PuzzleList::build_for_name(name);
  }

  pub fn get_puzzle(&mut self) -> &mut Puzzle {
    if let Some(pzl) = &mut self.current_puzzle { pzl } else { panic!("No puzzle is loaded.") }
  }

  pub fn is_mouse_down(&self) -> bool {
    self.mouse_down
  }
  pub fn click_mouse(&mut self) {
    self.mouse_down = true;
  }
  pub fn release_mouse(&mut self) {
    self.mouse_down = false;
  }

  pub fn is_ctrl_down(&self) -> bool {
    self.ctrl_down ^ self.flip_ctrl
  }
  pub fn press_ctrl(&mut self) {
    self.ctrl_down = true;
  }
  pub fn release_ctrl(&mut self) {
    self.ctrl_down = false;
  }
  pub fn flip_ctrl(&mut self) {
    self.flip_ctrl = true;
  }
  pub fn unflip_ctrl(&mut self) {
    self.flip_ctrl = false;
  }
}
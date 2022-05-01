extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement, KeyboardEvent};

mod puzzle;
mod puzzle_list;
mod gamestate;

use puzzle::Puzzle;
use gamestate::GameState;

// For debugging purposes
/*
#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}
*/

static mut GAMESTATE: Option<GameState> = None;

#[wasm_bindgen]
pub fn load_puzzle(name: &str) {
  load_puzzle_with_button_class(name, "");
}

#[wasm_bindgen]
pub fn load_tutorial_puzzle(brush_button_class: &str) {
  load_puzzle_with_button_class("Tutorial stripe", brush_button_class);

  let gamestate = get_gamestate();

  // Last 4-row
  toggle_square(gamestate, 1, 3);
  toggle_square(gamestate, 2, 3);
  toggle_square(gamestate, 3, 3);

  // First 3-column
  gamestate.press_ctrl();
  toggle_square(gamestate, 1, 0);
  gamestate.release_ctrl();
  toggle_square(gamestate, 1, 2);

  // (First) 2-row
  gamestate.press_ctrl();
  toggle_square(gamestate, 0, 0);
  gamestate.release_ctrl();
  toggle_square(gamestate, 3, 0);
}

#[wasm_bindgen]
pub fn load_puzzle_with_button_class(name: &str, brush_button_class: &str) {
  let gamestate = get_gamestate();
  gamestate.load_puzzle(name);
  let puzzle = gamestate.get_puzzle();
  let document = get_document();

  add_board_html(&document, puzzle);

  add_brush_button_html(&document, brush_button_class);

  add_gamestate_modifiers(&document);
}

fn add_board_html(document: &Document, puzzle: &mut Puzzle) {
  let parent = get_el(&document, "picross-wasm-player");
  parent.set_inner_html("");

  let mut nr_index = 1;

  let ul = append_el(&document, &parent, "ul", None, Some("pxw-ul"));

  let li = append_el(&document, &ul, "li", None, None);

  let empty_space_div = append_el(&document, &li, "div", Some("pxw-empty-space"), Some("pxw-empty-space"));
  for x in 0..puzzle.width() {
    let col_div = append_el(&document, &li, "div", Some(&format!("pxw-col-{}", x)), Some("pxw-el pxw-col"));
    for nr in &puzzle.col_nrs(x) {
      let nr_span = append_el(&document, &col_div, "span", Some(&format!("pxw-nr-{}", nr_index)), Some("pxw-nr"));
      add_nr_span_toggle(&nr_span);
      nr_span.set_inner_text(&nr.to_string());
      nr_index += 1;
    }
  }

  for y in 0..puzzle.height() {
    let li = append_el(&document, &ul, "li", None, None);

    let row_div = append_el(&document, &li, "div", Some(&format!("pxw-row-{}", y)), Some("pxw-el pxw-row"));
    for nr in &puzzle.row_nrs(y) {
      let nr_span = append_el(&document, &row_div, "span", Some(&format!("pxw-nr-{}", nr_index)), Some("pxw-nr"));
      add_nr_span_toggle(&nr_span);
      nr_span.set_inner_text(&nr.to_string());
      nr_index += 1;
    }

    for x in 0..puzzle.width() {
      let el = append_el(&document, &li, "button", Some(&format!("pxw-square-{}-{}", x, y)), Some("pxw-el pxw-square"));
      add_puzzle_onclick(&el, x, y);
    }
  }

  set_size_class_based_on_column_size(&ul, puzzle);

  let congratz_div = append_el(&document, &empty_space_div, "div", Some("pxw-congratz"), None);
  congratz_div.set_inner_text("Congratulations!");
}

fn add_puzzle_onclick(el: &HtmlElement, x: usize, y: usize) {
  let mouseenter_func = Closure::wrap(Box::new(move || {
    let gamestate = get_gamestate();
    if gamestate.is_mouse_down() {
      toggle_square(gamestate, x, y);
    }
  }) as Box<dyn FnMut()>);

  let click_func = Closure::wrap(Box::new(move || {
    let gamestate = get_gamestate();
    toggle_square(gamestate, x, y);
  }) as Box<dyn FnMut()>);

  el.set_draggable(false);

  el.set_onmouseenter(Some(mouseenter_func.as_ref().unchecked_ref()));
  el.set_onmousedown(Some(click_func.as_ref().unchecked_ref()));

  // Deallocate manually - DOM-closures are ugly :/
  mouseenter_func.forget();
  click_func.forget();
}

fn toggle_square(gamestate: &mut GameState, x: usize, y: usize) {
  let is_ctrl_down = gamestate.is_ctrl_down();
  let puzzle = gamestate.get_puzzle();
  if is_ctrl_down {
    puzzle.toggle_empty(x, y);
  } else {
    puzzle.toggle_filled(x, y);
  }
  let class_name = puzzle.get_class_name(x, y);

  let document = get_document();
  let el = get_el(&document, &format!("pxw-square-{}-{}", x, y));
  el.set_class_name(&format!("pxw-el pxw-square {}", class_name));

  let congratz_el = get_el(&document, "pxw-congratz");
  congratz_el.set_class_name(if puzzle.is_solved() { "visible" } else { "" });
}

fn add_nr_span_toggle(el: &HtmlElement) {
  let el_id = el.id();

  let click_func = Closure::wrap(Box::new(move || {
    let document = get_document();
    let el = get_el(&document, &el_id);
    if el.class_name().contains("pxw-nr-marked") {
      el.set_class_name("pxw-nr");
    } else {
      el.set_class_name("pxw-nr pxw-nr-marked");
    }
  }) as Box<dyn FnMut()>);

  el.set_onclick(Some(click_func.as_ref().unchecked_ref()));

  // Deallocate manually - DOM-closures are ugly :/
  click_func.forget();
}

fn set_size_class_based_on_column_size(el: &HtmlElement, puzzle: &Puzzle) {
  let mut size_class = "";
  if puzzle.get_max_col_count_so_far() <= 3 {
    size_class = "small";
  } else if puzzle.get_max_col_count_so_far() > 5 {
    size_class = "large";
  }
  el.set_class_name(&format!("{} {}", el.class_name(), size_class));
}

fn add_brush_button_html(document: &Document, brush_button_class: &str) {
  let initial_button_el = get_el_optional(document, "picross-wasm-toggle-brush")
    .unwrap_or_else(|| append_brush_button_el(document, brush_button_class));
  initial_button_el.set_inner_html("&#10002;");

  let click_func = Closure::wrap(Box::new(move || {
    let button_el = get_el(&get_document(), "picross-wasm-toggle-brush");
    let gamestate = get_gamestate();

    if button_el.inner_text() == "-" {
      gamestate.unflip_ctrl();
      button_el.set_inner_html("&#10002;");
    } else {
      gamestate.flip_ctrl();
      button_el.set_inner_text("-");
    }
  }) as Box<dyn FnMut()>);

  initial_button_el.set_onmousedown(Some(click_func.as_ref().unchecked_ref()));

  // Deallocate manually - DOM-closures are ugly :/
  click_func.forget();
}

fn append_brush_button_el(document: &Document, class_name: &str) -> HtmlElement {
  let parent = get_el(document, "pxw-empty-space");
  let button_el = append_el(document, &parent, "button", Some("picross-wasm-toggle-brush"), Some(class_name));
  let text_el = append_el(document, &parent, "span", None, None);
  text_el.set_inner_text("(or hold ctrl)");
  button_el
}

fn add_gamestate_modifiers(document: &Document) {
  let mousedown_func = Closure::wrap(Box::new(move || {
    let gamestate = get_gamestate();
    gamestate.click_mouse();
  }) as Box<dyn FnMut()>);

  let mouseup_func = Closure::wrap(Box::new(move || {
    let gamestate = get_gamestate();
    gamestate.release_mouse();
  }) as Box<dyn FnMut()>);

  let keydown_func = Closure::wrap(Box::new(move |event: KeyboardEvent| {
    if event.ctrl_key() {
      let gamestate = get_gamestate();
      gamestate.press_ctrl();
    }
  }) as Box<dyn FnMut(_)>);

  let keyup_func = Closure::wrap(Box::new(move |event: KeyboardEvent| {
    if !event.ctrl_key() {
      let gamestate = get_gamestate();
      gamestate.release_ctrl();
    }
  }) as Box<dyn FnMut(_)>);

  document.set_onmousedown(Some(mousedown_func.as_ref().unchecked_ref()));
  document.set_onmouseup(Some(mouseup_func.as_ref().unchecked_ref()));

  document.set_onkeydown(Some(keydown_func.as_ref().unchecked_ref()));
  document.set_onkeyup(Some(keyup_func.as_ref().unchecked_ref()));

  // Deallocate manually - DOM-closures are ugly :/
  mousedown_func.forget();
  mouseup_func.forget();
  keydown_func.forget();
  keyup_func.forget();
}

fn get_el(document: &Document, id: &str) -> HtmlElement {
  get_el_optional(document, id)
    .expect(&format!("should have #{} on the page", id))
}

fn get_el_optional(document: &Document, id: &str) -> Option<HtmlElement> {
  document
    .get_element_by_id(id)
    .map(|el| el
      .dyn_into::<HtmlElement>() // dyn_into gives the struct, dyn_ref gives a reference
      .expect(&format!("#{} should be an HtmlElement", id))
    )
}

fn append_el(document: &Document, parent: &HtmlElement, el: &str, id: Option<&str>, class: Option<&str>)
    -> HtmlElement {
  let val = document.create_element(el).expect("Creation error")
    .dyn_into::<HtmlElement>() // dyn_into gives the struct, dyn_ref gives a reference
    .expect(&format!("#{} should be an HtmlElement", el));
  if let Some(id_val) = id {
    val.set_id(id_val);
  }
  if let Some(class_val) = class {
    val.set_class_name(class_val);
  }
  parent.append_child(&val).expect("Append error");
  val
}

fn get_gamestate() -> &'static mut GameState {
  unsafe {
    if let None = GAMESTATE {
      GAMESTATE = Some(GameState::build());
    }
    if let Some(gamestate) = &mut GAMESTATE { gamestate } else { panic!("Gamestate not set") }
  }
}

fn get_document() -> Document {
  let window = web_sys::window().expect("no global window exists");
  window.document().expect("should have a document on window")
}

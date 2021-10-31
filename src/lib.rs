extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

mod puzzle;
use puzzle::Puzzle;
mod puzzle_list;
mod gamestate;
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
  let gamestate = get_gamestate();
  gamestate.load_puzzle(name);
  let puzzle = gamestate.get_puzzle();
  let document = get_document();

  add_mouse_up_down(&document);

  let parent = get_el(&document, "picross-wasm-player");
  let ul = append_el(&document, &parent, "ul", None, Some("pxw-ul"));

  let li = append_el(&document, &ul, "li", None, None);

  append_el(&document, &li, "div", None, Some("pxw-empty-space"));
  for x in 0..puzzle.width() {
    let col_div = append_el(&document, &li, "div", Some(&format!("pxw-col-{}", x)), Some("pxw-el pxw-col"));
    col_div.set_inner_text(&puzzle.col_nrs(x));
  }

  for y in 0..puzzle.height() {
    let li = append_el(&document, &ul, "li", None, None);

    let row_div = append_el(&document, &li, "div", Some(&format!("pxw-row-{}", y)), Some("pxw-el pxw-row"));
    row_div.set_inner_text(&puzzle.row_nrs(y));

    for x in 0..puzzle.width() {
      let el = append_el(&document, &li, "button", Some(&format!("pxw-square-{}-{}", x, y)), Some("pxw-el pxw-square"));
      add_puzzle_onclick(&el, x, y);
    }
  }
}

fn add_mouse_up_down(document: &Document) {
  let mousedown_func = Closure::wrap(Box::new(move || {
    let gamestate = get_gamestate();
    gamestate.click_mouse();
  }) as Box<dyn FnMut()>);

  let mouseup_func = Closure::wrap(Box::new(move || {
    let gamestate = get_gamestate();
    gamestate.release_mouse();
  }) as Box<dyn FnMut()>);

  document.set_onmousedown(Some(mousedown_func.as_ref().unchecked_ref()));
  document.set_onmouseup(Some(mouseup_func.as_ref().unchecked_ref()));

  // Deallocate - DOM-closures are ugly :/
  mousedown_func.forget();
  mouseup_func.forget();
}

fn add_puzzle_onclick(el: &HtmlElement, x: usize, y: usize) {
  let mouseenter_func = Closure::wrap(Box::new(move || {
    let gamestate = get_gamestate();
    if gamestate.is_mouse_down() {
      toggle_square(gamestate.get_puzzle(), x, y);
    }
  }) as Box<dyn FnMut()>);

  let click_func = Closure::wrap(Box::new(move || {
    let gamestate = get_gamestate();
    toggle_square(gamestate.get_puzzle(), x, y);
  }) as Box<dyn FnMut()>);

  el.set_draggable(false);

  el.set_onmouseenter(Some(mouseenter_func.as_ref().unchecked_ref()));
  el.set_onmousedown(Some(click_func.as_ref().unchecked_ref()));

  // Deallocate - DOM-closures are ugly :/
  mouseenter_func.forget();
  click_func.forget();
}

fn toggle_square(puzzle: &mut Puzzle, x: usize, y: usize) {
    puzzle.toggle(x, y);
    let class_name = puzzle.get_class_name(x, y);

    let document = get_document();
    let el = get_el(&document, &format!("pxw-square-{}-{}", x, y));
    el.set_class_name(&format!("pxw-el pxw-square {}", class_name));
}

fn get_el(document: &Document, id: &str) -> HtmlElement {
  return document
    .get_element_by_id(id)
    .expect(&format!("should have #{} on the page", id))
    .dyn_into::<HtmlElement>() // dyn_into gives the struct, dyn_ref gives a reference
    .expect(&format!("#{} should be an HtmlElement", id));
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

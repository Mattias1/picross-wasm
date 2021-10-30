extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

mod puzzle;
use puzzle::Puzzle;

// For debugging purposes
/*
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
*/ 

static mut PUZZLE_STATIC: Option<Puzzle> = None;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  // Using unsafe because we have a static mutable puzzle object
  unsafe {
    PUZZLE_STATIC = Some(Puzzle::build(&vec![
      vec![0, 1, 0, 0],
      vec![1, 1, 1, 1],
      vec![0, 1, 0, 1],
      vec![1, 1, 0, 0]
    ]));
  }

  init_board();

  Ok(())
}

fn init_board() {
  let puzzle = get_puzzle();
  let document = get_document();

  let parent = get_el(&document, "picross-wasm-player");
  let ul = append_el(&document, &parent, "ul", None, Some("pxw-ul"));

  let li = append_el(&document, &ul, "li", None, None);

  for x in 0..puzzle.width() {
    let col_div = append_el(&document, &li, "div", Some(&format!("pxw-col-{}", x)), Some("pxw-col"));
    col_div.set_inner_text(&puzzle.col_nrs(x));
  }

  for y in 0..puzzle.height() {
    let li = append_el(&document, &ul, "li", None, None);

    let row_div = append_el(&document, &li, "div", Some(&format!("pxw-row-{}", y)), Some("pxw-row"));
    row_div.set_inner_text(&puzzle.row_nrs(y));

    for x in 0..puzzle.width() {
      let el = append_el(&document, &li, "a", Some(&format!("pxw-square-{}-{}", x, y)), Some("pxw-square"));
      add_puzzle_onclick(&el, x, y);
    }
  }
}

fn add_puzzle_onclick(el: &HtmlElement, x: usize, y: usize) {
  el.set_attribute("href", "#").expect("Href error");

  let func = Closure::wrap(Box::new(move || {
    let puzzle = get_puzzle();
    let document = get_document();

    puzzle.toggle(x, y);
    let class_name = puzzle.get_class_name(x, y);

    let el = get_el(&document, &format!("pxw-square-{}-{}", x, y));
    el.set_class_name(&format!("pxw-square {}", class_name));
  }) as Box<dyn FnMut()>);

  el.set_onclick(Some(func.as_ref().unchecked_ref()));

  // Deallocate, dom-closures are ugly :/
  func.forget();
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

fn get_puzzle() -> &'static mut Puzzle {
  // Using unsafe because we have a static mutable puzzle object
  unsafe {
    if let Some(pzl) = &mut PUZZLE_STATIC { pzl } else { panic!("The puzzle is not set") }
  }
}

fn get_document() -> Document {
  let window = web_sys::window().expect("no global window exists");
  window.document().expect("should have a document on window")
}

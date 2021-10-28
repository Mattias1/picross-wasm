extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  let window = web_sys::window().expect("no global window exists");
  let document = window.document().expect("should have a document on window");

  let puzzle: [[u8; 4]; 4] = [
    [0, 1, 0, 0],
    [1, 1, 1, 1],
    [0, 1, 0, 0],
    [0, 1, 0, 0]
  ];

  init_board(&document, &puzzle);

  Ok(())
}

fn init_board(document: &Document, puzzle: &[[u8; 4]; 4]) {
  let parent = get_el(&document, "picross-wasm-player");
  let ul = append_el(document, &parent, "ul", None, Some("pxw-ul"));

  let li = append_el(document, &ul, "li", None, None);
  append_el(document, &li, "div", None, Some("pxw-space"));
  for x in 0..puzzle[0].len() {
    append_el(document, &li, "div", Some(&format!("pxw-col-{}", x)), Some("pxw-col"));
  }

  for (y, row) in puzzle.iter().enumerate() {
    let li = append_el(document, &ul, "li", None, None);
    append_el(document, &li, "div", Some(&format!("pxw-row-{}", y)), Some("pxw-row"));
    for (x, _) in row.iter().enumerate() {
      append_el(document, &li, "a", Some(&format!("pxw-square-{}-{}", x, y)), Some("pxw-square"));
    }
  }
}

fn get_el(document: &Document, id: &str) -> HtmlElement {
  return document
    .get_element_by_id(id)
    .expect(&format!("should have #{} on the page", id))
    .dyn_into::<HtmlElement>() // dyn_into gives the struct, dun_ref gives a reference
    .expect(&format!("#{} should be an HtmlElement", id));
}

fn append_el(document: &Document, parent: &HtmlElement, el: &str, id: Option<&str>, class: Option<&str>)
    -> HtmlElement {
  let val = document.create_element(el).expect("Creation error")
    .dyn_into::<HtmlElement>() // dyn_into gives the struct, dun_ref gives a reference
    .expect(&format!("#{} should be an HtmlElement", el));
  if let Some(id_val) = id {
    val.set_id(id_val);
  }
  if let Some(class_val) = class {
    val.set_class_name(class_val);
  }
  if el == "a" {
    val.set_attribute("href", "#").expect("Href error");
  }
  parent.append_child(&val).expect("Append error");
  val
}

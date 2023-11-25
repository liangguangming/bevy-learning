use main::web_startup;
use wasm_bindgen::prelude::*;

pub mod main;
pub mod bridge;

#[wasm_bindgen]
pub fn wasm_main() {
  web_startup();
}

use wasm_bindgen::prelude::*;

pub mod main;
pub mod bridge;

use crate::main::main;
use bridge::message::send_msg_to_js;

#[wasm_bindgen]
pub fn wasm_main() {
  send_msg_to_js("Hello js, I am from wasm");
  main(); // 这里会抛出错误，不要把逻辑写在这一行的下面
}

use wasm_bindgen::prelude::*;
use js_sys::*;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
  a + b
}

// #[wasm_bindgen]
// pub fn onMessage(jsCallback: ) {
//   if (jsCallback.is_function()) {
//     unsafe {
//       JS_RECEIVE_MSG_CALLBACK = jsCallback;
//     }
//   }
// }

#[wasm_bindgen(module = "/src/js/message.js")]
extern {
    fn receive_msg_from_wasm(s: &str); // 内部自治
}

#[wasm_bindgen]
pub fn send_msg_to_wasm(s: &str) {
  receive_msg(s);
}

pub fn send_msg_to_js(s: &str) {
  receive_msg_from_wasm(s);
}

pub fn receive_msg(s: &str) {
  bevy::log::info!("receive web str: {}", s);
}
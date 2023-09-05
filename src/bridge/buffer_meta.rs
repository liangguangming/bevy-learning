use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
pub struct BufferMeta {
  pub ptr: *const u8,
  pub len: usize
}

// #[wasm_bindgen]
// impl BufferMeta {
//     BufferMeta(p: *const u8, l: usize) {

//     }
//     #[wasm_bindgen(getter)]
//     pub fn ptr(&self) -> *const u8 {
//         self.ptr
//     }

//     #[wasm_bindgen(getter)]
//     pub fn len(&self) -> usize {
//         self.len
//     }
// }
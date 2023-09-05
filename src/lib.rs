use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

pub mod main;
pub mod bridge;

use crate::main::main;
use bridge::message::{send_msg_to_js, send_msg_to_internal_js};
use bridge::buffer_meta::BufferMeta;
use bridge::js_global_storage::{ add_heap_object_by_buffer_meta, get_heap_object };

#[wasm_bindgen]
pub fn wasm_main() {
  send_msg_to_js("Hello js, I am from wasm");
  send_msg_to_internal_js("Hello js, send to internal js");
  let v = vec![1,2,3,4,5];
  let bufferMeta = BufferMeta {
    ptr: v.as_ptr(),
    len: v.len()
  };
  let idx = add_heap_object_by_buffer_meta(bufferMeta);
  let buffer = get_heap_object(idx).dyn_into::<Uint8Array>().unwrap().to_vec(); // 读取出来
  
  main(); // 这里会抛出错误，不要把逻辑写在这一行的下面
}

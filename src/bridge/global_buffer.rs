use wasm_bindgen::prelude::*;
use std::sync::Mutex;
use std::collections::HashMap;
use once_cell::sync::Lazy;

use super::buffer_meta::BufferMeta;

pub struct BufferStorage {
  pub buffer_map: HashMap<String, Vec<u8>>
}

impl BufferStorage {
    fn new() -> Self {
      BufferStorage {
        buffer_map: HashMap::new()
      }
    }
}

pub static GLOBAL_BUFFER_STORAGE: Lazy<Mutex<BufferStorage>> = Lazy::new(|| {
  let buffer_storage = BufferStorage::new();
  Mutex::new(buffer_storage)
});

#[wasm_bindgen]
pub fn new_buffer(key: String, len: usize) -> *const u8 {
  let mut global_buffer_storage = GLOBAL_BUFFER_STORAGE.lock().unwrap();
  let buffer = vec![255; len];

  let ptr = buffer.as_ptr();
  global_buffer_storage.buffer_map.insert(key, buffer);
  ptr
}

#[wasm_bindgen]
pub fn get_buffer_size(key: String) -> usize {
  let global_buffer_storage = GLOBAL_BUFFER_STORAGE.lock().unwrap();
  if let Some(buffer) = global_buffer_storage.buffer_map.get(&key) {
    return buffer.len();
  } else {
    return 0;
  }
}

#[wasm_bindgen]
pub fn get_buffer(key: String) -> *const u8 {
  let global_buffer_storage = GLOBAL_BUFFER_STORAGE.lock().unwrap();
  if let Some(buffer) = global_buffer_storage.buffer_map.get(&key) {
    return buffer.as_ptr();
  } else {
    return Vec::new().as_ptr();
  }
}

#[wasm_bindgen]
pub fn get_buffer_meta(key: String) -> Option<BufferMeta> {
  let global_buffer_storage = GLOBAL_BUFFER_STORAGE.lock().unwrap();
  if let Some(buffer) = global_buffer_storage.buffer_map.get(&key) {
    return Some(BufferMeta {
      ptr: buffer.as_ptr(),
      len: buffer.len()
    })
  } else {
    return None;
  }
}

#[wasm_bindgen]
pub fn remove_buffer(key: String) {
  let mut global_buffer_storage = GLOBAL_BUFFER_STORAGE.lock().unwrap();
  global_buffer_storage.buffer_map.remove(&key);
}

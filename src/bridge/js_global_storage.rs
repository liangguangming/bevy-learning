use wasm_bindgen::prelude::*;

use super::buffer_meta::BufferMeta;

#[wasm_bindgen]
pub fn set_wasm_instance(wasm: JsValue) {
  set_wasm(wasm)
}

#[wasm_bindgen(module = "/src/js/globalStorage.js")]
extern {
    #[wasm_bindgen(js_name = addHeapObject)]
    pub fn add_heap_object(s: JsValue) -> u32;

    #[wasm_bindgen(js_name = addHeapObjectByBufferMeta)]
    pub fn add_heap_object_by_buffer_meta(meta: BufferMeta) -> u32;

    #[wasm_bindgen(js_name = getHeapObject)]
    pub fn get_heap_object(idx: u32) -> JsValue;

    #[wasm_bindgen(js_name = getHeapObjectToVal)]
    pub fn get_heap_object_to_buffer_meta(idx: u32) -> JsValue;

    #[wasm_bindgen(js_name = setWasm)]
    fn set_wasm(w: JsValue);

    #[wasm_bindgen(js_name = dropHeapObject)]
    pub fn drop_heap_object(idx: u32);
}


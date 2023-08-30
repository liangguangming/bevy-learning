use wasm_bindgen::prelude::*;
use js_sys::Function;

static mut JS_RECEIVE_MSG_CALLBACK: JsValue = JsValue::null();

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
  a + b
}

// 外部注册信息回调
#[wasm_bindgen]
pub fn on_message(js_callback: JsValue) {
  unsafe {
    JS_RECEIVE_MSG_CALLBACK = js_callback;
  }
}

#[wasm_bindgen(module = "/src/js/message.js")]
extern {
    fn receive_msg_from_wasm(s: &str); // 内部自治
}

#[wasm_bindgen]
pub fn send_msg_to_wasm(s: &str) {
  receive_msg(s);
}

pub fn send_msg_to_js(s: &str) {
  receive_msg_from_wasm(s); // 内部的 js 接收信息
  unsafe {
    if JS_RECEIVE_MSG_CALLBACK.is_function() {
      let cb = JS_RECEIVE_MSG_CALLBACK.dyn_ref::<Function>();
      match cb {
          None => { print!("None") },
          Some(f) => {
            let _ = f.call1(&JsValue::null(), &JsValue::from_str(s)); // 外部的 js 接收信息
          }
      }
    }
  }

  
}

pub fn receive_msg(s: &str) {
  bevy::log::info!("receive web str: {}", s);
}

// 结构体
#[wasm_bindgen]
pub struct StructObj {
  name: String,
  value: u32
}

#[wasm_bindgen]
impl StructObj {

    #[wasm_bindgen(constructor)]
    pub fn new(n: &str, v: u32) -> StructObj {
      StructObj { name: n.into(), value: v }
    }

    pub fn set_name(&mut self, n: String) {
      self.name = n;
    }

    pub fn get_name(&self) -> String {
      return self.name.clone();
    }

    pub fn get_value(&self) -> u32 {
      return self.value;
    }
}

use wasm_bindgen::prelude::*;
use js_sys::Function;

static mut JS_RECEIVE_MSG_CALLBACK: JsValue = JsValue::null();

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
  a + b
}

// 外部注册信息回调
#[wasm_bindgen]
pub fn onMessage(jsCallback: JsValue) {
  unsafe {
    JS_RECEIVE_MSG_CALLBACK = jsCallback;
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
  receive_msg_from_wasm(s);
  unsafe {
    if (JS_RECEIVE_MSG_CALLBACK.is_function()) {
      let cb = Function::try_from(&JS_RECEIVE_MSG_CALLBACK);
      match cb {
          None => { print!("None") },
          Some(f) => {
            f.call1(&JsValue::null(), &JsValue::from_str(s));
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
pub struct Struct_Obj {
  name: String,
  value: u32
}

#[wasm_bindgen]
impl Struct_Obj {

    #[wasm_bindgen(constructor)]
    pub fn new(n: &str, v: u32) -> Struct_Obj {
      Struct_Obj { name: n.into(), value: v }
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

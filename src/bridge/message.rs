use wasm_bindgen::prelude::*;
use js_sys::Function;

fn no_op(s: &str) {}

static mut JS_RECEIVE_MSG_CALLBACK: JsValue = JsValue::null();
// static mut WASM_RECEIVE_MSG_CALLBACK: Fn(&str) = no_op;

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
  receive_msg_from_js(s);
}

/**
 * rust 使用 API
 */

/**
 * 发送消息给外部 js
 */
pub fn send_msg_to_js(s: &str) {
  unsafe {
    if JS_RECEIVE_MSG_CALLBACK.is_function() {
      let cb = JS_RECEIVE_MSG_CALLBACK.dyn_ref::<Function>();
      match cb {
          None => { print!("None") },
          Some(f) => {
            let _ = f.call1(&JsValue::null(), &JsValue::from_str(s));
          }
      }
    }
  }
}


/**
 * 发送消息给内部 js
 */
pub fn send_msg_to_internal_js(s: &str) {
  receive_msg_from_wasm(s);
}

// TODO 注册消息回调
// pub fn registerMessageListener(f: dyn Fn(&str)) {

// }

fn receive_msg_from_js(s: &str) {
  bevy::log::info!("receive web str: {}", s);
  // TODO: 调用注册的消息回调
}
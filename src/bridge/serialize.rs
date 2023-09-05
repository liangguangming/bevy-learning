use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message<T> {
    command: String,
    data: T
}

#[derive(Serialize, Deserialize)]
struct TestCommandData {
  name: String,
  company: String
}

#[wasm_bindgen]
pub fn get_message() -> JsValue {


    let data: TestCommandData = TestCommandData {
      name: "xsprial".to_string(),
      company: "xsprial".to_string()
    };

    let message = Message {
      command: "testCommand".to_string(),
      data
    };

    serde_wasm_bindgen::to_value(&message).unwrap()
}

#[wasm_bindgen]
pub fn set_message(value: JsValue) {
  let val: Message<TestCommandData> = serde_wasm_bindgen::from_value(value).unwrap();

  // 处理反序列化后的数据
}
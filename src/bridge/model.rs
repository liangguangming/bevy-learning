use wasm_bindgen::prelude::*;

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

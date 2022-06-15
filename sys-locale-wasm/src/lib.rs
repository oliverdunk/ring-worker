use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_locale() -> String {
    sys_locale::get_locale().unwrap()
}

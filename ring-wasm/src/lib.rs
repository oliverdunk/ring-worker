use ring::rand::{SecureRandom, SystemRandom};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_random_data() -> Vec<u8> {
    let mut data = vec![0u8;8];
    SystemRandom::new().fill(&mut data).unwrap();
    data
}

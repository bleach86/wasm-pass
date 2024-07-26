mod password;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_pass(length: usize, special: bool) -> String {
    let pass = password::generate_password(length, special);
    String::from_utf8(pass).unwrap()
}

#[wasm_bindgen]
pub fn get_pass_strength(password: &str) -> String {
    let strength = password::check_password_strength(password);
    strength.to_string()
}

mod password;
#[macro_use]
mod utils;
mod strings;

use serde::{Deserialize, Serialize};
use std::{char, str};

use base64::{engine::general_purpose::STANDARD, Engine};
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

#[derive(Serialize, Deserialize)]
struct StringInfo {
    length: u32,
    bytes: u32,
    words: u32,
    unique_chars: u32,
    unique_words: u32,
    lines: u32,
}

#[wasm_bindgen]
pub fn get_string(input_str: &str) -> JsValue {
    let words: Vec<&str> = input_str.split_whitespace().collect::<Vec<&str>>();
    let mut unique_words: Vec<String> = Vec::new();

    let chars: Vec<char> = input_str.chars().collect::<Vec<char>>();
    let mut unique_chars: Vec<char> = Vec::new();

    let lines = input_str.lines().count() as u32;

    for c in chars.iter() {
        if !unique_chars.contains(c) {
            unique_chars.push(*c);
        }
    }

    for word in words.iter() {
        let word: &str = word.trim_matches(|c: char| !char::is_alphabetic(c));
        let lowercase_word: String = word.to_lowercase();

        if !unique_words.contains(&lowercase_word) {
            unique_words.push(lowercase_word);
        }
    }

    let str_info = StringInfo {
        length: input_str.chars().count() as u32,
        bytes: input_str.len() as u32,
        words: words.len() as u32,
        unique_chars: unique_chars.len() as u32,
        unique_words: unique_words.len() as u32,
        lines,
    };
    serde_wasm_bindgen::to_value(&str_info).unwrap()
}

#[wasm_bindgen]
pub fn base64_encode(input_str: &str) -> String {
    let mut buf: String = String::new();
    STANDARD.encode_string(input_str, &mut buf);

    buf
}

#[wasm_bindgen]
pub fn base64_decode(input_str: &str) -> String {
    let res: Vec<u8> = match STANDARD.decode(input_str) {
        Ok(res) => res,
        Err(e) => {
            log!("Error decoding base64: {}", e);
            return e.to_string();
        }
    };

    str::from_utf8(&res).unwrap().to_string()
}

#[wasm_bindgen]
pub fn str_to_hex(input_str: &str) -> String {
    let mut hex_str: String = String::new();
    for c in input_str.chars() {
        hex_str.push_str(&format!("{:02x}", c as u32));
    }

    hex_str
}

#[wasm_bindgen]
pub fn hex_to_str(input_str: &str) -> String {
    let mut res: Vec<u8> = Vec::new();
    for i in 0..input_str.len() / 2 {
        let hex = &input_str[i * 2..i * 2 + 2];
        let byte = match u8::from_str_radix(hex, 16) {
            Ok(byte) => byte,
            Err(e) => {
                log!("Error converting hex to byte: {}", e);
                return e.to_string();
            }
        };
        res.push(byte);
    }

    match str::from_utf8(&res) {
        Ok(s) => s.to_string(),
        Err(e) => {
            log!("Error converting bytes to string: {}", e);
            e.to_string()
        }
    }
}

#[wasm_bindgen]
pub fn str_to_bin(input_str: &str) -> String {
    let mut bin_str: String = String::new();
    for c in input_str.chars() {
        bin_str.push_str(&format!("{:08b}", c as u32));
    }

    bin_str
}

#[wasm_bindgen]
pub fn bin_to_str(input_str: &str) -> String {
    let mut res: Vec<u8> = Vec::new();
    for i in 0..input_str.len() / 8 {
        let byte = match u8::from_str_radix(&input_str[i * 8..i * 8 + 8], 2) {
            Ok(byte) => byte,
            Err(e) => {
                log!("Error converting binary to byte: {}", e);
                return e.to_string();
            }
        };
        res.push(byte);
    }

    match str::from_utf8(&res) {
        Ok(s) => s.to_string(),
        Err(e) => {
            log!("Error converting bytes to string: {}", e);
            e.to_string()
        }
    }
}

#[wasm_bindgen]
pub fn rev_str(input_str: &str) -> String {
    input_str.chars().rev().collect::<String>()
}

#[wasm_bindgen]
pub fn substr_in_str(input_str: &str, substr: &str) -> u32 {
    input_str.trim().matches(substr.trim()).count() as u32
}

#[wasm_bindgen]
pub fn rand_uppercase(input_str: &str) -> String {
    let mut res: String = String::new();
    let mut rng = password::Rand::new(true);

    for c in input_str.chars() {
        let choice = rng.next_index(2);
        if choice == 0 {
            res.push(c.to_uppercase().next().unwrap());
        } else {
            res.push(c.to_lowercase().next().unwrap());
        }
    }

    res
}

use crate::log;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

// Special Characters: ASCII cosdes 33, 35-38, 42, 64, 94
const SPECIAL_CHARACTERS: &[u8] = b"!@#$%&*^";

// Uppercase Letters: ASCII codes 65-90
const UPPERCASE_LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

// Lowercase Letters: ASCII codes 97-122
const LOWERCASE_LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

// Numbers: ASCII codes 48-57
const NUMBERS: &[u8] = b"0123456789";

#[wasm_bindgen(module = "/www/rand_int.js")]
extern "C" {
    fn getRandomInt() -> usize;
}

pub struct Rand {
    special: bool,
}

impl Rand {
    pub fn new(special: bool) -> Self {
        Self { special }
    }

    pub fn next_index(&mut self, range: usize) -> usize {
        let rand_int: usize = getRandomInt();
        rand_int % range
    }

    fn next_char(&mut self) -> u8 {
        let category = loop {
            let selection = self.next_index(4);

            if !self.special && selection == 0 {
                continue;
            }
            break selection;
        };

        match category {
            0 => SPECIAL_CHARACTERS[self.next_index(SPECIAL_CHARACTERS.len())],
            1 => UPPERCASE_LETTERS[self.next_index(UPPERCASE_LETTERS.len())],
            2 => LOWERCASE_LETTERS[self.next_index(LOWERCASE_LETTERS.len())],
            _ => NUMBERS[self.next_index(NUMBERS.len())],
        }
    }
}

pub fn generate_password(length: usize, special: bool) -> Vec<u8> {
    let mut password: Vec<u8> = Vec::new();
    let mut rng = Rand::new(special);

    for i in 0..length {
        match i {
            0 | 3 => {
                if special {
                    let index = rng.next_index(SPECIAL_CHARACTERS.len());
                    password.push(SPECIAL_CHARACTERS[index]);
                } else {
                    password.push(rng.next_char());
                }

                continue;
            }
            1 | 4 => {
                let index = rng.next_index(UPPERCASE_LETTERS.len());

                password.push(UPPERCASE_LETTERS[index]);

                continue;
            }
            2 | 5 => {
                let index = rng.next_index(NUMBERS.len());

                password.push(NUMBERS[index]);

                continue;
            }
            6 | 7 => {
                let index = rng.next_index(LOWERCASE_LETTERS.len());

                password.push(LOWERCASE_LETTERS[index]);

                continue;
            }
            _ => password.push(rng.next_char()),
        }
    }

    let shuffle_target = length * 2;

    // Fisher-Yates shuffle
    for _ in 0..shuffle_target {
        for i in (1..length).rev() {
            let j = rng.next_index(i + 1);
            password.swap(i, j);
        }
    }

    password
}

pub fn check_password_strength(password: &str) -> &'static str {
    let unique_chars: HashSet<_> = password.chars().collect();
    let n = (unique_chars.len() as f64).log2();

    let has_digit = password.chars().any(|c| c.is_digit(10));
    let not_only_digits = password.chars().any(|c| !c.is_digit(10));
    let num = has_digit && not_only_digits;

    let caps =
        password.chars().any(|c| c.is_uppercase()) && password.chars().any(|c| c.is_lowercase());
    let extra = password.chars().any(|c| !c.is_alphanumeric());

    let score = (password.len() as f64)
        * (n + if caps { 1.0 } else { 0.0 }
            + if num { 1.0 } else { 0.0 }
            + if extra { 1.0 } else { 0.0 })
        / 30.0;

    let password_strength = match score as u8 {
        0 => "Weak",
        1 => "Medium",
        2 => "Strong",
        _ => "Very Strong",
    };

    password_strength
}

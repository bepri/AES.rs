#![allow(dead_code, unused)]
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use utiltypes::{State, Word, Key};

pub mod utiltypes;

pub fn encrypt(plaintext: u128, key: &BigUint) -> Result<u128, String> {
    let mode = match key.to_str_radix(16).len() {
        1..=32 => {
            128
        },
        33..=48 => {
            192
        },
        49..=64 => {
            256
        },
        _ => return Err(String::from("Invalid key size!")),
    };

    let state = match mode {
        128 => State::from(key.to_u128().unwrap()),
        _ => State::default(),
    };

    Ok(plaintext)
}

pub fn decrypt(ciphertext: &BigUint, key: &BigUint) -> BigUint {
    todo!();
}

fn key_expansion(key: Key, mode: i32) -> Word {
    let mut tmp: Word = Word::default();
    let mut i = 0;
    const NB: usize = 4;
    let nk: i32 = match mode {
        128 => 4,
        192 => 6,
        256 => 8,
        _ => panic!("Unreachable!"),
    };

    // Selecting Nr = 10 here.
    let w: [Word; NB * 11] = [Word::default(); NB * 11];

    while (i < nk) {
        w[i] = 
    }

    Word::default()
}

fn add_round_key() {
    todo!();
}

fn inv_mix_columns() {
    todo!();
}

fn inv_shift_rows() {
    todo!();
}

fn inv_sub_bytes() {
    todo!();
}

fn mix_columns() {
    todo!();
}

fn rot_word() {
    todo!();
}

fn shift_rows() {
    todo!();
}

fn sub_bytes() {
    todo!();
}

fn sub_word() {
    todo!();
}

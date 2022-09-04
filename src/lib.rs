#![allow(dead_code, unused)]
use num_bigint::BigUint;

pub mod state;

pub fn encrypt(plaintext: u128, key: &BigUint) -> Result<u128, String> {
    let mode = match key.to_str_radix(16).len() {
        16 => {
            128
        },
        24 => {
            192
        },
        32 => {
            256
        },
        _ => return Err(String::from("Invalid key size!")),
    };



    Ok(plaintext)
}

pub fn decrypt(ciphertext: &BigUint, key: &BigUint) -> BigUint {
    todo!();
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

#![allow(dead_code, unused)]

pub mod u256_t;
pub use u256_t::u256;

pub fn encrypt<'a>(plaintext: &'a u256, key: &u256) -> &'a u256 {
    plaintext
}

pub fn decrypt(ciphertext: &u256, key: &u256) -> u256 {
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

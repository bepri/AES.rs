#![allow(non_snake_case)]
use utiltypes::{Key, State};

pub mod utiltypes;

/// Round constants, from [USERLAB Useful Arrays](https://userlab.utk.edu/courses/cosc483/resources/aes-arrays)
const RCON: [u32; 52] = [
    0x00000000, 0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000, 0x20000000, 0x40000000,
    0x80000000, 0x1B000000, 0x36000000, 0x6C000000, 0xD8000000, 0xAB000000, 0x4D000000, 0x9A000000,
    0x2F000000, 0x5E000000, 0xBC000000, 0x63000000, 0xC6000000, 0x97000000, 0x35000000, 0x6A000000,
    0xD4000000, 0xB3000000, 0x7D000000, 0xFA000000, 0xEF000000, 0xC5000000, 0x91000000, 0x39000000,
    0x72000000, 0xE4000000, 0xD3000000, 0xBD000000, 0x61000000, 0xC2000000, 0x9F000000, 0x25000000,
    0x4A000000, 0x94000000, 0x33000000, 0x66000000, 0xCC000000, 0x83000000, 0x1D000000, 0x3A000000,
    0x74000000, 0xE8000000, 0xCB000000, 0x8D000000,
];

/// Substitution characters to shuffle all bytes, from [USERLAB Helpful Arrays](https://userlab.utk.edu/courses/cosc483/resources/aes-arrays)
const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

/// Inverse substitution characters to unshuffle all bytes, from [USERLAB Helpful Arrays](https://userlab.utk.edu/courses/cosc483/resources/aes-arrays)
const INVSBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

/**
Takes a 128-bit plaintext (padding if necessary to reach 128) and a key of 128, 192, or 256 bits (again padding to ceiling).

Returns a 128-bit AES ciphertext.
*/
pub fn cipher(plaintext: &str, key: &str) -> String {
    let key = Key::from(key);
    let mut state = State::from(plaintext);

    let (Nr, _) = key.sizes();

    let w = &key_expansion(key);

    println!("round[ 0].input    {:0>32x}", state.dump());
    print!("round[ 0].k_sch    ");
    print_words(&w, 0);

    add_round_key(&mut state, w, 0);

    for round in 1..Nr {
        println!("round[{:2}].start    {:0>32x}", round, state.dump());
        sub_bytes(&mut state);
        println!("round[{:2}].s_box    {:0>32x}", round, state.dump());
        shift_rows(&mut state);
        println!("round[{:2}].s_row    {:0>32x}", round, state.dump());
        mix_columns(&mut state);
        println!("round[{:2}].m_col    {:0>32x}", round, state.dump());
        add_round_key(&mut state, w, round);
        print!("round[{:2}].k_sch    ", round);
        print_words(w, round);
    }

    println!("round[{:2}].start    {:0>32x}", Nr, state.dump());
    sub_bytes(&mut state);
    println!("round[{:2}].s_box    {:0>32x}", Nr, state.dump());
    shift_rows(&mut state);
    println!("round[{:2}].s_row    {:0>32x}", Nr, state.dump());
    add_round_key(&mut state, w, Nr);
    print!("round[{:2}].k_sch    ", Nr);
    print_words(&w, Nr);
    println!("round[{:2}].output   {:0>32x}", Nr, state.dump());
    state.as_str()
}

/// AES mix columns protocol. Internal use only.
fn mix_columns(state: &mut State) {
    for i in 0..4 {
        let a = ff_add(
            ff_add(
                ff_add(
                    ff_multiply(0x02, state.get(i, 0)),
                    ff_multiply(0x03, state.get(i, 1)),
                ),
                state.get(i, 2),
            ),
            state.get(i, 3),
        );

        let b = ff_add(
            ff_add(
                ff_add(state.get(i, 0), ff_multiply(0x02, state.get(i, 1))),
                ff_multiply(0x03, state.get(i, 2)),
            ),
            state.get(i, 3),
        );

        let c = ff_add(
            ff_add(
                ff_add(state.get(i, 0), state.get(i, 1)),
                ff_multiply(0x02, state.get(i, 2)),
            ),
            ff_multiply(0x03, state.get(i, 3)),
        );

        let d = ff_add(
            ff_add(
                ff_add(ff_multiply(0x03, state.get(i, 0)), state.get(i, 1)),
                state.get(i, 2),
            ),
            ff_multiply(0x02, state.get(i, 3)),
        );

        *state.get_mut(i, 0) = a;
        *state.get_mut(i, 1) = b;
        *state.get_mut(i, 2) = c;
        *state.get_mut(i, 3) = d;
    }
}

/// AES shift rows protocol. Internal use only.
fn shift_rows(state: &mut State) {
    let mut tmp = [0u8; 4];
    for i in 1..4 {
        for j in 0..4 {
            tmp[j] = state.get(j, i);
        }

        for j in 0..4 {
            *state.get_mut(j, i) = tmp[(i + j) % 4];
        }
    }
}

/// AES sub bytes protocol. Internal use only.
fn sub_bytes(state: &mut State) {
    for i in state.0.iter_mut() {
        *i = SBOX[*i as usize];
    }
}

/**
Takes a 128-bit AES ciphertext (padding if necessary to reach 128) and a key of 128, 192, or 256 bits (again padding to ceiling).

Returns a 128-bit plaintext.
*/
pub fn invcipher(ciphertext: &str, key: &str) -> String {
    let key = Key::from(key);
    let mut state = State::from(ciphertext);

    let (Nr, _) = key.sizes();

    let w = key_expansion(key);

    println!("round[ 0].iinput   {:0>32x}", state.dump());
    print!("round[ 0].ik_sch   ");
    print_words(&w, Nr);

    add_round_key(&mut state, &w, Nr);
    for round in (1..Nr).rev() {
        println!("round[{:2}].istart   {:0>32x}", Nr - round, state.dump());
        inv_shift_rows(&mut state);
        println!("round[{:2}].is_row   {:0>32x}", Nr - round, state.dump());
        inv_sub_bytes(&mut state);
        println!("round[{:2}].is_box   {:0>32x}", Nr - round, state.dump());
        add_round_key(&mut state, &w, round);
        print!("round[{:2}].ik_sch   ", Nr - round);
        print_words(&w, round);
        println!("round[{:2}].ik_add   {:0>32x}", Nr - round, state.dump());
        inv_mix_columns(&mut state);
    }

    println!("round[{:2}].istart   {:0>32x}", Nr, state.dump());
    inv_shift_rows(&mut state);
    println!("round[{:2}].is_row   {:0>32x}", Nr, state.dump());
    inv_sub_bytes(&mut state);
    println!("round[{:2}].is_box   {:0>32x}", Nr, state.dump());
    add_round_key(&mut state, &w, 0);
    print!("round[{:2}].ik_sch   ", Nr);
    print_words(&w, 0);
    println!("round[{:2}].ioutput  {:0>32x}", Nr, state.dump());

    state.as_str()
}

/// AES inverse mix columns protocol. Internal use only.
fn inv_mix_columns(state: &mut State) {
    for i in 0..4 {
        let a = ff_add(
            ff_add(
                ff_add(
                    ff_multiply(0x0e, state.get(i, 0)),
                    ff_multiply(0x0b, state.get(i, 1)),
                ),
                ff_multiply(0x0d, state.get(i, 2)),
            ),
            ff_multiply(0x09, state.get(i, 3)),
        );

        let b = ff_add(
            ff_add(
                ff_add(
                    ff_multiply(0x09, state.get(i, 0)),
                    ff_multiply(0x0e, state.get(i, 1)),
                ),
                ff_multiply(0x0b, state.get(i, 2)),
            ),
            ff_multiply(0x0d, state.get(i, 3)),
        );

        let c = ff_add(
            ff_add(
                ff_add(
                    ff_multiply(0x0d, state.get(i, 0)),
                    ff_multiply(0x09, state.get(i, 1)),
                ),
                ff_multiply(0x0e, state.get(i, 2)),
            ),
            ff_multiply(0x0b, state.get(i, 3)),
        );

        let d = ff_add(
            ff_add(
                ff_add(
                    ff_multiply(0x0b, state.get(i, 0)),
                    ff_multiply(0x0d, state.get(i, 1)),
                ),
                ff_multiply(0x09, state.get(i, 2)),
            ),
            ff_multiply(0x0e, state.get(i, 3)),
        );

        *state.get_mut(i, 0) = a;
        *state.get_mut(i, 1) = b;
        *state.get_mut(i, 2) = c;
        *state.get_mut(i, 3) = d;
    }
}

/// AES inverse shift rows protocol. Internal use only.
fn inv_shift_rows(state: &mut State) {
    let mut tmp = [0u8; 4];
    for i in 1..4 {
        for j in 0..4 {
            tmp[j] = state.get(j, i);
        }

        for j in 0..4 {
            *state.get_mut(j, i) = tmp[(4 + j - i) % 4];
        }
    }
}

/// AES inverse sub bytes protocol. Internal use only.
fn inv_sub_bytes(state: &mut State) {
    for i in state.0.iter_mut() {
        *i = INVSBOX[*i as usize];
    }
}

/// AES word rotation protocol. Internal use only.
fn rot_word(input: u32) -> u32 {
    let mut bytes = bytes_from_word(input);

    let tmp = bytes[0];
    bytes[0] = bytes[3];

    let tmp2 = bytes[1];
    bytes[1] = tmp;

    let tmp3 = bytes[2];
    bytes[2] = tmp2;

    bytes[3] = tmp3;

    word_from_bytes(bytes)
}

/// AES key expansion protocol. Internal use only.
fn key_expansion(inputkey: Key) -> Vec<u32> {
    let (Nr, Nk) = inputkey.sizes();

    let mut tmp: u32;
    let mut i: usize = 0;
    const NB: usize = 4;

    let mut w: Vec<u32> = vec![0u32; NB * (Nr + 1)];

    match inputkey {
        Key::AES128(key, _) | Key::AES192(key, _) | Key::AES256(key, _) => {
            while i < Nk {
                w[i] =
                    word_from_bytes([key[4 * i + 3], key[4 * i + 2], key[4 * i + 1], key[4 * i]]);
                i += 1;
            }

            i = Nk;

            while i < NB * (Nr + 1) {
                tmp = w[i - 1];
                if (i % Nk) == 0 {
                    tmp = sub_word(rot_word(tmp)) ^ RCON[i / Nk];
                } else if Nk > 6 && i % Nk == 4 {
                    tmp = sub_word(tmp);
                }

                w[i] = w[i - Nk] ^ tmp;
                i += 1;
            }
        }
    }

    w
}

// AES round key protocol. Internal use only.
fn add_round_key(state: &mut State, w: &Vec<u32>, round: usize) {
    for i in 0..4 {
        let key = bytes_from_word(w[(round * 4) + i]);
        for j in 0..4 {
            *state.get_mut(i, j) ^= key[3 - j];
        }
    }
}

/// AES sub word protocol. Internal use only.
fn sub_word(input: u32) -> u32 {
    let bytes = bytes_from_word(input);
    SBOX[bytes[0] as usize] as u32
        | (SBOX[bytes[1] as usize] as u32) << 8
        | (SBOX[bytes[2] as usize] as u32) << 16
        | (SBOX[bytes[3] as usize] as u32) << 24
}

/// Finite-field addition. Returns a + b.
fn ff_add(a: u8, b: u8) -> u8 {
    a ^ b
}

/// Finite-field multiplication. Returns a * b.
fn ff_multiply(mut a: u8, mut b: u8) -> u8 {
    let mut res = 0u8;
    while a != 0 && b != 0 {
        if b & 1 == 1 {
            res ^= a;
        }

        a = xtime(a);
        b >>= 1;
    }

    res
}

/// Finite-field multiplication xtime implementation.
fn xtime(a: u8) -> u8 {
    match a & 0x80 {
        0 => a << 1,
        0x80 => (a << 1) ^ 0x1b,
        _ => unreachable!(),
    }
}

/// Taking an array of 4 bytes, return a 32-bit word of the assembled bytes.
fn word_from_bytes(bytes: [u8; 4]) -> u32 {
    bytes[0] as u32 | (bytes[1] as u32) << 8 | (bytes[2] as u32) << 16 | (bytes[3] as u32) << 24
}

/// Taking in a u32-bit word, disassemble it into an array of 4 bytes.
fn bytes_from_word(word: u32) -> [u8; 4] {
    [
        (word & 0xff).try_into().unwrap(),
        ((word >> 8) & 0xff).try_into().unwrap(),
        ((word >> 16) & 0xff).try_into().unwrap(),
        ((word >> 24) & 0xff).try_into().unwrap(),
    ]
}

/// Print a specific word from the AES round key.
fn print_words(w: &Vec<u32>, index: usize) {
    for i in 0..4 {
        print!("{:08x}", w[index * 4 + i]);
    }
    println!();
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn xtime_tests() {
        assert_eq!(xtime(0x57), 0xae);
        assert_eq!(xtime(0xae), 0x47);
        assert_eq!(xtime(0x47), 0x8e);
        assert_eq!(xtime(0x8e), 0x07);
    }

    #[test]
    fn ff_multiply_tests() {
        assert_eq!(ff_multiply(0x57, 0x13), 0xfe);
    }

    #[test]
    fn subword_tests() {
        assert_eq!(sub_word(0x00102030), 0x63cab704);
        assert_eq!(sub_word(0x00102030), 0x63cab704);
        assert_eq!(sub_word(0x40506070), 0x0953d051);
        assert_eq!(sub_word(0x8090a0b0), 0xcd60e0e7);
        assert_eq!(sub_word(0xc0d0e0f0), 0xba70e18c);
    }

    #[test]
    fn rotword_tests() {
        assert_eq!(rot_word(0x09cf4f3c), 0xcf4f3c09);
        assert_eq!(rot_word(0x2a6c7605), 0x6c76052a);
    }

    #[test]
    fn keyexpansion_tests() {
        let key: Key = Key::from("2b7e151628aed2a6abf7158809cf4f3c");
        let expanded: Vec<u32> = vec![
            0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c, 0xa0fafe17, 0x88542cb1, 0x23a33939,
            0x2a6c7605, 0xf2c295f2, 0x7a96b943, 0x5935807a, 0x7359f67f, 0x3d80477d, 0x4716fe3e,
            0x1e237e44, 0x6d7a883b, 0xef44a541, 0xa8525b7f, 0xb671253b, 0xdb0bad00, 0xd4d1c6f8,
            0x7c839d87, 0xcaf2b8bc, 0x11f915bc, 0x6d88a37a, 0x110b3efd, 0xdbf98641, 0xca0093fd,
            0x4e54f70e, 0x5f5fc9f3, 0x84a64fb2, 0x4ea6dc4f, 0xead27321, 0xb58dbad2, 0x312bf560,
            0x7f8d292f, 0xac7766f3, 0x19fadc21, 0x28d12941, 0x575c006e, 0xd014f9a8, 0xc9ee2589,
            0xe13f0cc8, 0xb6630ca6,
        ];

        assert_eq!(key_expansion(key), expanded);
    }

    #[test]
    /// Currently fails due to a mixup of rows and columns, but actual functionality of the program is fine.
    fn big_test_lol() {
        let mut state = State([
            0x19, 0xa0, 0x9a, 0xe9, 0x3d, 0xf4, 0xc6, 0xf8, 0xe3, 0xe2, 0x8d, 0x48, 0xbe, 0x2b,
            0x2a, 0x08,
        ]);

        let sub = State([
            0xd4, 0xe0, 0xb8, 0x1e, 0x27, 0xbf, 0xb4, 0x41, 0x11, 0x98, 0x5d, 0x52, 0xae, 0xf1,
            0xe5, 0x30,
        ]);

        let shift = State([
            0xd4, 0xe0, 0xb8, 0x1e, 0xbf, 0xb4, 0x41, 0x27, 0x5d, 0x52, 0x11, 0x98, 0x30, 0xae,
            0xf1, 0xe5,
        ]);

        let mix = State([
            0x04, 0xe0, 0x48, 0x28, 0x66, 0xcb, 0xf8, 0x06, 0x81, 0x19, 0xd3, 0x26, 0xe5, 0x9a,
            0x7a, 0x4c,
        ]);

        let round = State([
            0xa4, 0x68, 0x6b, 0x02, 0x9c, 0x9f, 0x5b, 0x6a, 0x7f, 0x35, 0xea, 0x50, 0xf2, 0x2b,
            0x43, 0x49,
        ]);

        sub_bytes(&mut state);
        assert_eq!(state, sub);

        shift_rows(&mut state);
        assert_eq!(state, shift);

        mix_columns(&mut state);
        assert_eq!(state, mix);

        add_round_key(
            &mut state,
            &key_expansion(Key::from("2b7e151628aed2a6abf7158809cf4f3c")),
            1,
        );
        assert_eq!(state, round);
    }

    #[test]
    fn cipher_test() {
        let result = State([
            0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a,
            0x0b, 0x32,
        ]);

        let output = cipher(
            "3243f6a8885a308d313198a2e0370734",
            "2b7e151628aed2a6abf7158809cf4f3c",
        );

        assert_eq!(output, result.as_str());
    }
}

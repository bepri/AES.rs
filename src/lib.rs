#![allow(non_snake_case, unused)]
use utiltypes::{Key, State, Word};

pub mod utiltypes;

const RCON: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];

pub fn encrypt(plaintext: u128, key: &str) -> Result<u128, String> {
    let key = Key::from(key);
    let mut state = State::from(plaintext);

    let (Nr, Nk) = key.sizes();

    let Nb: usize = 4;

    for round in 1..=Nr {
        sub_bytes(&mut state);
        shift_rows(&mut state);
        mix_columns(&mut state);
        // TODO: add back once you figure out wtf w is
        // add_round_key(&mut state);
    }

    // add_round_key(&state, )

    Ok(state.dump())
}

pub fn decrypt(ciphertext: u128, key: &str) -> Result<u128, String> {
    todo!();
}

fn key_expansion(key: Key, mode: i32) -> Word {
    let (Nr, Nk) = key.sizes();

    let mut tmp: Word = Word::default();
    let mut i: usize = 0;
    const NB: usize = 4;

    let mut w: Vec<Word> = vec![Word::default(); NB * (Nr + 1)];

    match key {
        Key::AES128(arr, size) | Key::AES192(arr, size) | Key::AES256(arr, size) => {
            let nk = size / 4;

            while (i < Nk) {
                w[i] = Word::from([arr[4 * i], arr[4 * i + 1], arr[4 * i + 2], arr[4 * i + 3]]);
                i += 1;
            }

            i = Nk;

            while (i < NB * (Nr + 1)) {
                tmp = w[i - 1];
                if ((i % Nk) == 0) {
                    // TODO: pls god finish this
                    // tmp = sub_word(rot_word(tmp)) ^ todo!();
                } else if (Nk > 6 && i % Nk == 4) {
                    tmp = sub_word(tmp);
                }

                w[i] = w[i - Nk] ^ tmp;
                i += 1;
            }
        }
    }

    Word::default()
}

fn add_round_key(mut state: &State, mut w: &Vec<Word>) {
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

fn mix_columns(mut state: &mut State) {
    for i in 0..4 {
        let a = poly_mult(0x02, state.get(0, i))
                  ^ poly_mult(0x03, state.get(1, i))
                  ^ state.get(2, i)
                  ^ state.get(3, i);

        let b = state.get(0, i)
                  ^ poly_mult(0x02, state.get(1, i))
                  ^ poly_mult(0x03, state.get(2, i))
                  ^ state.get(3, i);

        let c = state.get(0, i)
                  ^ state.get(1, i)
                  ^ poly_mult(0x02, state.get(2, i))
                  ^ poly_mult(0x03, state.get(3, i));

        let d = poly_mult(0x03, state.get(0, i))
                  ^ state.get(1, i)
                  ^ state.get(2, i)
                  ^ poly_mult(0x02, state.get(3, i));

        *state.get_mut(0, i) = a;
        *state.get_mut(1, i) = b;
        *state.get_mut(2, i) = c;
        *state.get_mut(3, i) = d;
    }
}

fn rot_word(mut input: Word) -> Word {
    todo!();
}

fn shift_rows(mut state: &State) {
    todo!();
}

fn sub_bytes(mut state: &State) {
    todo!();
}

fn sub_word(mut input: Word) -> Word {
    todo!();
}

fn poly_mult(mut a: u8, mut b: u8) -> u8 {
    let mut res = 0u8;
    while (a != 0) {
        if (b & 1 == 1) {
            res ^= a;
        }

        a = xtime(a);
        b >>= 1;
    }

    res
}

fn xtime(a: u8) -> u8 {
    match a & 0x80 {
        0 => a << 1,
        0x80 => (a << 1) ^ 0x1b,
        _ => unreachable!(),
    }
}

use std::{fmt, ops::BitXor};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct State(pub [u8; 16]);

impl From<u128> for State {
    fn from(mut input: u128) -> Self {
        let mut state: [u8; 16] = [0; 16];
        for i in (0..=15).rev() {
            state[i] = (input & 0xff) as u8;
            input >>= 8;
        }

        Self(state)
    }
}

impl Default for State {
    fn default() -> Self {
        Self::from(0)
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..4 {
            for col in 0..4 {
                write!(f, "{:02X} ", self.get(row, col));
            }
            writeln!(f);
        }

        Ok(())
    }
}

impl State {
    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.0[col + (row * 4)]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut u8 {
        &mut self.0[col + (row * 4)]
    }

    pub fn dump(&self) -> u128 {
        let mut res = 0u128;
        for i in self.0.iter() {
            res |= *i as u128;
            res <<= 8;
        }

        res
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Word(u32);

impl Word {
    fn as_arr(&self) -> [u8; 4] {
        let mut res: [u8; 4] = [0; 4];
        let mut hold = self.0;
        for i in res.iter_mut() {
            *i = (hold & 0xf) as u8;
            hold >>= 4;
        }

        res
    }

    fn as_word(&self) -> u32 {
        self.0
    }
}

impl From<[u8; 4]> for Word {
    fn from(input: [u8; 4]) -> Self {
        let mut res = 0u32;
        for i in (0..4).rev() {
            res <<= 4;
            res |= input[i] as u32;
            println!("{}", res);
        }

        Self(res)
    }
}

impl BitXor<Self> for Word {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Key {
    AES256(Vec<u8>, usize),
    AES192(Vec<u8>, usize),
    AES128(Vec<u8>, usize),
}

impl From<&str> for Key {
    fn from(input: &str) -> Self {
        let size = match input.len() {
            1..=32 => 16,
            33..=48 => 24,
            49..=64 => 32,
            _ => panic!("Invalid key size!"),
        };

        let mut arr = vec![0u8; size];

        for (i, item) in input.as_bytes().chunks(2).enumerate() {
            arr[i] = match u8::from_str_radix(String::from_utf8_lossy(item).into_owned().as_str(), 16) {
                Ok(i) => i,
                Err(_) => panic!("Could not parse string key!"),
            };
        }

        match size {
            16 => Key::AES128(arr, size),
            24 => Key::AES192(arr, size),
            32 => Key::AES256(arr, size),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn state_from() {
        let a = State::default();
        assert_eq!(a.0, [0; 16]);
    }

    #[test]
    fn word_from() {
        let a = Word::from([1, 2, 3, 4]);        
        assert_eq!(a.as_arr(), [1, 2, 3, 4]);
    }

    #[test]
    fn key_from() {
        let a = Key::from("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
        assert_eq!(a, Key::AES256(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31], 32));
    }
}

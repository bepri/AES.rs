use std::{fmt, ops::Index};

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

#[derive(Clone, Copy, Debug)]
pub enum Key {
    AES256([u8; 32]),
    AES192([u8; 24]),
    AES128([u8; 16]),
}

impl Index<usize> for Key {
    let Output = u8;

    fn index(i: usize) -> Output {
        
    }
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
            arr[i] = u8::from_str_radix(String::from_utf8_lossy(item).into_owned().as_str(), 16).unwrap();
        }

        println!("{:?}", arr);
        Key::AES128([0u8; 16])

        // match size {
        //     16 => Key::AES128(arr.as_slice()),
        //     24 => Key::AES192(*arr.as_slice()),
        //     32 => Key::AES256(*arr.as_slice()),
        //     _ => panic!("Unreachable!"),
        // }
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
}

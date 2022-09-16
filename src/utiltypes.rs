use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct State(pub [u8; 16]);

impl From<&str> for State {
    fn from(input: &str) -> Self {
        let mut state: [u8; 16] = [0; 16];
        for (i, item) in input.as_bytes().chunks(2).enumerate() {
            state[i] = match u8::from_str_radix(String::from_utf8_lossy(item).into_owned().as_str(), 16) {
                Ok(i) => i,
                Err(_) => panic!("Could not parse string input!"),
            };
        }

        Self(state)
    }
}

impl Default for State {
    fn default() -> Self {
        Self([0u8; 16])
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..4 {
            for col in 0..4 {
                match write!(f, "{:02X} ", self.get(row, col)) {
                    Ok(_) => (),
                    Err(_) => panic!("Failed to print out state!"),
                }
            }
            match writeln!(f) {
                Ok(_) => (),
                Err(_) => panic!("Failed to print out state!"),
            }
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
            res <<= 8;
            res |= *i as u128;
        }

        res
    }

    pub fn as_str(&self) -> String {
        format!("{:032x}", self.dump())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Key {
    AES256(Vec<u8>, usize),
    AES192(Vec<u8>, usize),
    AES128(Vec<u8>, usize),
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AES128(arr, _) | Self::AES192(arr, _) | Self::AES256(arr, _) => {
                for i in arr {
                    match write!(f, "{:02X} ", i) {
                        Ok(_) => (),
                        Err(_) => panic!("Failed to print out key!"),
                    }
                }
            }
        }

        Ok(())
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
            arr[i] =
                match u8::from_str_radix(String::from_utf8_lossy(item).into_owned().as_str(), 16) {
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

impl Key {
    pub fn sizes(&self) -> (usize, usize) {
        match self {
            Key::AES128(_, _) => (10usize, 4usize),
            Key::AES192(_, _) => (12usize, 6usize),
            Key::AES256(_, _) => (14usize, 8usize),
        }
    }
}

#[cfg(test)]
mod util_tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn state_from() {
        let a = State::default();
        assert_eq!(a.0, [0; 16]);
    }

    #[test]
    fn key_from() {
        let a = Key::from("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
        assert_eq!(
            a,
            Key::AES256(
                vec![
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31
                ],
                32
            )
        );
    }
}

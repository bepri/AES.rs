use std::fmt;
use std::ops::{Add, Sub};

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq)]
pub struct u256 {
    num: [u8; 8],
}

impl From<&str> for u256 {
    fn from(string: &str) -> u256 {
        let mut newnum: [u8; 8] = [0; 8];
        let string = String::from(string);
        let string = match string.len() {
            1..=32 => format!("{:0<32}", string),
            33..=48 => format!("{:0<48}", string),
            49..=64 => format!("{:0<64}", string),
            _ => panic!("Input out of range!"),
        };

        println!("{}", string);

        for (i, byte) in string.as_bytes().chunks(2).enumerate() {
            let mut x = 0;

            for c in byte {
                println!("{}", *c);
                x *= 16;
                x += match *c {
                    b'a'..=b'f' => *c - b'a' + 10,
                    b'A'..=b'F' => *c - b'A' + 10,
                    b'0'..=b'9' => *c - b'0',
                    _ => panic!("Parse error!"),
                };
            }
        }
        u256 { num: newnum }
    }
}

impl Add for u256 {
    type Output = u256;

    fn add(self, other: u256) -> u256 {
        let mut newnum: [u8; 8] = [0; 8];
        let mut holder: (u8, bool) = (0, false);
        let mut carry = false;

        for (i, item) in newnum.iter_mut().enumerate() {
            holder = self.num[i].overflowing_add(other.num[i]);
            *item = match carry {
                true => holder.0 + 1,
                false => holder.0,
            };

            carry = holder.1;
        }

        u256 { num: newnum }
    }
}

impl Sub for u256 {
    type Output = u256;

    fn sub(self, other: u256) -> u256 {
        let mut newnum: [u8; 8] = [0; 8];
        let mut holder: (u8, bool) = (0, false);
        let mut carry = false;

        for (i, item) in newnum.iter_mut().enumerate() {
            holder = self.num[i].overflowing_sub(other.num[i]);
            *item = match carry {
                true => holder.0 - 1,
                false => holder.0,
            };

            carry = holder.1;
        }

        u256 { num: newnum }
    }
}

impl fmt::Debug for u256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for item in self.num.iter() {
            result.push_str(format!("{:x?}", item).as_str());
        }

        write!(f, "{}", result)
    }
}

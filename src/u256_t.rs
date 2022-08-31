use std::fmt;
use std::ops::Add;

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq)]
pub struct u256 {
    num: [u8; 8],
}

impl From<&str> for u256 {
    fn from(string: &str) -> u256 {
        let mut newnum: [u8; 8] = [0; 8];

        for (i, byte) in string.as_bytes().chunks(2).enumerate() {
            let mut x = 0;

            // println!("{:?}", byte);
            for c in byte {
                x *= 16;
                x += match *c {
                    b'a'..=b'f' => {
                        println!("first");
                        *c - b'a' + 10
                    }
                    b'A'..=b'F' => {
                        println!("second");
                        *c - b'A' + 10
                    }
                    b'1'..=b'9' => {
                        println!("third");
                        *c - b'0'
                    }
                    _ => panic!("Parse error!"),
                };
                println!("loop once, x = {}, c = {}", x, *c as char);
            }
            // println!("completed conversion");
        }
        u256 { num: newnum }
    }
}

impl Add for u256 {
    type Output = u256;

    fn add(self, other: u256) -> u256 {
        let mut newnum: [u8; 8] = [0; 8];
        let mut carry = false;
        let mut holder: (u8, bool) = (0, false);

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

impl fmt::Debug for u256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for item in self.num.iter() {
            result.push_str(format!("{:x?}", item).as_str());
        }

        write!(f, "{}", result)
    }
}

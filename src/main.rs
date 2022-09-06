use aes;
use num_bigint::BigUint;

fn main() {
    let a = BigUint::parse_bytes(b"000102030405060708090a0b0c0d0e0f", 16).unwrap();

    aes::encrypt(1234, &a).unwrap();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    
}

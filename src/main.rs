use aes::{cipher, invcipher};

fn main() {
    invcipher(cipher("00112233445566778899aabbccddeeff", "000102030405060708090a0b0c0d0e0f").as_str(), "000102030405060708090a0b0c0d0e0f");
}
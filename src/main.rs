use aes::{cipher, invcipher};
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        print_usage_and_quit(&args[0]);
    }

    match args[1].as_str() {
        "--decrypt" | "-d" => println!("{}", invcipher(&args[2], &args[3])),
        "--encrypt" | "-e" => println!("{}", cipher(&args[2], &args[3])),
        _ => print_usage_and_quit(&args[0]),
    }
}

fn print_usage_and_quit(progname: &str) {
    println!("Usage: {} [--decrypt|--encrypt] text key", progname);
    exit(1);
}

#[cfg(test)]
mod main_tests {
    use super::*;

    #[test]
    fn aes128() {
        let plaintext = "00112233445566778899aabbccddeeff";
        let key = "000102030405060708090a0b0c0d0e0f";
        let res = invcipher(cipher(plaintext, key).as_str(), key);

        assert_eq!(plaintext, res.as_str());
    }

    #[test]
    fn aes192() {
        let plaintext = "00112233445566778899aabbccddeeff";
        let key = "000102030405060708090a0b0c0d0e0f1011121314151617";
        let res = invcipher(cipher(plaintext, key).as_str(), key);

        assert_eq!(plaintext, res.as_str());
    }

    #[test]
    fn aes256() {
        let plaintext = "00112233445566778899aabbccddeeff";
        let key = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
        let res = invcipher(cipher(plaintext, key).as_str(), key);

        assert_eq!(plaintext, res.as_str());
    }
}
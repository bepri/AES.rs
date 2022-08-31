use clap::Parser;
use aes::u256_t::u256;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// Key to encode/decode with
    #[clap(short, long)]
    key: String,
    input: String,
}

fn main() {
    println!("hi");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_128() {
        let plaintext = u256::from("69c4e0d86a7b0430d8cdb78070b4c55a");
        let key = u256::from("000102030405060708090a0b0c0d0e0f");

        assert_eq!(aes::encrypt(&plaintext, &key), &plaintext);
        // assert_eq!(1, 2);
    }
}
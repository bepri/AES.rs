use aes::u256_t::u256;
use clap::Parser;

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
    fn test_add_simple() {
        let a = u256::from("1");
        let b = u256::from("2");

        assert_eq!(a + b, u256::from("3"));
    }

    #[test]
    fn test_add_complex() {
        let a = u256::from("BB87F7A424F735952F7633E8F7AB3E3012976E753C6E6D7699D51EE5CF2C0792");
        let b = u256::from("1E0A2FA51EFC693A661C6EB72F91C27168F1A03C2605AAACC43434C39835AFF8");

        assert_eq!(
            a + b,
            u256::from("d992274943f39ecf9592a2a0273d00a17b890eb1627418235e0953a96761b78a")
        );
    }

    #[test]
    fn test_128() {
        let plaintext = u256::from("69c4e0d86a7b0430d8cdb78070b4c55a"); //ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
        let key = u256::from("000102030405060708090a0b0c0d0e0f");

        assert_eq!(aes::encrypt(&plaintext, &key), &plaintext);
    }
}

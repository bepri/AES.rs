use clap::Parser;
use num_bigint::BigInt;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// Key to encode/decode with
    #[clap(short, long)]
    key: String,
    input: String,
}

fn main() {
    let a = BigInt::parse_bytes(b"69c4e0d86a7b0430d8cdb78070b4c55a", 16).unwrap();

    println!("{}", a+1);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    
}

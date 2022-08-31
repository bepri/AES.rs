# A Rust implementation of AES for COSC583 @ UTK
## DISCLAIMER
### This was written for a university course. It is ***NOT*** a smart choice for cryptographically important applications and absolutely does not replace the [rust-crypto](https://crates.io/crates/rust-crypto) crate.

## Compiling and running
### Using Cargo:
```
cargo run --release
```

### Using Rustc:
```
rustc src/main.rs -o aes
./aes
```

### Running unit tests:
```
cargo test
```

## Resources used
- [FIPS 197](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf)
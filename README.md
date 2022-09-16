# A Rust implementation of AES for COSC583 @ UTK
## DISCLAIMER
### This was written for a university course. It is ***NOT*** a smart choice for cryptographically important applications and absolutely does not replace the [rust-crypto](https://crates.io/crates/rust-crypto) crate.

## Compiling and running
### Using Cargo:
```
cargo run --release --
```

### Using Rustc:
```
rustc src/main.rs -o aes
./aes
```

### To run the three example inputs from FIPS 197 Appendix C:
```
cargo test main_tests -- --show-output
```
As these tests will show, I currently pass all three test cases.

### To run a specific test:
```
cargo test main_tests::aes<KEYSIZE> -- --show-output
```

## Resources used
The below resources are the only ones I made use of to understand and test my AES implementation.
- [FIPS 197](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf)
- [USERLAB Unit Tests](https://userlab.utk.edu/courses/cosc483/resources/aes-unit-tests)
- [USERLAB Helpful Arrays](https://userlab.utk.edu/courses/cosc483/resources/aes-arrays)
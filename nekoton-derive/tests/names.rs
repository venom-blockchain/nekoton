use std::fmt::Display;

use nekoton_abi::UnpackAbi;
use ton_abi::{Token, TokenValue, Uint};

#[derive(UnpackAbi)]
struct ValidSt {
    #[abi(name = "validField")]
    _field: u32,
}

#[derive(UnpackAbi)]
struct InvalidSt {
    #[abi(name = "invalidField")]
    _field: u32,
}

fn assert<T: Display>(expected: &str, value: T) {
    assert_eq!(expected, value.to_string());
}

fn main() {
    let field = Token::new("validField", TokenValue::Uint(Uint::new(10, 32)));
    let tokens = vec![field];

    let tuple = Token::new("tuple", TokenValue::Tuple(tokens));

    let invalid: Result<InvalidSt, _> = tuple.clone().unpack();
    assert(
        "Invalid name (expected \"invalidField\", found \"validField\")",
        invalid.err().unwrap(),
    );

    let valid: Result<ValidSt, _> = tuple.unpack();
    assert!(valid.is_ok());
}
use super::*;
use crypto_bigint::U256;
use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

#[test]
fn deserialize() {
	assert_de_tokens(
		&HexString::from(81985529216486895u128),
		&[Token::Str("0x0123456789abcdef")],
	);
	assert_de_tokens(&HexString::from(0u128), &[Token::Str("0x0")]);
	assert_de_tokens(&HexString::from(1u128), &[Token::Str("0x1")]);
	assert_de_tokens(
		&HexString::from(U256::MAX),
		&[Token::Str(
			"0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		)],
	);

    assert_de_tokens(
		&HexString::from(U256::MAX),
		&[Token::Str(
			"0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		)],
	);
}

#[test]
fn test_de_hex_prefixed_string_too_long() {
	let test_str = "0xffffffffffffffffffffffffffffffff";
	assert_de_tokens_error::<HexString>(
		&[Token::Str(test_str)],
		&format!(
			"invalid length {}, expected a \"0x\" prefixed string, encoding the hexadecimal representation of an u256",
			test_str.len()
		),
	)
}

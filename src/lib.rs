extern crate base64;
extern crate byteorder;
extern crate hex;
pub mod error;
use byteorder::{BigEndian, ReadBytesExt};
use error::{exit, CodyError};
use std::io::Cursor;

fn trim(encoded_input: Vec<u8>) -> Result<String, std::string::FromUtf8Error> {
    // Strip leading and trailing ASCII whitespace
    let in_string = String::from_utf8(encoded_input)?;
    Ok(in_string.trim().into())
}

#[test]
fn test_trim() {
    assert_eq!(trim("".into()).unwrap(), "");
    assert_eq!(trim(" a ".into()).unwrap(), "a");
    assert_eq!(trim(" a b c\td\t".into()).unwrap(), "a b c\td");
}

pub fn binary_hexadecimal(input: &Vec<u8>) -> String {
    hex::encode(input)
}

#[test]
fn test_binary_hexadecimal() {
    assert_eq!(binary_hexadecimal(&vec![]), "".to_string());
    assert_eq!(
        binary_hexadecimal(&b"PETE".to_vec()),
        "50455445".to_string()
    );
    assert_eq!(
        binary_hexadecimal(&b"unit-test".to_vec()),
        "756e69742d74657374".to_string()
    );
}

pub fn binary_base64(input: &Vec<u8>) -> String {
    base64::encode(input)
}

#[test]
fn test_binary_base64() {
    assert_eq!(binary_base64(&vec![]), "".to_string());
    assert_eq!(binary_base64(&b"PETE".to_vec()), "UEVURQ==".to_string());
    assert_eq!(
        binary_base64(&b"unit-test".to_vec()),
        "dW5pdC10ZXN0".to_string()
    );
}

pub fn binary_decimal(mut input: Vec<u8>) -> Result<u64, CodyError> {
    if input.len() > 8 {
        return exit("Can only decode a maximum of 8 bytes to decimal").map(|_| 0);
    }
    // Pad with leading zero bytes until we have 64 bits total
    while input.len() < 8 {
        input.insert(0, 0);
    }

    let mut reader = Cursor::new(&input);
    Ok(reader.read_u64::<BigEndian>()?)
}

#[test]
fn test_binary_decimal() {
    assert_eq!(binary_decimal(vec![]).unwrap(), 0);
    assert_eq!(binary_decimal(vec![0b0000001]).unwrap(), 1);
    assert_eq!(binary_decimal(vec![0b0000001, 0b00000001]).unwrap(), 257);
    assert_eq!(binary_decimal(vec![0b0000001, 0]).unwrap(), 256);
    assert_eq!(binary_decimal(vec![0, 0b0000010, 0b00000010]).unwrap(), 514);
    assert_eq!(
        binary_decimal(vec![0, 0, 0, 0, 0b0000010, 0b00000010]).unwrap(),
        514
    );
    assert!(binary_decimal(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]).is_err());
}

pub fn hexadecimal_binary(encoded_input: Vec<u8>) -> Result<Vec<u8>, CodyError> {
    let in_string = trim(encoded_input)?;
    let decoded_input = hex::decode(in_string.as_bytes())?;
    Ok(decoded_input)
}

#[test]
fn test_hexadecimal_binary() {
    assert_eq!(hexadecimal_binary(vec![]).unwrap().len(), 0);
    assert_eq!(
        hexadecimal_binary("01".as_bytes().to_vec()).unwrap(),
        vec![1]
    );
    assert_eq!(
        hexadecimal_binary("00fe".as_bytes().to_vec()).unwrap(),
        vec![0, 254]
    );
    assert_eq!(
        hexadecimal_binary("aa00fe".as_bytes().to_vec()).unwrap(),
        vec![170, 0, 254]
    );
    assert!(hexadecimal_binary("fg".as_bytes().to_vec()).is_err());
}

pub fn hexadecimal_base64(encoded_input: Vec<u8>) -> Result<String, CodyError> {
    let in_string = trim(encoded_input)?;
    let decoded_input = hex::decode(&in_string)?;
    Ok(base64::encode(&decoded_input))
}

#[test]
fn test_hexadecimal_base64() {
    // https://cryptii.com/binary-to-base64
    assert_eq!(hexadecimal_base64(vec![]).unwrap().len(), 0);
    assert_eq!(
        hexadecimal_base64("01".as_bytes().to_vec()).unwrap(),
        "AQ==".to_string()
    );
    assert_eq!(
        hexadecimal_base64("00fe".as_bytes().to_vec()).unwrap(),
        "AP4=".to_string()
    );
    assert_eq!(
        hexadecimal_base64("aa00fe".as_bytes().to_vec()).unwrap(),
        "qgD+".to_string()
    );
    assert!(hexadecimal_base64("fg".as_bytes().to_vec()).is_err());
}

pub fn hexadecimal_decimal(encoded_input: Vec<u8>) -> Result<u64, CodyError> {
    let in_string = trim(encoded_input)?;
    let decoded_input = hex::decode(in_string.as_bytes())?;
    if decoded_input.len() > 8 {
        return exit("Can only decode a maximum of 16 hexadecimal characters to decimal").map(|_| 0);
    }
    binary_decimal(decoded_input)
}

#[test]
fn test_hexadecimal_decimal() {
    assert_eq!(hexadecimal_decimal(vec![]).unwrap(), 0);
    assert_eq!(hexadecimal_decimal(b"ff".to_vec()).unwrap(), 255);
    assert_eq!(hexadecimal_decimal(b"FF".to_vec()).unwrap(), 255);
    assert_eq!(hexadecimal_decimal(b"fF".to_vec()).unwrap(), 255);
    assert_eq!(hexadecimal_decimal(b"fe".to_vec()).unwrap(), 254);
    assert_eq!(hexadecimal_decimal(b"0007".to_vec()).unwrap(), 7);
    assert_eq!(hexadecimal_decimal(b"00000001".to_vec()).unwrap(), 1);
    assert_eq!(hexadecimal_decimal(b"0100".to_vec()).unwrap(), 256);
    assert!(hexadecimal_decimal(b"0".to_vec()).is_err() "Must have even number of hex chars");
    assert!(hexadecimal_decimal(b"000".to_vec()).is_err() "Must have even number of hex chars");
    assert!(hexadecimal_decimal(b"00000".to_vec()).is_err() "Must have even number of hex chars");
    assert!(
        hexadecimal_decimal(b"1122334455667788A".to_vec()).is_err(),
        "Max 16 hex chars can be parsed"
    );
    assert!(hexadecimal_decimal(b"fg".to_vec()).is_err());
}

pub fn base64_binary(encoded_input: Vec<u8>) -> Result<Vec<u8>, CodyError> {
    let in_string = trim(encoded_input)?;
    Ok(base64::decode(&in_string)?)
}

#[test]
fn test_base64_binary() {
    assert_eq!(base64_binary("".as_bytes().to_vec()).unwrap().len(), 0);
    // $ printf "Hello, Cody!" | base64
    // SGVsbG8sIENvZHkh
    assert_eq!(
        base64_binary("SGVsbG8sIENvZHkh".as_bytes().to_vec()).unwrap(),
        b"Hello, Cody!"
    );
    assert!(base64_binary("nope-nope".as_bytes().to_vec()).is_err());
}

pub fn base64_hexadecimal(encoded_input: Vec<u8>) -> Result<String, CodyError> {
    let in_string = trim(encoded_input)?;
    let decoded_input = base64::decode(&in_string)?;
    let mut output = String::new();
    for byte in &decoded_input {
        output.push_str(&format!("{:x}", byte));
    }
    Ok(output)
}

#[test]
fn test_base64_hexadecimal() {
    assert_eq!(base64_hexadecimal("".as_bytes().to_vec()).unwrap().len(), 0);
    // $ printf "Hello, Cody!" | base64
    // SGVsbG8sIENvZHkh
    assert_eq!(
        base64_hexadecimal("SGVsbG8sIENvZHkh".as_bytes().to_vec()).unwrap(),
        "48656c6c6f2c20436f647921"
    );
    assert!(base64_hexadecimal("nope-nope".as_bytes().to_vec()).is_err());
}

pub fn decimal_hexadecimal(encoded_input: Vec<u8>) -> Result<i64, CodyError> {
    let in_string = trim(encoded_input)?;
    Ok(in_string.parse()?)
}

#[test]
fn test_decimal_hexadecimal() {
    assert!(decimal_hexadecimal("".as_bytes().to_vec()).is_err());
    assert_eq!(decimal_hexadecimal("10".as_bytes().to_vec()).unwrap(), 10);
    assert_eq!(decimal_hexadecimal("256".as_bytes().to_vec()).unwrap(), 256);
    assert_eq!(decimal_hexadecimal("-42".as_bytes().to_vec()).unwrap(), -42);
    assert!(decimal_hexadecimal("nope".as_bytes().to_vec()).is_err());
}

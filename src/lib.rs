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

pub fn binary_hex(input: &Vec<u8>) -> String {
    hex::encode(input)
}

pub fn hex_dec(encoded_input: Vec<u8>) -> Result<u64, CodyError> {
    let in_string = trim(encoded_input)?;
    let decoded_input = hex::decode(in_string.as_bytes())?;
    if decoded_input.len() > 8 {
        return exit("Can only decode a maximum of 16 hexadecimal characters to decimal").map(|_| 0);
    }
    binary_decimal(decoded_input)
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

pub fn hex_binary(encoded_input: Vec<u8>) -> Result<Vec<u8>, CodyError> {
    let in_string = trim(encoded_input)?;
    let decoded_input = hex::decode(in_string.as_bytes())?;
    Ok(decoded_input)
}

pub fn dec_hex(encoded_input: Vec<u8>) -> Result<u64, CodyError> {
    let in_string = trim(encoded_input)?;
    Ok(in_string.parse()?)
}

pub fn base64_hex(encoded_input: Vec<u8>) -> Result<String, CodyError> {
    let in_string = trim(encoded_input)?;
    let decoded_input = base64::decode(&in_string)?;
    let mut output = String::new();
    for byte in &decoded_input {
        output.push_str(&format!("{:x}", byte));
    }
    Ok(output)
}

pub fn base64_binary(encoded_input: Vec<u8>) -> Result<Vec<u8>, CodyError> {
    let in_string = trim(encoded_input)?;
    Ok(base64::decode(&in_string)?)
}

pub fn hex_base64(encoded_input: Vec<u8>) -> Result<String, CodyError> {
    let in_string = trim(encoded_input)?;
    let decoded_input = hex::decode(&in_string)?;
    Ok(base64::encode(&decoded_input))
}

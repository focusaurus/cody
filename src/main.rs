extern crate base64;
extern crate byteorder;
extern crate hex;
mod error;
use byteorder::{BigEndian, ReadBytesExt};
use error::{exit, CodyError};
use std::env::args;
use std::io::{self, Cursor, Read, Write};

fn trim(encoded_input: Vec<u8>) -> Result<String, std::string::FromUtf8Error> {
    // Strip leading and trailing ASCII whitespace
    let in_string = String::from_utf8(encoded_input)?;
    Ok(in_string.trim().into())
}

fn hex_dec(encoded_input: Vec<u8>) -> Result<u64, CodyError> {
    let in_string = trim(encoded_input)?;
    let mut decoded_input = hex::decode(in_string.as_bytes())?;
    if decoded_input.len() > 8 {
        return exit("Can only decode a maximum of 16 hexadecimal characters to decimal").map(|_| 0);
    }
    // Pad with leading zero bytes until we have 64 bits total
    while decoded_input.len() < 8 {
        decoded_input.insert(0, 0);
    }

    let mut reader = Cursor::new(&decoded_input);
    Ok(reader.read_u64::<BigEndian>()?)
}

fn hex_binary(encoded_input: Vec<u8>) -> Result<Vec<u8>, CodyError> {
    let in_string = trim(encoded_input)?;
    let decoded_input = hex::decode(in_string.as_bytes())?;
    Ok(decoded_input)
}

fn dec_hex(encoded_input: Vec<u8>) -> Result<u64, CodyError> {
    let in_string = trim(encoded_input)?;
    Ok(in_string.parse()?)
}

fn base64_hex(encoded_input: Vec<u8>) -> Result<String, CodyError> {
    let in_string = trim(encoded_input)?;
    let decoded_input = base64::decode(&in_string)?;
    let mut output = String::new();
    for byte in &decoded_input {
        output.push_str(&format!("{:x}", byte));
    }
    Ok(output)
}

fn base64_binary(encoded_input: Vec<u8>) -> Result<Vec<u8>, CodyError> {
    let in_string = trim(encoded_input)?;
    Ok(base64::decode(&in_string)?)
}

fn hex_base64(encoded_input: Vec<u8>) -> Result<String, CodyError> {
    let in_string = trim(encoded_input)?;
    let decoded_input = hex::decode(&in_string)?;
    Ok(base64::encode(&decoded_input))
}

fn main() -> Result<(), CodyError> {
    let mut stdin = io::stdin();
    let in_format = args().nth(1).unwrap_or("binary".into());
    let in_format = in_format.as_str();
    let out_format = args().nth(2).unwrap_or("binary".into());
    let out_format = out_format.as_str();
    let mut encoded_input = vec![];
    stdin.read_to_end(&mut encoded_input)?;
    if encoded_input.len() == 0 {
        return exit("standard input was empty");
    }
    println!("{} {}", in_format, out_format);
    match (in_format, out_format) {
        ("binary", "hex") => {
            println!("{}", hex::encode(&encoded_input));
        }
        ("decimal", "hex") => {
            println!("{:x}", dec_hex(encoded_input)?);
        }
        ("hex", "decimal") => {
            println!("{}", hex_dec(encoded_input)?);
        }
        ("hex", "binary") => {
            io::stdout().write(&hex_binary(encoded_input)?)?;
        }
        ("base64", "hex") => {
            println!("{}", base64_hex(encoded_input)?);
        }
        ("base64", "binary") => {
            io::stdout().write(&base64_binary(encoded_input)?)?;
        }
        ("binary", "base64") => {
            println!("{}", base64::encode(&encoded_input));
        }
        ("hex", "base64") => {
            println!("{}", hex_base64(encoded_input)?);
        }
        (_, _) => {
            io::stdout().write(&encoded_input)?;
        }
    }
    Ok(())
}

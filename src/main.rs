extern crate base64;
extern crate byteorder;
extern crate hex;
mod error;
use byteorder::{BigEndian, ReadBytesExt};
use error::{exit, CodyError};
use std::env::args;
use std::io::{self, Cursor, Read, Write};

fn hex_dec(encoded_input: Vec<u8>) -> Result<u64, CodyError> {
    // Strip leading and trailing ASCII whitespace
    let in_string = String::from_utf8(encoded_input)?;
    let in_string = in_string.trim();
    let mut decoded_input = hex::decode(in_string.as_bytes())?;
    if decoded_input.len() > 8 {
        return exit("Can only decode a maximum of 16 hexadecimal characters to decimal").map(|_| 0);
    }
    // Pad with leading zero bytes until we have 64 bits total
    while decoded_input.len() < 8 {
        decoded_input.insert(0, 0);
    }

    // println!("{:?}", decoded_input);
    let mut reader = Cursor::new(&decoded_input);
    Ok(reader.read_u64::<BigEndian>()?)
    // reader.read_u64::<BigEndian>().map_err(|e|e.into())
}

fn main() -> Result<(), CodyError> {
    let mut stdin = io::stdin();
    let in_format = args().nth(1).unwrap_or("auto".into());
    let in_format = in_format.as_str();
    let out_format = args().nth(2).unwrap_or("auto".into());
    let out_format = out_format.as_str();
    let mut encoded_input = vec![];
    stdin.read_to_end(&mut encoded_input)?;
    if encoded_input.len() == 0 {
        return exit("standard input was empty");
    }

    match (in_format, out_format) {
        ("decimal", "hex") => {
            let in_string = String::from_utf8(encoded_input)?;
            let number: u64 = in_string.trim().parse()?;
            println!("{:x}", number);
        }
        ("hex", "decimal") => {
            println!("{}", hex_dec(encoded_input)?);
        }
        ("base64", "hex") => {
            let in_string = String::from_utf8(encoded_input)?;
            let in_string = in_string.trim();
            let decoded_input = base64::decode(&in_string)?;
            for byte in &decoded_input {
                print!("{:x}", byte);
            }
        }
        ("base64", "binary") => {
            let in_string = String::from_utf8(encoded_input)?;
            let in_string = in_string.trim();
            let decoded_input = base64::decode(&in_string)?;
            io::stdout().write(&decoded_input)?;
        }
        ("binary", "base64") => {
            let encoded_output = base64::encode(&encoded_input);
            io::stdout().write(encoded_output.as_bytes())?;
        }
        ("hex", "base64") => {
            let in_string = String::from_utf8(encoded_input)?;
            let in_string = in_string.trim();
            let decoded_input = hex::decode(&in_string)?;
            println!("{:?}", decoded_input);
            let encoded_output = base64::encode(&decoded_input);
            io::stdout().write(encoded_output.as_bytes())?;
        }
        (_, _) => {
            io::stdout().write(&encoded_input)?;
        }
    }
    /*
    let decoded_input = match in_format.as_str() {
        "base64" => {
            let in_string = String::from_utf8(encoded_input)?;
            let in_string = in_string.trim();
            base64::decode(&in_string)?
        }
        "hex" => {
            let in_string = String::from_utf8(encoded_input)?;
            let in_string = in_string.trim();
            hex::decode(&in_string)?
        }
        "decimal" => {}
        _ => encoded_input,
    };
    match out_format.as_str() {
        "hex" => {
            let encoded_output = hex::encode(&decoded_input);
            io::stdout().write(encoded_output.as_bytes())?;
        }
        "base64" => {
            let encoded_output = base64::encode(&decoded_input);
            io::stdout().write(encoded_output.as_bytes())?;
        }
        _ => {
            io::stdout().write(&decoded_input)?;
        }
    }
    */
    Ok(())
}

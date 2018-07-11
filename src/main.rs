extern crate base64;
extern crate byteorder;
extern crate hex;
mod error;
use byteorder::{BigEndian, ReadBytesExt};
use error::CodyError;
use std::env::args;
use std::io::{self, Cursor, Read, Write};

fn main() -> Result<(), CodyError> {
    let mut stdin = io::stdin();
    let in_format = args().nth(1).unwrap_or("auto".into());
    let in_format = in_format.as_str();
    let out_format = args().nth(2).unwrap_or("auto".into());
    let out_format = out_format.as_str();
    let mut encoded_input = vec![];
    stdin.read_to_end(&mut encoded_input)?;

    match (in_format, out_format) {
        ("decimal", "hex") => {
            let in_string = String::from_utf8(encoded_input)?;
            let number: u64 = in_string.trim().parse()?;
            println!("{:x}", number);
        }
        ("hex", "decimal") => {
            let decoded_input = hex::decode(&encoded_input)?;
            println!("{:?}", decoded_input);

            let mut reader = Cursor::new(&decoded_input);
            match decoded_input.len() {
                1 => {
                    let value = reader.read_u8()?;
                    println!("{}", value);
                }
                2 => {
                    let value = reader.read_u16::<BigEndian>()?;
                    println!("{}", value);
                }
                4 => {
                    let value = reader.read_u32::<BigEndian>()?;
                    println!("{}", value);
                }
                8 => {
                    let value = reader.read_u64::<BigEndian>()?;
                    println!("{}", value);
                }
                _ => {
                    return Err(CodyError {
                        message: "Converting hex to decimal requires 1,2,4, or 8 bytes of hex"
                            .into(),
                    })
                }
            }
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

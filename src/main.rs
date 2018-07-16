extern crate cody;
use cody::error::CodyError;
use std::env::args;
use std::io::{self, Read, Write};

fn main() -> Result<(), CodyError> {
    let mut stdin = io::stdin();
    let in_format = args().nth(1).unwrap_or("binary".into());
    let in_format = in_format.as_str();
    let out_format = args().nth(2).unwrap_or("binary".into());
    let out_format = out_format.as_str();
    let mut encoded_input = vec![];

    stdin.read_to_end(&mut encoded_input)?;
    if encoded_input.len() == 0 {
        return cody::error::exit("standard input was empty");
    }

    match (in_format, out_format) {
        ("binary", "base64") => {
            println!("{}", cody::binary_base64(&encoded_input));
        }
        ("binary", "decimal") => {
            println!("{}", cody::binary_decimal(encoded_input)?);
        }
        ("binary", "hex") => {
            println!("{}", cody::binary_hex(&encoded_input));
        }
        ("base64", "binary") => {
            io::stdout().write(&cody::base64_binary(encoded_input)?)?;
        }
        ("base64", "hex") => {
            println!("{}", cody::base64_hex(encoded_input)?);
        }
        ("decimal", "hex") => {
            println!("{:x}", cody::dec_hex(encoded_input)?);
        }
        ("hex", "decimal") => {
            println!("{}", cody::hex_dec(encoded_input)?);
        }
        ("hex", "binary") => {
            io::stdout().write(&cody::hex_binary(encoded_input)?)?;
        }
        ("hex", "base64") => {
            println!("{}", cody::hex_base64(encoded_input)?);
        }
        (_, _) => {
            io::stdout().write(&encoded_input)?;
        }
    }
    Ok(())
}

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
        ("binary", "hexadecimal") => {
            println!("{}", cody::binary_hexadecimal(&encoded_input));
        }
        ("base64", "binary") => {
            io::stdout().write(&cody::base64_binary(encoded_input)?)?;
        }
        ("base64", "hexadecimal") => {
            println!("{}", cody::base64_hex(encoded_input)?);
        }
        ("decimal", "hexadecimal") => {
            println!("{:x}", cody::decimal_hexadecimal(encoded_input)?);
        }
        ("hexadecimal", "decimal") => {
            println!("{}", cody::hexadecimal_decimal(encoded_input)?);
        }
        ("hexadecimal", "binary") => {
            io::stdout().write(&cody::hexadecimal_binary(encoded_input)?)?;
        }
        ("hexadecimal", "base64") => {
            println!("{}", cody::hexadecimal_base64(encoded_input)?);
        }
        (_, _) => {
            io::stdout().write(&encoded_input)?;
        }
    }
    Ok(())
}

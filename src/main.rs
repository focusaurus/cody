extern crate cody;
use cody::error::CodyError;
use std::env::args;
use std::io::{self, Read, Write};

fn main() -> Result<(), CodyError> {
    let mut stdin = io::stdin();
    let in_format = args().nth(1).unwrap_or_else(|| "binary".into());
    let in_format = cody::normalize(in_format);
    if in_format.as_str() == "--help" || in_format.as_str() == "-h" {
        return cody::error::exit(
            "Usage: cody in-format out-format
data to transcode is read from standard input
encoded data is written to standard output
supported encodings are:
binary
hexadecimal
decimal
base64
",
        );
    }
    let out_format = args().nth(2).unwrap_or_else(|| "binary".into());
    let out_format = cody::normalize(out_format);
    let mut encoded_input = vec![];

    stdin.read_to_end(&mut encoded_input)?;
    if encoded_input.is_empty() {
        return cody::error::exit("standard input was empty");
    }

    match (in_format.as_str(), out_format.as_str()) {
        ("binary", "hexadecimal") => {
            println!("{}", cody::binary_hexadecimal(&encoded_input));
        }
        ("binary", "base64") => {
            println!("{}", cody::binary_base64(&encoded_input));
        }
        ("binary", "decimal") => {
            println!("{}", cody::binary_decimal(encoded_input)?);
        }
        ("hexadecimal", "binary") => {
            io::stdout().write_all(&cody::hexadecimal_binary(encoded_input)?)?;
        }
        ("hexadecimal", "base64") => {
            println!("{}", cody::hexadecimal_base64(encoded_input)?);
        }
        ("hexadecimal", "decimal") => {
            println!("{}", cody::hexadecimal_decimal(encoded_input)?);
        }
        ("base64", "binary") => {
            io::stdout().write_all(&cody::base64_binary(encoded_input)?)?;
        }
        ("base64", "hexadecimal") => {
            println!("{}", cody::base64_hexadecimal(encoded_input)?);
        }
        ("decimal", "binary") => {
            println!("{}", cody::decimal_binary(encoded_input)?);
        }
        ("decimal", "hexadecimal") => {
            println!("{:x}", cody::decimal_binary(encoded_input)?);
        }
        ("decimal", "base64") => {
            println!("{}", cody::decimal_base64(encoded_input)?);
        }
        (_, _) => {
            io::stdout().write_all(&encoded_input)?;
        }
    }
    Ok(())
}

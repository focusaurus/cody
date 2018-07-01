extern crate base64;
extern crate hex;

use std::io::{self, Read, Write};

#[derive(Debug)]
struct CodyError {
    message: String,
}

impl From<io::Error> for CodyError {
    fn from(error: io::Error) -> Self {
        CodyError {
            message: "IO Error".into(),
        }
    }
}

impl From<std::string::FromUtf8Error> for CodyError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        CodyError {
            message: "UTF-8 Error".into(),
        }
    }
}

impl From<base64::DecodeError> for CodyError {
    fn from(error: base64::DecodeError) -> Self {
        CodyError {
            message: "Input was not valid base64".into(),
        }
    }
}
impl From<hex::FromHexError> for CodyError {
    fn from(error: hex::FromHexError) -> Self {
        CodyError {
            message: "Input was not valid hexadecimal".into(),
        }
    }
}

fn main() -> Result<(), CodyError> {
    let mut stdin = io::stdin();
    match std::env::args().nth(1) {
        Some(format) => {
            if &format == "base64" {
                let mut in_string = String::new();
                stdin.read_to_string(&mut in_string)?;
                let in_string = &in_string.trim();
                let out_bytes = base64::decode(&in_string)?;
                io::stdout().write(&out_bytes)?;
            }
            if &format == "hex" {
                let mut in_string = String::new();
                stdin.read_to_string(&mut in_string)?;
                let in_string = &in_string.trim();
                let out_bytes = hex::decode(&in_string)?;
                io::stdout().write(&out_bytes)?;
            }
        }
        None => {}
    }
    Ok(())
}

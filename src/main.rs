extern crate base64;

use base64::decode;
use std::io::{self, Read};

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

fn main() -> Result<(), CodyError> {
    let mut in_string = String::new();
    io::stdin().read_to_string(&mut in_string)?;
    let in_string = &in_string.trim();
    // println!("IN: ^{}$", &in_string);
    let out_bytes = decode(&in_string)?;
    // println!("OUT: {:?}", out_bytes);
    let result = String::from_utf8(out_bytes)?;
    println!("{}", result);
    Ok(())
}

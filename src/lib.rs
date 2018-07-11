extern crate base64;
extern crate byteorder;
extern crate hex;
use std::fmt;
use std::io;

pub struct CodyError {
    pub message: String,
}

impl From<io::Error> for CodyError {
    fn from(_error: io::Error) -> Self {
        CodyError {
            message: "IO Error".into(),
        }
    }
}

impl From<std::string::FromUtf8Error> for CodyError {
    fn from(_error: std::string::FromUtf8Error) -> Self {
        CodyError {
            message: "UTF-8 Error".into(),
        }
    }
}

impl From<base64::DecodeError> for CodyError {
    fn from(_error: base64::DecodeError) -> Self {
        CodyError {
            message: "Input was not valid base64".into(),
        }
    }
}

impl From<hex::FromHexError> for CodyError {
    fn from(_error: hex::FromHexError) -> Self {
        CodyError {
            message: "Input was not valid hexadecimal".into(),
        }
    }
}

impl From<std::num::ParseIntError> for CodyError {
    fn from(_error: std::num::ParseIntError) -> Self {
        CodyError {
            message: "Input was not valid decimal number".into(),
        }
    }
}

impl fmt::Debug for CodyError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "{}", self.message)
    }
}
// impl fmt::Debug for CodyError {
//     fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
//         write!(formatter, "{}", self.message)
//     }
// }

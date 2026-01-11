mod line_error;

use std::fmt;
use std::fmt::Display;

pub use line_error::LineError;

#[derive(Debug)]
pub enum MyError {
    IoError(std::io::Error),
    Utf8Error(core::str::Utf8Error),
    ParseError(chrono::ParseError),
    XlsxError(rust_xlsxwriter::XlsxError),
    LineError(LineError),
}

impl MyError {}

impl From<std::io::Error> for MyError {
    fn from(value: std::io::Error) -> Self {
        MyError::IoError(value)
    }
}

impl From<core::str::Utf8Error> for MyError {
    fn from(value: core::str::Utf8Error) -> Self {
        MyError::Utf8Error(value)
    }
}

impl From<chrono::ParseError> for MyError {
    fn from(value: chrono::ParseError) -> Self {
        MyError::ParseError(value)
    }
}

impl From<rust_xlsxwriter::XlsxError> for MyError {
    fn from(value: rust_xlsxwriter::XlsxError) -> Self {
        MyError::XlsxError(value)
    }
}

impl From<LineError> for MyError {
    fn from(value: LineError) -> Self {
        MyError::LineError(value)
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::IoError(err) => write!(f, "io error:\n{}", err),
            MyError::LineError(err) => write!(f, "utf8 error:\n{}", err),
            MyError::ParseError(err) => write!(f, "parse error:\n{}", err),
            MyError::Utf8Error(err) => write!(f, "utf8 error:\n{}", err),
            MyError::XlsxError(err) => write!(f, "xlsx error:\n{}", err),
        }
    }
}

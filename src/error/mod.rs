mod line_error;

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

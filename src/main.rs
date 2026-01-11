use parsefile_lib::MyError;
use parsefile_lib::monitor;
use parsefile_lib::parse_file;

use std::path::Path;

fn main() {
    monitor();
    let path = Path::new("1.txt");
    match parse_file(path) {
        Ok(number) => {
            println!("passed {} records", number);
        }
        Err(e) => match e {
            MyError::IoError(error) => {
                println!("io error:\n{}", error);
            }
            MyError::Utf8Error(utf8_error) => {
                println!("utf8 error:\n{}", utf8_error);
            }
            MyError::ParseError(parse_error) => {
                println!("parse error:\n{}", parse_error);
            }
            MyError::XlsxError(xlsx_error) => {
                println!("xlsx error:\n{}", xlsx_error);
            }
            MyError::LineError(line_error) => {
                println!("line error:\n{}", line_error);
            }
        },
    }
}

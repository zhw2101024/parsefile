use parsefile_lib::MyError;
use parsefile_lib::{parse_file, write_map};

fn handle(path: &str) -> Result<bool, MyError> {
    let record_map = parse_file(path)?;
    let ret = write_map(record_map)?;
    Ok(ret)
}

fn main() {
    match handle("1.txt") {
        Ok(succeed) => {
            if succeed {
                println!("Success!");
            } else {
                println!("Failed!");
            }
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
        },
    }
}

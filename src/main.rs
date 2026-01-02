use parsefile_lib::MyError;
use parsefile_lib::monitor;
use parsefile_lib::parse_file;

fn main() {
    monitor();
    match parse_file("1.txt") {
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
            MyError::LineError(line_error) => {
                println!("line error:\n{}", line_error);
            }
        },
    }
}

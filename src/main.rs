use parsefile_lib::{MyError, Program};
use parsefile_lib::{parse_file, write_single};

fn handle(path: &str) -> Result<bool, MyError> {
    let mut programs: Vec<Program> = vec![];
    let parsed = parse_file(path, &mut programs)?;
    let ret = if parsed {
        write_single(&programs)?
    } else {
        false
    };
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

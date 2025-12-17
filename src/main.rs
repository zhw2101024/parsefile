use parsefile_lib::parse_file;
use parsefile_lib::{MyError, Program};

fn main() {
    let path = "1.txt";

    let mut programs: Vec<Program> = vec![];
    match parse_file(path, &mut programs) {
        Ok(v) => {
            if v {
                println!("{:?}", v);
                for program in programs {
                    program.print();
                }
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
        },
    }
}

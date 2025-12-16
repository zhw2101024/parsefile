use parsefile_lib::MyError;
use parsefile_lib::parse_file;

fn main() {
    let path = "1.txt";

    match parse_file(path) {
        Ok(v) => println!("{:?}", v),
        Err(e) => match e {
            MyError::IoError(error) => {
                println!("io error: {}", error);
            }
            MyError::Utf8Error(utf8_error) => {
                println!("{}", utf8_error);
            }
        },
    }
}

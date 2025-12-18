use parsefile_lib::{MyError, Program};
use parsefile_lib::{parse_file, write, write_single};

fn output(programs: &Vec<Program>) -> Result<(), MyError> {
    let _res = write(programs)?;
    let res = write_single(programs)?;
    println!("{}", res);
    Ok(())
}

fn main() {
    let path = "1.txt";

    let mut programs: Vec<Program> = vec![];
    match parse_file(path, &mut programs) {
        Ok(v) => {
            if v {
                println!("{:?}", v);
                let _ = output(&programs);
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

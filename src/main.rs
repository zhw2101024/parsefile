use parsefile_lib::init_subject;
use parsefile_lib::parse_file;

use std::path::Path;

fn main() {
    init_subject();

    let path = Path::new("1.txt");
    match parse_file(path) {
        Ok(number) => {
            println!("passed {} records", number);
        }
        Err(e) => eprintln!("{}", e),
    }
}

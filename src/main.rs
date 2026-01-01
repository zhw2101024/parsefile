mod ui;

use parsefile_lib::monitor;
use parsefile_lib::parse_file;

use std::path::Path;

use ui::hello;

fn main() {
    monitor();
    hello();
    let path = Path::new("1.txt");
    match parse_file(path) {
        Ok(number) => {
            println!("passed {} records", number);
        }
        Err(e) => println!("{}", e),
    }
}

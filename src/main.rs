use parsefile_lib::{ConcreteSubject, Subject};
use parsefile_lib::{add_observer, parse_file};

use std::path::Path;

fn main() {
    let subject: &mut ConcreteSubject = &mut ConcreteSubject::default();
    add_observer(subject, "init");

    subject.notify("Hello, observers!");

    let path = Path::new("1.txt");
    match parse_file(path) {
        Ok(number) => {
            println!("passed {} records", number);
        }
        Err(e) => eprintln!("{}", e),
    }
}

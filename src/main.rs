mod ui;

use parsefile_lib::{ConcreteSubject, Subject};
use parsefile_lib::{add_observer, parse_file};

use std::cell::OnceCell;
use std::path::Path;

use ui::hello;

fn main() {
    hello();

    let mut cell = OnceCell::new();
    if cell.set(ConcreteSubject::default()).is_err() {
        panic!("unable to initialize the subject");
    }
    let subject: &mut ConcreteSubject = cell.get_mut().unwrap();
    add_observer(subject);
    subject.notify("Hello, observers!");
    let path = Path::new("1.txt");
    match parse_file(path) {
        Ok(number) => {
            println!("passed {} records", number);
        }
        Err(e) => println!("{}", e),
    }
}

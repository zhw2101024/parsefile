pub mod cli;
pub mod error;
pub mod message;
pub mod output;
pub mod parser;
pub mod program;

pub use parser::parse_file;

use message::{ConcreteSubject, Observer, Subject};

use cli::SUBJECT;
use cli::add_observer;

use error::{LineError, MyError};
use output::write_map;
use program::{Item, Program, Record};

pub fn init_subject() {
    unsafe {
        let subject = &mut *SUBJECT;
        add_observer(subject, "init");
    }
}

pub fn notify(msg: &str) {
    unsafe {
        let subject = &mut *SUBJECT;
        subject.notify(msg);
    }
}

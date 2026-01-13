pub mod cli;
pub mod error;
pub mod message;
pub mod output;
pub mod parser;
pub mod program;
pub mod ui;

pub use parser::parse_file;

use message::{ConcreteSubject, Observer, Subject};

#[cfg(feature = "cli")]
use cli::{SUBJECT, add_observer};
#[cfg(feature = "ui")]
use ui::{SUBJECT, add_observer};

pub use ui::hello;

use error::{LineError, MyError};
use output::write_map;
use program::{Item, Program, Record};

pub fn start() {
    #[cfg(feature = "ui")]
    hello();
}

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

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
use std::path::Path;
#[cfg(feature = "ui")]
use ui::{SUBJECT, add_observer};

#[cfg(feature = "ui")]
pub use ui::hello;

use error::{LineError, MyError};
use output::write_map;
use program::{Item, Program, Record};

enum Channel {
    Tv(i32),
    Open,
}

fn program_channel(path: &Path) -> Option<Channel> {
    let filename = path.file_name()?.to_string_lossy();
    let first = filename.char_indices().next()?.1;
    let channel = match first {
        '一' | '1' => Channel::Tv(1),
        '二' | '2' => Channel::Tv(2),
        '三' | '3' => Channel::Tv(3),
        _ => Channel::Open,
    };
    Some(channel)
}

#[cfg(feature = "cli")]
pub fn hello() {
    let first = std::env::args().nth(1).unwrap_or("1.txt".to_string());
    let path = Path::new(&first);

    match parse_file(Path::new(&path)) {
        Ok(number) => {
            println!("passed {} records", number);
        }
        Err(e) => println!("{}", e),
    };
}

pub fn start() {
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

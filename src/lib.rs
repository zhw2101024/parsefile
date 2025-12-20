pub mod error;
pub mod output;
pub mod parser;
pub mod program;

pub use error::MyError;
pub use output::write_map;
pub use parser::parse_file;
pub use program::{Item, Program, Record};

use super::parse_content;
use crate::MyError;

pub fn parse_file(path: &str) -> Result<bool, MyError> {
    let contents = std::fs::read_to_string(path)?;
    parse_content(&contents)
}

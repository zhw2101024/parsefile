use super::parse_content;
use crate::{MyError, Program};

pub fn parse_file(path: &str, programs: &mut Vec<Program>) -> Result<bool, MyError> {
    let contents = std::fs::read_to_string(path)?;

    let res = parse_content(&contents, programs)?;
    Ok(res)
}

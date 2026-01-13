use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct LineError {
    lineno: usize,
    line: String,
}

impl LineError {
    pub fn new(lineno: usize, line: String) -> Self {
        LineError { lineno, line }
    }

    pub fn lineno(&self) -> usize {
        self.lineno
    }

    pub fn line(&self) -> &str {
        &self.line
    }
}

impl Display for LineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: 无法处理行数据\n{}", self.lineno(), self.line())
    }
}

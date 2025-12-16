mod fileparser;

use crate::MyError;
use crate::{Item, Program};
use regex::Regex;

pub use fileparser::parse_file;

fn parse_date(line: &str) -> Result<&str, MyError> {
    let date_re = Regex::new(r"^\b播出日期\s*(.*)$").unwrap();

    let Some((_, [date])) = date_re.captures(line).map(|caps| caps.extract()) else {
        panic!("解析失败行：\n{}", line);
    };
    Ok(date)
}

fn parse_content(contents: &str) -> Result<bool, MyError> {
    let list_re = Regex::new(r"^\b(\d{2})\b\s*:\s*\b(\d{2})\s(.+)\s*$").unwrap();

    let mut programs: Vec<Program> = vec![];
    let mut program = Program::default();

    for (index, line) in contents.lines().enumerate() {
        match line {
            line if line.trim().is_empty() | line.trim().starts_with("节目长度") => {}
            line if line.trim().starts_with("播出日期") => {
                let line = line.trim();
                if !program.empty() {
                    programs.push(program.clone());
                    program = Program::default();
                }

                let lineno = index + 1;
                let date = parse_date(line);
                assert!(date.is_ok(), "第{}行解析出错：\n{}", lineno, line);
                let date = date.unwrap();
                program.set_date(date);
            }
            line => {
                let line_trimed = line.trim();
                let Some((_, [hour, minute, name])) =
                    list_re.captures(line_trimed).map(|caps| caps.extract())
                else {
                    panic!("第{}行\n{} 不符合格式：\n05:30 节目预告", index + 1, line);
                };
                let hour = hour.parse::<u8>().expect("Invalid hour");
                let minute = minute.parse::<u8>().expect("Invalid minute");
                let name = name.trim();
                assert!(hour < 24, "{hour}");
                assert!(minute < 60, "{minute}");
                assert!(!name.is_empty(), "{name}");
                let item = Item::new(hour, minute, name.to_string(), line);
                program.add_item(item);
            }
        }
    }
    programs.push(program.clone());

    for program in programs {
        program.print();
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date() {
        assert_eq!(parse_date("播出日期 2025/12/15").unwrap(), "2025/12/15");
        assert_eq!(parse_date("播出日期2025/12/15").unwrap(), "2025/12/15");
        assert_eq!(parse_date("播出日期  2025/12/15").unwrap(), "2025/12/15");
    }
}

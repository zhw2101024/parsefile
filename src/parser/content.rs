use crate::{Item, LineError, Program};
use regex::Regex;
use std::io::{Error, ErrorKind};

fn parse_date(line: &str) -> Result<String, Error> {
    let date_re = Regex::new(r"^\b播出日期\s*(\d{4}).*(\d{2}).*(\d{2})$").unwrap();

    let Some((_, [year, month, day])) = date_re.captures(line).map(|caps| caps.extract()) else {
        let err = Error::new(
            ErrorKind::InvalidData,
            format!("  {} 不符合格式:\n  播出日期 2025/12/15", line),
        );
        return Err(err);
    };
    let date = [year, month, day].join("/");

    Ok(date)
}

fn parse_list(line: &str) -> Result<(u32, u32, &str), Error> {
    let list_re = Regex::new(r"^\b(\d{2})\b\s*:\s*\b(\d{2})\s*(.+)\s*$").unwrap();

    let Some((_, [hour, minute, name])) = list_re.captures(line).map(|caps| caps.extract()) else {
        let err = Error::new(
            ErrorKind::InvalidData,
            format!("  {} 不符合格式：\n  05:30 节目预告", line),
        );
        return Err(err);
    };

    let hour: u32 = hour.parse().expect("Invalid hour");
    let minute: u32 = minute.parse().expect("Invalid minute");
    let name = name.trim();
    assert!(minute < 60, "{minute}");
    assert!(!name.is_empty(), "{name}");
    Ok((hour, minute, name))
}

pub fn parse_content<'a>(
    contents: &'a str,
    programs: &mut Vec<Program<'a>>,
) -> Result<bool, Error> {
    let mut ret = true;
    let mut program = Program::default();

    for (index, line) in contents.lines().enumerate() {
        let lineno = index + 1;
        let line = line.trim();
        match line {
            line if line.is_empty() | line.starts_with("节目长度") | line.contains("频道") => {
            }
            line if line.starts_with("播出日期") => {
                if !program.empty() {
                    programs.push(program);
                    program = Program::default();
                }

                let date = parse_date(line).unwrap_or("".to_string());

                match program.set_date(date.as_str()) {
                    Ok(_) => {}
                    Err(_err) => {
                        println!("{}", LineError::new(lineno, line.to_string()));
                        ret = false;
                    }
                }
            }
            line => match parse_list(line) {
                Ok((hour, minute, name)) => {
                    let item = Item::new(hour, minute, name);
                    program.add_item(item);
                }
                Err(_err) => {
                    println!("{}", LineError::new(lineno, line.to_string()));
                    ret = false;
                }
            },
        }
    }
    programs.push(program);

    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date() {
        assert_eq!(parse_date("播出日期 2025/12/15").unwrap(), "2025/12/15");
        assert_eq!(parse_date("播出日期  2025/12/15").unwrap(), "2025/12/15");
        assert_eq!(parse_date("播出日期 2025/ 12 /15").unwrap(), "2025/12/15");
        assert_eq!(parse_date("播出日期2025/12/15").unwrap(), "2025/12/15");
    }

    #[test]
    fn test_parse_list() {
        assert_eq!(parse_list("05:30 节目预告").unwrap(), (05, 30, "节目预告"));
        assert_eq!(parse_list("05: 30 节目预告").unwrap(), (05, 30, "节目预告"));
        assert_eq!(parse_list("05:30节目预告").unwrap(), (05, 30, "节目预告"));
    }

    #[test]
    #[should_panic]
    fn test_parse_list_invalid() {
        assert_eq!(parse_list("05:303 节目预告").unwrap(), (05, 30, "节目预告"));
    }
}

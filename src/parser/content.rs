use crate::MyError;
use crate::{Item, LineError, Program};
use chrono::{NaiveDate, TimeDelta};
use regex::Regex;
use std::collections::BTreeMap;
use std::io::{Error, ErrorKind};

fn parse_date(line: &str) -> Result<String, Error> {
    let date_re = Regex::new(r"^\b播出日期\s*(\d{4})/\s*(\d{1,2})\s*/\s*(\d{1,2})\s*$").unwrap();

    let Some((_, [year, month, day])) = date_re.captures(line).map(|caps| caps.extract()) else {
        let err = Error::new(
            ErrorKind::InvalidData,
            format!("  {} 不符合格式:\n  播出日期 2025/12/15", line),
        );
        return Err(err);
    };
    let date = format!("{}/{:0>2}/{:0>2}", year, month, day);

    Ok(date)
}

fn parse_list(source_date: String, line: &str) -> Result<(NaiveDate, u32, u32, &str), Error> {
    let list_re = Regex::new(r"^\s*\b(\d{2})\b\s*:\s*\b(\d{2})\s*(.+)\s*$").unwrap();

    let Some((_, [hour, minute, name])) = list_re.captures(line).map(|caps| caps.extract()) else {
        let err = Error::new(
            ErrorKind::InvalidData,
            format!("  {} 不符合格式：\n  05:30 节目预告", line),
        );
        return Err(err);
    };

    let Some(mut hour) = hour.parse::<u32>().ok() else {
        let err = Error::new(ErrorKind::InvalidData, format!("Invalid hour: {}", hour));
        return Err(err);
    };
    let item_date = NaiveDate::parse_from_str(source_date.as_str(), "%Y/%m/%d").unwrap();
    if hour >= 24 {
        hour -= 24;
    }
    let Some(minute) = minute.parse::<u32>().ok() else {
        let err = Error::new(
            ErrorKind::InvalidData,
            format!("Invalid minute: {}", minute),
        );
        return Err(err);
    };
    let name = name.trim();
    assert!(minute < 60, "{minute}");
    assert!(!name.is_empty(), "{name}");
    Ok((item_date, hour, minute, name))
}

struct ProgramItem<'a> {
    date: NaiveDate,
    hour: u32,
    minute: u32,
    name: &'a str,
    is_last: bool,
}

pub fn parse_content_map<'a>(
    contents: &'a str,
    program_map: &mut BTreeMap<NaiveDate, Program<'a>>,
) -> Result<bool, MyError> {
    let mut ret = true;

    let mut temp_date = "".to_string();
    let mut program_items: Vec<ProgramItem> = vec![];
    for (index, line) in contents.lines().enumerate() {
        let lineno = index + 1;
        let line = line.trim();
        match line {
            line if line.is_empty() | line.starts_with("节目长度") | line.contains("频道") => {
            }
            line if line.starts_with("播出日期") => {
                if let Some(item) = program_items.last_mut() {
                    item.is_last = true;
                }
                temp_date = parse_date(line).unwrap_or("".to_string());
            }
            line => match parse_list(temp_date.clone(), line) {
                Ok((date, hour, minute, name)) => {
                    let program_item = ProgramItem {
                        date,
                        hour,
                        minute,
                        name,
                        is_last: false,
                    };
                    program_items.push(program_item);
                }
                Err(err) => {
                    eprintln!("{}", LineError::new(lineno, line.to_string()));
                    eprintln!("error: {}", err);
                    ret = false;
                }
            },
        }
    }
    if let Some(item) = program_items.last_mut() {
        item.is_last = true;
    }
    for ProgramItem {
        mut date,
        hour,
        minute,
        name,
        is_last,
    } in program_items
    {
        let mut cross = false;
        if is_last && hour < 12 {
            date += TimeDelta::days(1);
            cross = true;
        }
        let item = Item::new(date, hour, minute, name, is_last, cross);

        if let std::collections::btree_map::Entry::Vacant(e) = program_map.entry(date) {
            let mut program = Program::default();
            program.add_item(item);
            e.insert(program);
        } else {
            program_map.get_mut(&date).unwrap().add_item(item);
        }
    }
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
        let date_str = "2026/11/18";
        let date = NaiveDate::parse_from_str(date_str, "%Y/%m/%d").unwrap();
        assert_eq!(
            parse_list(date_str.to_string(), "05:30 节目预告").unwrap(),
            (date, 05, 30, "节目预告")
        );
        assert_eq!(
            parse_list(date_str.to_string(), "05: 30 节目预告").unwrap(),
            (date, 05, 30, "节目预告")
        );
        assert_eq!(
            parse_list(date_str.to_string(), " 05:30 节目预告").unwrap(),
            (date, 05, 30, "节目预告")
        );
        assert_eq!(
            parse_list(date_str.to_string(), " 05: 30 节目预告").unwrap(),
            (date, 05, 30, "节目预告")
        );
        assert_eq!(
            parse_list(date_str.to_string(), "05:30节目预告").unwrap(),
            (date, 05, 30, "节目预告")
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_list_invalid() {
        let date_str = "2026/11/18";
        let date = NaiveDate::parse_from_str(date_str, "%Y/%m/%d").unwrap();
        assert_eq!(
            parse_list(date_str.to_string(), "05:303 节目预告").unwrap(),
            (date, 05, 30, "节目预告")
        );
    }
}

use std::collections::BTreeMap;

use super::parse_content;
use crate::{MyError, Program, Record};

pub fn parse_file(path: &str) -> Result<BTreeMap<String, Vec<Record>>, MyError> {
    let contents = std::fs::read_to_string(path)?;

    let mut programs: Vec<Program> = vec![];
    let _ = parse_content(&contents, &mut programs)?;
    let mut record_map: BTreeMap<String, Vec<Record>> = BTreeMap::new();
    for program in programs {
        for record in program.records()? {
            let program_date = program.date().to_string();
            let date = if program_date.eq(record.date()) {
                program_date.as_str()
            } else {
                record.date()
            };
            if let Some(records) = record_map.get_mut(date) {
                records.push(record);
            } else {
                record_map.insert(record.date().to_string(), vec![record]);
            };
        }
    }
    Ok(record_map)
}

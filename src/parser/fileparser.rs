use chardet::{charset2encoding, detect};
use chrono::NaiveDate;
use encoding::DecoderTrap;
use encoding::label::encoding_from_whatwg_label;
use std::{collections::BTreeMap, fs::OpenOptions, io::Read, path::Path};

use super::parse_content_map;
use crate::{MyError, Program, Record, write_map};

pub fn parse_file(path: &Path) -> Result<i32, MyError> {
    let mut fh = OpenOptions::new()
        .read(true)
        .open(path)
        .inspect_err(|_| eprintln!("Could not open file {}", path.to_string_lossy()))?;
    let mut reader: Vec<u8> = Vec::new();
    fh.read_to_end(&mut reader)
        .inspect_err(|_| eprintln!("Could not read file {}", path.to_string_lossy()))?;
    let (encode, _confidence, _language) = detect(&reader);
    let contents = if encode.ne("utf-8") {
        let Some(coder) = encoding_from_whatwg_label(charset2encoding(&encode)) else {
            let err = std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("cannot detect the encoding from label: {}", encode),
            );
            return Err(MyError::IoError(err));
        };
        let Some(contents) = coder.decode(&reader, DecoderTrap::Strict).ok() else {
            let err = std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "cannot decode the content of {} to encode {}",
                    path.to_string_lossy(),
                    encode
                ),
            );
            return Err(MyError::IoError(err));
        };
        contents.replace("：", ":")
    } else {
        let Some(contents) = String::from_utf8(reader).ok() else {
            return Err(MyError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid utf8 content: {}", path.to_string_lossy()),
            )));
        };
        contents
    };

    let mut record_map: BTreeMap<String, Vec<Record>> = BTreeMap::new();

    let mut program_map: BTreeMap<NaiveDate, Program> = BTreeMap::new();
    let _ = parse_content_map(&contents, &mut program_map)?;

    for (date, program) in program_map {
        let records = program.records()?;
        for record in records {
            if let Some(records) = record_map.get_mut(&date.to_string()) {
                records.push(record);
            } else {
                record_map.insert(record.date().to_string(), vec![record]);
            };
        }
    }

    write_map(&record_map, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        assert!(parse_file(Path::new("1.txt")).is_ok());
        assert!(parse_file(Path::new("2.txt")).is_ok());
        assert!(parse_file(Path::new("3.txt")).is_ok());
    }
}

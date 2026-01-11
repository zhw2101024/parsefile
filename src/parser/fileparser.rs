use chardet::{charset2encoding, detect};
use encoding::DecoderTrap;
use encoding::label::encoding_from_whatwg_label;
use std::{collections::BTreeMap, fs::OpenOptions, io::Read, path::Path};

use super::parse_content;
use crate::{MyError, Program, Record, write_map};

pub fn parse_file(path: &Path) -> Result<i32, MyError> {
    let mut fh = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("Could not open file");
    let mut reader: Vec<u8> = Vec::new();
    fh.read_to_end(&mut reader).expect("Could not read file");
    let (encode, _confidence, _language) = detect(&reader);
    let contents = if encode.ne("utf-8") {
        let Some(coder) = encoding_from_whatwg_label(charset2encoding(&encode)) else {
            let err = std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("cannot detect the encoding from label: {}", encode),
            );
            return Err(MyError::IoError(err));
        };
        coder
            .decode(&reader, DecoderTrap::Strict)
            .expect("Error")
            .replace("：", ":")
    } else {
        String::from_utf8(reader).unwrap()
    };

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

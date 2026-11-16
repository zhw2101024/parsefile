use crate::{MyError, Record};
use chardet::{charset2encoding, detect};
use chrono::{NaiveDateTime, Timelike};
use encoding::DecoderTrap;
use encoding::label::encoding_from_whatwg_label;
use rust_xlsxwriter::Workbook;

use std::fmt;
use std::{collections::BTreeMap, fs::OpenOptions, io::Read, path::Path};

#[derive(Clone)]
pub struct DataRecord {
    pub name: String,
    pub date: String,
    pub start: String,
    pub end: String,
    datetime: NaiveDateTime,
}

impl fmt::Display for DataRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.name, self.date, self.start, self.end)
    }
}

fn disabled(name: &str, srcpath: &Path) -> bool {
    let mut path = srcpath.to_path_buf();
    path.set_extension("white");
    let mut fh = match OpenOptions::new().read(true).open(&path) {
        Ok(fh) => fh,
        Err(_) => {
            eprintln!("Could not open file {}", path.to_string_lossy());
            return true;
        }
    };
    let mut reader: Vec<u8> = Vec::new();
    let _size = match fh.read_to_end(&mut reader) {
        Ok(size) => size,
        Err(_) => {
            eprintln!("Could not open file {}", path.to_string_lossy());
            return true;
        }
    };
    let (encode, _confidence, _language) = detect(&reader);
    let contents = if encode.ne("utf-8") {
        let Some(coder) = encoding_from_whatwg_label(charset2encoding(&encode)) else {
            return true;
        };
        let Some(contents) = coder.decode(&reader, DecoderTrap::Strict).ok() else {
            return true;
        };
        contents.replace("：", ":")
    } else {
        let Some(contents) = String::from_utf8(reader).ok() else {
            return true;
        };
        contents
    };

    let lines = contents.lines();
    for mut line in lines {
        line = line.trim();
        if line.eq(name) {
            return false;
        }
    }
    true
}

pub fn write_map(record_map: &BTreeMap<String, Vec<Record>>, path: &Path) -> Result<i32, MyError> {
    let mut number = 0;

    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write(0, 0, "直播状态填启用或禁用，默认禁用")?;

    worksheet.write(1, 0, "节目名称")?;
    worksheet.write(1, 1, "播放日期")?;
    worksheet.write(1, 2, "开始时间")?;
    worksheet.write(1, 3, "结束时间")?;
    worksheet.write(1, 4, "直播状态")?;

    let data_records = data_records(record_map);

    for (
        row_index,
        DataRecord {
            name,
            date,
            start,
            end,
            ..
        },
    ) in (2..).zip(data_records)
    {
        worksheet.write(row_index, 0, &name)?;
        worksheet.write(row_index, 1, date)?;
        worksheet.write(row_index, 2, start)?;
        worksheet.write(row_index, 3, end)?;
        let disabled_str = if disabled(&name, path) {
            "禁用"
        } else {
            "启用"
        };
        worksheet.write(row_index, 4, disabled_str)?;

        number += 1;
    }

    let mut destpath = path.to_path_buf();
    destpath.set_extension("xlsx");

    workbook.save(destpath)?;

    Ok(number)
}

fn data_records(record_map: &BTreeMap<String, Vec<Record>>) -> Vec<DataRecord> {
    let mut sum_records = Vec::new();
    for records in record_map.values() {
        for record in records {
            let datarecord = DataRecord {
                name: record.name.clone(),
                date: record.date.clone(),
                start: record.time.clone(),
                end: "".to_string(),
                datetime: record.datetime,
            };
            sum_records.push(datarecord)
        }
    }
    let mut fixed_records = Vec::new();
    for index in 0..sum_records.len() {
        let curr = &sum_records[index];
        if index < sum_records.len() - 1 {
            let next = &sum_records[index + 1];
            if next.datetime.date() > curr.datetime.date() {
                let mut record = curr.clone();
                record.end = "23:59:59".to_string();
                fixed_records.push(record);
                if next.datetime.time().num_seconds_from_midnight() > 0 {
                    let mut sep_record = curr.clone();
                    sep_record.date = next.date.clone();
                    sep_record.start = "00:00".to_string();
                    sep_record.end = next.start.clone();
                    fixed_records.push(sep_record);
                }
            } else {
                let mut record = curr.clone();
                record.end = next.start.clone();
                fixed_records.push(record);
            }
        } else {
            let record = curr.clone();
            fixed_records.push(record);
        }
    }
    fixed_records
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_records() {
        let record1 = Record {
            name: "foobar".to_string(),
            date: "2026-11-13".to_string(),
            time: "11:08".to_string(),
        };
        let record2 = Record {
            name: "foobar".to_string(),
            date: "2026-11-14".to_string(),
            time: "05:30".to_string(),
        };
        let mut record_map: BTreeMap<String, Vec<Record>> = BTreeMap::new();
        record_map.insert(record1.date().to_string(), vec![record1, record2]);
        let data_records = data_records(&record_map);

        let record1 = DataRecord {
            name: "foobar".to_string(),
            date: "2026-11-13".to_string(),
            start: "11:08".to_string(),
            end: "05:30".to_string(),
        };
        let record2 = DataRecord {
            name: "foobar".to_string(),
            date: "2026-11-14".to_string(),
            start: "05:30".to_string(),
            end: "".to_string(),
        };
        assert_eq!(data_records[0].to_string(), record1.to_string());
        assert_eq!(data_records[1].to_string(), record2.to_string());
    }
}

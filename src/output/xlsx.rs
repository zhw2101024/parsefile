use crate::{MyError, Record};
use rust_xlsxwriter::Workbook;
use std::{collections::BTreeMap, path::Path};

pub fn write_map(record_map: &BTreeMap<String, Vec<Record>>, path: &Path) -> Result<i32, MyError> {
    let mut number = 0;

    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write(0, 0, "节目名称")?;
    worksheet.write(0, 1, "节目时间")?;

    let mut row_index = 1;

    for records in record_map.values() {
        for Record { name, time, .. } in records {
            worksheet.write(row_index, 0, name)?;
            worksheet.write(row_index, 1, time)?;
            row_index += 1;

            number += 1;
        }
    }

    let mut destpath = path.to_path_buf();
    destpath.set_extension("xlsx");

    println!("destpath: {}", destpath.to_string_lossy());
    workbook.save(destpath)?;

    Ok(number)
}

use crate::{MyError, Record};
use rust_xlsxwriter::Workbook;
use std::collections::BTreeMap;

pub fn write_map(record_map: BTreeMap<String, Vec<Record>>) -> Result<bool, MyError> {
    let ret = true;

    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write(0, 0, "节目名称")?;
    worksheet.write(0, 1, "节目时间")?;

    let mut row_index = 1;

    for (_date, records) in record_map.iter() {
        for record in records {
            worksheet.write(row_index, 0, record.name())?;
            worksheet.write(row_index, 1, record.time())?;
            row_index += 1;
        }
    }

    workbook.save("hello_single.xlsx")?;

    Ok(ret)
}

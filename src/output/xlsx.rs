use crate::{MyError, Record};
use rust_xlsxwriter::Workbook;
use std::{collections::BTreeMap, path::Path};

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

    let mut row_index = 2;

    let mut sum_records = Vec::new();
    /*
    for records in record_map.values() {
        for Record { name, date, time } in records {
            worksheet.write(row_index, 0, name)?;
            worksheet.write(row_index, 1, date)?;
            worksheet.write(row_index, 2, time)?;
            worksheet.write(row_index, 3, "")?;
            worksheet.write(row_index, 4, "禁用")?;
            row_index += 1;

            number += 1;
        }
    }
    */
    for records in record_map.values() {
        for record in records {
            sum_records.push(record)
        }
    }

    for (i, Record { name, date, time }) in sum_records.clone().into_iter().enumerate() {
        let mut end = "";
        if i > 0 {
            end = &sum_records[i].time;
        }
        worksheet.write(row_index, 0, name)?;
        worksheet.write(row_index, 1, date)?;
        worksheet.write(row_index, 2, time)?;
        worksheet.write(row_index - 1, 3, end)?;
        worksheet.write(row_index, 4, "禁用")?;
        row_index += 1;

        number += 1;
    }

    let mut destpath = path.to_path_buf();
    destpath.set_extension("xlsx");

    workbook.save(destpath)?;

    Ok(number)
}

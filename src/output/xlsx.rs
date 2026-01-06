use crate::{MyError, Record};
use rust_xlsxwriter::Workbook;
use std::{collections::BTreeMap, path::Path};

pub fn write_map(record_map: &BTreeMap<String, Vec<Record>>, path: &Path) -> Result<bool, MyError> {
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

    let parent = path
        .parent()
        .unwrap_or_else(|| panic!("unable to get parent for path: {}", path.to_string_lossy()));
    let prefix_os = path
        .file_prefix()
        .unwrap_or_else(|| panic!("unable to get prefix for path: {}", path.to_string_lossy()));
    let prefix = prefix_os
        .to_str()
        .unwrap_or_else(|| panic!("{}", prefix_os.display().to_string()));
    let destname = format!("{}.xlsx", prefix);
    let destpath = parent.join(destname);
    workbook.save(destpath)?;

    Ok(ret)
}

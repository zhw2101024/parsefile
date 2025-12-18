use crate::{MyError, Program};
use rust_xlsxwriter::Workbook;

pub fn write(programs: &Vec<Program>) -> Result<bool, MyError> {
    let ret = true;

    let mut workbook = Workbook::new();

    for program in programs {
        let worksheet = workbook.add_worksheet();
        let date = program.get_date();
        worksheet.set_name(&date)?;

        worksheet.write(0, 0, "节目名称")?;
        worksheet.write(0, 1, "节目时间")?;

        let mut row_index = 1;

        for item in program.get_items() {
            let (time, name) = item.get_info();

            worksheet.write(row_index, 0, name)?;
            worksheet.write(row_index, 1, format!("{} {}", date, time))?;
            row_index += 1
        }
    }

    workbook.save("hello.xlsx")?;

    Ok(ret)
}

pub fn write_single(programs: &Vec<Program>) -> Result<bool, MyError> {
    let ret = true;

    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write(0, 0, "节目名称")?;
    worksheet.write(0, 1, "节目时间")?;

    let mut row_index = 1;

    for program in programs {
        let date = program.get_date();
        for item in program.get_items() {
            let (time, name) = item.get_info();

            worksheet.write(row_index, 0, name)?;
            worksheet.write(row_index, 1, format!("{} {}", date, time))?;
            row_index += 1;
        }
    }

    workbook.save("hello_single.xlsx")?;

    Ok(ret)
}

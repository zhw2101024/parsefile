use crate::{MyError, Record};
use chrono::NaiveDate;
use chrono::prelude::*;

#[derive(Clone, Debug)]
pub struct Item<'a> {
    date: NaiveDate,
    hour: u32,
    minute: u32,
    name: &'a str,
}

impl<'a> Item<'a> {
    pub fn new(date: NaiveDate, hour: u32, minute: u32, name: &'a str) -> Self {
        Self {
            date,
            hour,
            minute,
            name,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Program<'a> {
    items: Vec<Item<'a>>,
}

impl<'a> Program<'a> {
    pub fn empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn add_item(&mut self, item: Item<'a>) {
        self.items.push(item);
    }

    pub fn records(&self) -> Result<Vec<Record>, MyError> {
        let mut records: Vec<Record> = vec![];
        for Item {
            date,
            hour,
            minute,
            name,
        } in &self.items
        {
            assert!(*hour < 48, "{hour}");

            let Some(time) = NaiveTime::from_hms_opt(*hour, *minute, 0) else {
                let error = MyError::IoError(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("无效时间： {}:{}", hour, minute),
                ));
                return Err(error);
            };
            let datetime = date.and_time(time);

            let time = format!("{:0>2}:{:0>2}", hour, minute);

            records.push(Record::new(
                name.to_string(),
                date.to_string(),
                time,
                datetime,
            ));
        }
        Ok(records)
    }
}

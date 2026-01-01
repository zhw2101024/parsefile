use crate::{MyError, Record};
use chrono::TimeDelta;
use chrono::prelude::*;

#[derive(Clone, Debug)]
pub struct Item<'a> {
    hour: u32,
    minute: u32,
    name: &'a str,
}

impl<'a> Item<'a> {
    pub fn new(hour: u32, minute: u32, name: &'a str) -> Self {
        Self { hour, minute, name }
    }

    pub fn hour(&self) -> u32 {
        self.hour
    }

    pub fn minute(&self) -> u32 {
        self.minute
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

#[derive(Clone, Debug, Default)]
pub struct Program<'a> {
    date: chrono::NaiveDate,
    items: Vec<Item<'a>>,
}

impl<'a> Program<'a> {
    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn set_date(&mut self, date: &str) -> Result<(), chrono::ParseError> {
        self.date = NaiveDate::parse_from_str(date, "%Y/%m/%d")?;
        Ok(())
    }

    pub fn add_item(&mut self, item: Item<'a>) {
        self.items.push(item);
    }

    pub fn records(&self) -> Result<Vec<Record>, MyError> {
        let mut records: Vec<Record> = vec![];
        for Item { hour, minute, name } in &self.items {
            assert!(*hour < 48, "{hour}");

            let date;
            let time = if *hour < 24 {
                date = self.date.to_string();
                format!("{} {:0>2}:{:0>2}", self.date, hour, minute)
            } else {
                let hour = hour - 24;
                let Some(time) = NaiveTime::from_hms_opt(hour, *minute, 0) else {
                    let error = MyError::IoError(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("无效时间： {}:{}", hour, minute),
                    ));
                    return Err(error);
                };
                let datetime = self.date().and_time(time);
                let datetime = datetime + TimeDelta::days(1);

                let year = datetime.year();
                let month = datetime.month();
                let day = datetime.day();
                date = format!("{year}-{:0>2}-{:0>2}", month, day);
                format!(
                    "{year}-{:0>2}-{:0>2} {:0>2}:{:0>2}",
                    month, day, hour, minute
                )
            };

            records.push(Record::new(name.to_string(), date, time));
        }
        Ok(records)
    }
}

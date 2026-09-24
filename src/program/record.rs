use chrono::NaiveDateTime;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Record {
    pub name: String,
    pub date: String,
    pub time: String,
    pub datetime: NaiveDateTime,
}

impl Record {
    pub fn new(name: String, date: String, time: String, datetime: NaiveDateTime) -> Record {
        Record {
            name,
            date,
            time,
            datetime,
        }
    }

    pub fn date(&self) -> &str {
        &self.date
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.name, self.date, self.time)
    }
}

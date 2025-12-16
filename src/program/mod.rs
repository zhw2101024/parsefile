use chrono::prelude::*;

#[derive(Clone, Debug)]
pub struct Item<'a> {
    hour: u8,
    minute: u8,
    name: String,
    _source: &'a str,
}

impl<'a> Item<'a> {
    pub fn new(hour: u8, minute: u8, name: String, source: &'a str) -> Self {
        Self {
            hour,
            minute,
            name,
            _source: source,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Program<'a> {
    date: DateTime<FixedOffset>,
    items: Vec<Item<'a>>,
}

impl<'a> Program<'a> {
    pub fn empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn set_date(&mut self, date: &str) {
        match DateTime::parse_from_str(
            &(date.to_owned() + " 00:00:00 +08:00"),
            "%Y/%m/%d %H:%M:%S %z",
        ) {
            Ok(date) => {
                self.date = date;
            }
            Err(e) => {
                println!("Error parsing date: {}, source: {}", e, date);
            }
        }
    }

    pub fn add_item(&mut self, item: Item<'a>) {
        self.items.push(item);
    }

    pub fn print(&self) {
        println!("date: {}", self.date);
        self.items.iter().for_each(|item| {
            println!("values: {},{},{}", item.hour, item.minute, item.name);
        });
    }
}

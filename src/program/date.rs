use chrono::prelude::*;

#[derive(Clone, Debug)]
pub struct Item {
    hour: u8,
    minute: u8,
    name: String,
}

impl Item {
    pub fn new(hour: u8, minute: u8, name: String) -> Self {
        Self { hour, minute, name }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Program {
    date: chrono::NaiveDate,
    items: Vec<Item>,
}

impl Program {
    pub fn empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn set_date(&mut self, date: &str) -> Result<(), chrono::ParseError> {
        self.date = NaiveDate::parse_from_str(date, "%Y/%m/%d")?;
        Ok(())
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn print(&self) {
        println!("date: {}", self.date);
        self.items.iter().for_each(|item| {
            println!("values: {},{},{}", item.hour, item.minute, item.name);
        });
    }
}

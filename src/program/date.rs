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

    pub fn get_info(&self) -> (String, String) {
        let time = format!("{:0>2}:{:0>2}", self.hour, self.minute);
        (time, self.name.clone())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Program {
    date: chrono::NaiveDate,
    items: Vec<Item>,
}

impl Program {
    pub fn get_date(&self) -> String {
        self.date.to_string()
    }

    pub fn get_items(&self) -> &Vec<Item> {
        &self.items
    }

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
}

#[derive(Clone, Debug)]
pub struct Record {
    pub name: String,
    date: String,
    pub time: String,
}

impl Record {
    pub fn new(name: String, date: String, time: String) -> Record {
        Record { name, date, time }
    }

    pub fn date(&self) -> &str {
        &self.date
    }
}

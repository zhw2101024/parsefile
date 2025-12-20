#[derive(Clone, Debug)]
pub struct Record {
    name: String,
    date: String,
    time: String,
}

impl Record {
    pub fn new(name: String, date: String, time: String) -> Record {
        Record { name, date, time }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn date(&self) -> &str {
        &self.date
    }

    pub fn time(&self) -> &str {
        &self.time
    }
}

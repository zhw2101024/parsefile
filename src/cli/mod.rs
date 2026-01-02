use crate::{ConcreteSubject, Observer, Subject};

pub struct ConcreteObserver {
    name: String,
}
impl ConcreteObserver {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
impl Observer for ConcreteObserver {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn update(&self, message: &str) -> String {
        let msg = format!("{} received message: {}", self.name, message);
        println!("{msg}");
        msg
    }
}

pub fn add_observer(subject: &mut ConcreteSubject) {
    let observer1 = Box::new(ConcreteObserver::new("cli".to_string()));
    subject.attach(observer1);
}

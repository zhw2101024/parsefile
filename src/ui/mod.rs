mod hard;

pub use hard::hello;

use crate::{ConcreteSubject, Observer, Subject};
use std::cell::LazyCell;

pub static mut SUBJECT: LazyCell<ConcreteSubject> = LazyCell::new(ConcreteSubject::default);

pub struct ConcreteObserver {
    name: String,
}
impl ConcreteObserver {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }
}
impl Observer for ConcreteObserver {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn update(&self, message: &str) -> String {
        let msg = format!("{} received message: {}", self.name, message);
        msg
    }
}

pub fn add_observer(subject: &mut ConcreteSubject, name: &str) {
    let observer1 = Box::new(ConcreteObserver::new(name));
    subject.attach(observer1);
}

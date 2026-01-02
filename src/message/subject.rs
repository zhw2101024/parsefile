use crate::message::Observer;

// 定义被观察者接口
pub trait Subject {
    fn attach(&mut self, observer: Box<dyn Observer>);
    fn _detach(&mut self, observer: Box<dyn Observer>);
    fn notify(&self, message: &str) -> Vec<String>;
}
// 实现具体被观察者
#[derive(Default)]
pub struct ConcreteSubject {
    observers: Vec<Box<dyn Observer>>,
}
impl Subject for ConcreteSubject {
    fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }
    fn _detach(&mut self, observer: Box<dyn Observer>) {
        self.observers.retain(|o| o.ne(&observer));
    }
    fn notify(&self, message: &str) -> Vec<String> {
        let mut msgs = vec![];
        for observer in &self.observers {
            msgs.push(observer.update(message));
        }
        msgs
    }
}

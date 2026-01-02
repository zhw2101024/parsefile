mod observer;
mod subject;

use observer::{ConcreteObserver, Observer};
use subject::{ConcreteSubject, Subject};

pub fn monitor() {
    // 创建具体观察者对象
    let observer1 = Box::new(ConcreteObserver::new("Observer 1".to_string()));
    let observer2 = Box::new(ConcreteObserver::new("Observer 2".to_string()));
    // 创建具体被观察者对象
    let mut subject = ConcreteSubject::default();
    // 注册观察者
    subject.attach(observer1);
    subject.attach(observer2);
    // 发送通知
    subject.notify("Hello, observers!");
}

#[cfg(test)]
mod tests {
    use super::*;

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
            msg
        }
    }

    #[test]
    fn test_message_update() {
        // 创建具体观察者对象
        let observer1 = Box::new(ConcreteObserver::new("Observer 1".to_string()));
        let observer2 = Box::new(ConcreteObserver::new("Observer 2".to_string()));
        // 创建具体被观察者对象
        let mut subject = ConcreteSubject::default();
        // 注册观察者
        subject.attach(observer1);
        subject.attach(observer2);
        // 发送通知
        assert_eq!(
            subject.notify("Hello, observers!"),
            vec![
                "Observer 1 received message: Hello, observers!",
                "Observer 2 received message: Hello, observers!"
            ]
        );
    }
}

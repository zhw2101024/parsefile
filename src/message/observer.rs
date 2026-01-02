// 定义观察者接口
pub trait Observer {
    fn update(&self, message: &str) -> String;
    fn get_name(&self) -> &str;
}
// 实现具体观察者
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
impl PartialEq for Box<dyn Observer> {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name()
    }
}

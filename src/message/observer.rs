// 定义观察者接口
pub trait Observer {
    fn get_name(&self) -> &str;
    fn update(&self, message: &str) -> String;
}

impl PartialEq for Box<dyn Observer> {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name()
    }
}

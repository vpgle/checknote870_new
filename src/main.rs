use std::collections::HashMap;
#[derive(Debug)]
struct Datastruct {
    key: String,
    a_value: Vec<String>,
    b_value: HashMap<String, String>,
    c_value: Vec<String>,
}
impl Datastruct {
    fn average(&self) -> u64 {
        // 这是计算均值部分
        // 计算b_value中数据的均值
        println!("{:?}", &self);
        999_999_999
    }
}
fn main() {
    dbg!("Hello, world!");
}

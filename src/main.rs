#[warn(dead_code)]
use std::fmt;
use std::collections::HashMap;
#[derive(Debug)]
struct Datastruct {
    key: String,
    a_value: Vec<String>,
    b_value: HashMap<String, String>,
    c_value: Vec<String>,
}
impl Datastruct {
    fn average(self: &Self) -> u64 {
        // 这是计算均值部分
        // 计算b_value中数据的均值
        println!("{}", self);

        dbg!("{}", &self.b_value);

        let hmap: &HashMap<String, String>= &self.b_value;
        println!("{:?}", hmap);

        999_999_999
    }
}
impl fmt::Display for Datastruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(\n    key: {:?},\na_value: {:?},\nb_value: {:?},\nc_value: {:?},\n)",
               self.key, self.a_value, self.b_value, self.c_value)
    }
}
fn main() {
    let mut b_value = HashMap::new();
    b_value.insert( "a".to_string(), "b".to_string());
    let m = Datastruct {
        key: "key".to_string(),
        a_value: vec!["a_value".to_string()],
        b_value: b_value,
        c_value: vec!["c_value".to_string()],
    };
//  println!("{}", m);

    println!("{:?}", m.average());
}

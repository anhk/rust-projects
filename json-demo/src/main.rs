use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    school: String,
}

fn main() {
    let json = r#"{
        "name": "张三丰",
        "age": 18,
        "school": "武当山"
    }"#;

    let v: serde_json::Value = serde_json::from_str(json).unwrap();
    println!(
        "#1 name={}, age={}, school={}",
        v["name"], v["age"], v["school"]
    );

    let p: Person = serde_json::from_str(json).unwrap();
    println!("#2 name={}, age={}, school={}", p.name, p.age, p.school);

    let s2 = serde_json::to_string(&p).unwrap();
    println!("#3 {}", s2);

    let j3 = json!({
        "name": "张三丰",
        "age": 18,
        "school": "武当山"
    } );
    println!(
        "#4 name={}, age={}, school={}",
        j3["name"], j3["age"], j3["school"]
    );
}

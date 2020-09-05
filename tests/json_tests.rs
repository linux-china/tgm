use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::env;
use std::fs;
use chrono::{DateTime, Local, Datelike};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub contact: Contact,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    email: String,
    pub phones: Vec<String>,
    address: String,
}

#[test]
fn test_path() {
    let home = env::var("HOME").unwrap();
    let file_name = home + "/.tgm/settings.json";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    println!("{}", contents);
}

#[test]
fn test_json() -> Result<()> {
    let contact = Contact {
        email: "libing.chen@gmail.com".into(),
        phones: vec!["185".into(), "186".into()],
        address: "中国杭州".into(),
    };
    let person = Person {
        name: String::from("linux_china"),
        age: 40,
        contact,
    };
    let json_text = serde_json::to_string(&person).unwrap();
    println!("{}", json_text);
    let person2: Person = serde_json::from_str(json_text.as_str())?;
    assert_eq!(person.name, person2.name);
    println!("{:?}", person2);

    Ok(())
}

#[test]
fn test_chrono() {
    let now: DateTime<Local> = Local::now();
    println!("{}",now.year());
    println!("{}",format!("{}/{}/{}", now.month(),now.day(),now.year()));
}

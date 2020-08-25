use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use std::env;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub templates: Vec<Template>,
    pub variables: Vec<Variable>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub name: String,
    pub repository: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Variable {
    pub name: String,
    pub value: String,
    pub description: String,
}

impl Settings {
    pub fn load() -> Settings {
        let home = env::var("HOME").unwrap();
        let setting_json_path = home + "/.tgm/settings.json";
        let file_path = Path::new(&setting_json_path);
        if file_path.exists() {
            let setting_json = fs::read_to_string(setting_json_path)
                .expect("Failed to read ~/.tgm/settings.json");
            serde_json::from_str(setting_json.as_str()).unwrap()
        } else {
            Settings { templates: vec![], variables: vec![] }
        }
    }
}

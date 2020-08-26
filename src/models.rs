use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::thread::sleep;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct AppTemplate {
    pub name: String,
    pub repository: String,
    pub description: String,
    pub auto_run: String,
    pub variables: Vec<Variable>,
    pub files: Vec<String>,
}

impl Settings {
    pub fn load() -> Settings {
        let home = env::var("HOME").unwrap();
        let setting_json_path = home + "/.tgm/settings.json";
        let file_path = Path::new(&setting_json_path);
        if file_path.exists() {
            let setting_json =
                fs::read_to_string(setting_json_path).expect("Failed to read ~/.tgm/settings.json");
            serde_json::from_str(setting_json.as_str()).unwrap()
        } else {
            Settings {
                templates: vec![],
                variables: vec![],
            }
        }
    }

    pub fn fresh_settings(settings: &Settings) {
        let home = env::var("HOME").unwrap();
        let tgm_home = format!("{}/.tgm", home);
        let tgm_path = Path::new(&tgm_home);
        if !tgm_path.exists() {
            std::fs::create_dir_all(tgm_path);
        }
        let setting_json_path = home + "/.tgm/settings.json";
        let mut file = File::create(Path::new(&setting_json_path)).unwrap();
        let json_text = serde_json::to_string_pretty(&settings).unwrap();
        file.write_all(json_text.as_bytes()).unwrap();
    }

    pub fn find_template(&self, template_name: &String) -> Option<&Template> {
        for template in self.templates.iter() {
            if template_name == &template.name {
                return Some(template);
            }
        }
        return None;
    }

    pub fn add_template(&mut self, name: String, url: String) {
        if self.find_template(&name).is_none() {
            self.templates.push(Template { name, repository: url, description: String::from("desc") });
            Settings::fresh_settings(self);
        } else {
            println!("{} template already exits!", name);
        }
    }
}

impl AppTemplate {
    pub fn new(template_json_file: &String) -> AppTemplate {
        let setting_json =
            fs::read_to_string(template_json_file).expect("Failed to read template.json");
        serde_json::from_str(setting_json.as_str()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let settings = Settings::load();
        println!("{:?}", settings);
        assert!(!settings.templates.is_empty());
    }

    #[test]
    fn test_find_template() {
        let settings = Settings::load();
        let template_name = String::from("spring-boot-java");
        let template = settings.find_template(&template_name).unwrap();
        println!("template description: {}", template.description);
    }

    #[test]
    fn test_app_template() {
        let app_template_file = String::from("temp/demo/template.json");
        let app_template = AppTemplate::new(&app_template_file);
        println!("{:?}", app_template);
    }
}

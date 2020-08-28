use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
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
    pub value: Option<String>,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppTemplate {
    pub name: String,
    pub repository: String,
    pub description: String,
    pub post_create: Option<String>,
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

    pub fn fresh_settings(&self) {
        let home = env::var("HOME").unwrap();
        let tgm_home = format!("{}/.tgm", home);
        let tgm_path = Path::new(&tgm_home);
        if !tgm_path.exists() {
            std::fs::create_dir_all(tgm_path).unwrap();
        }
        let setting_json_path = home + "/.tgm/settings.json";
        let mut file = File::create(Path::new(&setting_json_path)).unwrap();
        let json_text = serde_json::to_string_pretty(self).unwrap();
        file.write_all(json_text.as_bytes()).unwrap();
    }

    pub fn find_template(&self, template_name: &str) -> Option<&Template> {
        for template in self.templates.iter() {
            if *template_name == template.name {
                return Some(template);
            }
        }
        None
    }

    pub fn add_template(&mut self, name: String, url: String, description: String) {
        if self.find_template(&name).is_none() {
            self.templates.push(Template {
                name: name.clone(),
                repository: url,
                description,
            });
            self.fresh_settings();
            println!("{} template added!", name);
        } else {
            println!("{} template already exits!", name);
        }
    }

    pub fn delete_template(&mut self, name: &str) {
        if self.find_template(name).is_some() {
            self.templates.retain(|t| t.name != *name);
            self.fresh_settings();
            println!("{} template removed!", name);
        } else {
            println!("{} template not found!", name);
        }
    }
}

impl AppTemplate {
    pub fn new(template_json_file: &str) -> AppTemplate {
        let path = Path::new(template_json_file);
        if path.exists() {
            let setting_json =
                fs::read_to_string(template_json_file).expect("Failed to read template.json");
            serde_json::from_str(setting_json.as_str()).unwrap()
        } else {
            AppTemplate::default()
        }
    }

    pub fn fetch_remote(url: &str) -> reqwest::Result<AppTemplate> {
        reqwest::blocking::get(url)?.json::<AppTemplate>()
    }
}

impl Default for AppTemplate {
    fn default() -> Self {
        AppTemplate {
            name: String::from("unknown"),
            description: String::from("not available"),
            repository: String::from("not available"),
            variables: vec![],
            files: vec![],
            post_create: Some(String::from("Desc absent")),
        }
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
        let template_name = "spring-boot-java";
        let template = settings.find_template(&template_name).unwrap();
        println!("template description: {}", template.description);
    }

    #[test]
    fn test_app_template() {
        let app_template_file = "temp/demo/template.json";
        let app_template = AppTemplate::new(&app_template_file);
        println!("{:?}", app_template);
    }

    #[test]
    fn test_fetch_remote_template() -> reqwest::Result<()> {
        let url = "https://raw.githubusercontent.com/linux-china/spring-boot-java-template/master/template.json";
        let app_template = AppTemplate::fetch_remote(&url).unwrap();
        println!("{:?}", app_template);
        Ok(())
    }
}

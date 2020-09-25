use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub central: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppTemplate {
    pub name: String,
    pub repository: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_create: Option<String>,
    pub variables: Vec<Variable>,
    pub files: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubRepo {
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub html_url: String,
}

impl GithubRepo {
    pub fn fetch_tgm_template_repos(org_name: &str) -> reqwest::Result<Vec<GithubRepo>> {
        let url = format!("https://api.github.com/orgs/{}/repos?type=public", org_name);
        let response = Client::builder()
            .build()?
            .get(&url)
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "Awesome-tgm-App")
            .send()?;
        let mut repos = response.json::<Vec<GithubRepo>>()?;
        if repos.len() >= 2 {
            repos.sort_by(|a, b| a.name.cmp(&b.name));
        }
        Ok(repos)
    }
}

impl Settings {
    pub fn load() -> Settings {
        let home = env::var("HOME").unwrap();
        let setting_json_path = home + "/.tgm/settings.json";
        let file_path = Path::new(&setting_json_path);
        if file_path.exists() {
            let setting_json =
                fs::read_to_string(setting_json_path).expect("Failed to read ~/.tgm/settings.json");
            let mut settings: Settings = serde_json::from_str(setting_json.as_str()).unwrap();
            if settings.templates.len() > 1 {
                settings.templates.sort_by(|a, b| a.name.cmp(&b.name));
            }
            if settings.variables.len() > 1 {
                settings.variables.sort_by(|a, b| a.name.cmp(&b.name));
            }
            settings
        } else {
            Settings {
                central: None,
                templates: vec![],
                variables: vec![],
            }
        }
    }
}

impl Settings {
    pub fn flush(&self) {
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
            self.flush();
            println!("{} template added!", name);
        } else {
            println!("{} template already exits!", name);
        }
    }

    pub fn delete_template(&mut self, name: &str) {
        if self.find_template(name).is_some() {
            self.templates.retain(|t| t.name != *name);
            self.flush();
            println!("{} template removed!", name);
        } else {
            println!("{} template not found!", name);
        }
    }

    pub fn find_variable_value(&self, name: &str) -> Option<String> {
        for variable in self.variables.iter() {
            if variable.name == name {
                return variable.value.clone();
            }
        }
        None
    }

    pub fn set_variable(&mut self, name: &str, value: &str, description: &str) {
        for variable in self.variables.iter_mut() {
            if variable.name == name {
                variable.value = Some(String::from(value));
                variable.description = String::from(description);
                return;
            }
        }
        self.variables.push(Variable {
            name: String::from(name),
            value: Some(String::from(value)),
            description: String::from(description),
            pattern: None,
        });
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

    pub fn with_remote(url: &str) -> reqwest::Result<AppTemplate> {
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
        let url = "https://gist.githubusercontent.com/linux-china/50d0ad9db30489951dc66ecfa4fe2785/raw/8cef649356a4b073e4d55e0221eff97f31133522/template.json";
        let app_template = AppTemplate::with_remote(&url).unwrap();
        println!("{:?}", app_template);
        Ok(())
    }

    #[test]
    fn test_github_repos() -> reqwest::Result<()> {
        let repos = GithubRepo::fetch_tgm_template_repos("tgm-templates")?;
        println!("{:?}", repos);
        Ok(())
    }
}

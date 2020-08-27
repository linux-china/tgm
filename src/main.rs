mod models;

use crate::models::AppTemplate;
use colored::*;
use models::Settings;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

const VERSION: &str = "0.1.3";

fn main() {
    let sub_command = env::args().nth(1);
    let settings = Settings::load();
    if let Some(command) = sub_command {
        match command.as_str() {
            "list" => {
                list_templates(&settings);
            }
            "add" => {
                let name = env::args().nth(2).unwrap();
                let url_arg = env::args().nth(3);
                if let Some(url) = url_arg {
                    add_template(&name, &url, "Desc absent!");
                } else {
                    let mut url = name.clone();
                    if !(url.starts_with("http://") || url.starts_with("https://")) {
                        // github template repository
                        url = format!("https://raw.githubusercontent.com/{}/master/template.json", name);
                    }
                    match AppTemplate::fetch_remote(&url) {
                        Ok(app_template) => {
                            add_template(&app_template.name, &app_template.repository, &app_template.description);
                        }
                        Err(_e) => {
                            println!("{}", format!("Failed to load template from {}, please check the json data!", url).as_str().red());
                        }
                    }
                }
            }
            "remove" => {
                let name_arg = env::args().nth(2);
                if let Some(name) = name_arg {
                    delete_template(&name);
                } else {
                    println!("{}", "Please specify template name!".red());
                }
            }
            "create" => {
                let template_name = env::args().nth(2);
                let app_dir_arg = env::args().nth(3);
                if let Some(app_dir) = app_dir_arg {
                    let current_dir = String::from(std::env::current_dir().unwrap().to_str().unwrap());
                    create_app(&template_name.unwrap(), &current_dir, &app_dir, &settings);
                    //check app created or not
                    let dest_dir = format!("{}/{}", current_dir, app_dir);
                    let dest_path = Path::new(&dest_dir);
                    if dest_path.exists() {
                        println!("{}", format!("app created successfully under {} directory!", app_dir).as_str().green());
                    }
                } else {
                    println!("{}", "Please specify the destination directory!".red());
                }
            }
            "sync" => {
                let dest_dir = String::from(std::env::current_dir().unwrap().to_str().unwrap());
                prompt_input_variables(&settings, &dest_dir);
            }
            "help" => {
                display_help();
            }
            _ => {
                let hint = format!("Command not found: {}", command).as_str().red();
                println!("{}", hint);
            }
        }
    } else {
        println!("Please use sub commands: list, create, add, remove, help etc");
    }
}

fn add_template(name: &str, url: &str, description: &str) {
    let mut settings = Settings::load();
    settings.add_template(name.into(), url.into(), description.into());
}

fn delete_template(name: &str) {
    let mut settings = Settings::load();
    settings.delete_template(name);
}

fn list_templates(settings: &Settings) {
    if settings.templates.is_empty() {
        println!("No template available! Please use '{}' to add new template.", "tgm add name repo_url".green());
    } else {
        for template in settings.templates.iter() {
            println!("{} - {} : {}", template.name.as_str().blue(), template.repository, template.description);
        }
    }
}

fn create_app(template_name: &str, workspace_dir: &str, app_dir: &str, settings: &Settings) {
    let dest_dir = format!("{}/{}", workspace_dir, app_dir);
    if let Some(template) = settings.find_template(&template_name) {
        let dest_path = Path::new(&dest_dir);
        if !dest_path.exists() {
            println!("Beginning to create app from {}", template.name);
            let args = vec![
                "clone",
                "--depth",
                "1",
                template.repository.as_str(),
                app_dir,
            ];
            match execute_command("git", &args) {
                Ok(stdout_text) => {
                    println!("{}", stdout_text);
                }
                Err(e) => {
                    println!("{}", e.as_str().red());
                }
            }
        }
        // template variables input
        prompt_input_variables(&settings, &dest_dir);
    } else {
        println!("{}", format!("Template not found: {}", template_name).as_str().red());
    }
}

fn display_help() {
    println!("tgm-{}: https://github.com/linux-china/tgm", VERSION);
    println!("sub commands: list, create, add, remove etc")
}

fn execute_command(command: &str, args: &[&str]) -> Result<String, String> {
    let result = Command::new(command).args(args).output();
    match result {
        Ok(output) => {
            if output.status.success() {
                std::str::from_utf8(output.stdout.as_slice())
                    .map(String::from)
                    .map_err(|e| e.to_string())
            } else {
                Ok(String::from("good"))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

fn prompt_input_variables(_settings: &Settings, app_dest_dir: &str) {
    let template_json_file = format!("{}/template.json", app_dest_dir);
    let app_template = AppTemplate::new(&template_json_file);
    let mut variables = HashMap::<String, String>::new();
    if !app_template.variables.is_empty() {
        println!("Please complete template variables.");
        for v in app_template.variables.iter() {
            print!("{}({}){}", v.name.as_str().green(), v.description, ">".blue());
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            variables.insert(format!("@{}@", v.name), String::from(input.trim()));
        }
        for file in app_template.files.iter() {
            let resource_file = format!("{}/{}", app_dest_dir, file);
            replace_variables(&resource_file, &variables);
        }
    }
    std::env::set_current_dir(Path::new(app_dest_dir)).unwrap();
    // re-init
    execute_command("rm", &["-rf", ".git"]).unwrap();
    execute_command("git", &["init"]).unwrap();
    // post create
    if !app_template.post_create.is_some() {
        let post_create = app_template.post_create.unwrap();
        if !post_create.is_empty() {
            let parts: Vec<&str> = post_create.split(' ').collect();
            println!("Begin to execute post_create: {}", post_create);
            let args: Vec<&str> = parts[1..].to_vec();
            match execute_command(parts[0], &args) {
                Ok(stdout_text) => {
                    println!("{}", stdout_text);
                }
                Err(e) => {
                    println!("{}", e.as_str().red());
                }
            }
        }
    }
}

fn replace_variables(resource_file: &str, variables: &HashMap<String, String>) {
    let path = Path::new(resource_file);
    let mut replaced_text = fs::read_to_string(path).unwrap();
    for (k, v) in variables.iter() {
        replaced_text = replaced_text.replacen(k.as_str(), v.as_str(), 1024);
    }
    let mut file = File::create(path).unwrap();
    file.write_all(replaced_text.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_templates() {
        let settings = Settings::load();
        list_templates(&settings);
    }

    #[test]
    fn test_create_app() {
        let settings = Settings::load();
        let template_name = "spring-boot-java";
        let app_dir = "temp/demo";
        let current_dir = String::from(std::env::current_dir().unwrap().to_str().unwrap());
        create_app(template_name, &current_dir, app_dir, &settings);
    }

    #[test]
    fn test_add_template() {
        let name = "demo";
        let url = "git://xxx";
        let description = "no description";
        add_template(name, url, description);
    }

    #[test]
    fn test_delete_template() {
        let name = "demo";
        delete_template(name);
    }

    #[test]
    fn test_display_help() {
        display_help();
    }
}

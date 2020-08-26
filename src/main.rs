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

fn main() {
    let sub_command = env::args().nth(1);
    let settings = Settings::load();
    if let Some(command) = sub_command {
        match command.as_str() {
            "list" => {
                list_templates(&settings);
            }
            "add" => {
                let name_arg = env::args().nth(2);
                let url_arg = env::args().nth(3);
                if let Some(url) = url_arg {
                    add_template(&name_arg.unwrap(), &url);
                } else {
                    println!("{}", "Please specify template name and git url!".red());
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
                let template_name = String::from("spring-boot-java");
                let app_dir_arg = env::args().nth(2);
                if let Some(app_dir) = app_dir_arg {
                    create_app(&template_name, &app_dir, &settings);
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

fn add_template(name: &String, url: &String) {
    let mut settings = Settings::load();
    settings.add_template(name.clone(), url.clone());
}

fn delete_template(name: &String) {
    let mut settings = Settings::load();
    settings.delete_template(name);
}

fn list_templates(settings: &Settings) {
    for template in settings.templates.iter() {
        println!("{} - {} : {}", template.name.as_str().blue(), template.repository, template.description);
    }
}

fn create_app(template_name: &String, app_dir: &String, settings: &Settings) {
    let current_dir = std::env::current_dir().unwrap();
    let dest_dir = format!("{}/{}", current_dir.to_str().unwrap(), app_dir);
    if let Some(template) = settings.find_template(&template_name) {
        let dest_path = Path::new(&dest_dir);
        if !dest_path.exists() {
            println!("Beginning to clone {}", template.name);
            let args = vec![
                "clone",
                "https://github.com/linux-china/spring-boot-java-template.git",
                dest_dir.as_str(),
            ];
            if let Ok(stdout_text) = execute_command("git", &args) {
                println!("{}", stdout_text);
            }
        }
        // change work directory
        std::env::set_current_dir(Path::new(&dest_dir)).unwrap();
        prompt_input_variables(&settings, &dest_dir);
    } else {
        println!("Template not found: {}", template_name);
    }
}

fn display_help() {
    println!("tgm: https://github.com/linux-china/tgm");
    println!("sub commands: list, clone, sync")
}

fn execute_command(command: &str, args: &Vec<&str>) -> Result<String, String> {
    let result = Command::new(command).args(args.as_slice()).output();
    match result {
        Ok(output) => {
            if output.status.success() {
                std::str::from_utf8(output.stdout.as_slice())
                    .map(|x| String::from(x))
                    .map_err(|e| e.to_string())
            } else {
                Ok(String::from("good"))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

fn prompt_input_variables(_settings: &Settings, app_dest_dir: &String) {
    let template_json_file = format!("{}/template.json", app_dest_dir);
    let app_template = AppTemplate::new(&template_json_file);
    let mut variables = HashMap::<String, String>::new();
    for v in app_template.variables.iter() {
        print!("{}>", v.name.as_str().green());
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

fn replace_variables(resource_file: &String, variables: &HashMap<String, String>) {
    let path = Path::new(resource_file);
    let content = fs::read_to_string(path).unwrap();
    let mut replaced_text = content.clone();
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
        let template_name = String::from("spring-boot-java");
        let app_dir = String::from("temp/demo");
        create_app(&template_name, &app_dir, &settings);
    }

    #[test]
    fn test_add_template() {
        let name = String::from("demo");
        let url = String::from("git://xxx");
        add_template(&name, &url);
    }

    #[test]
    fn test_delete_template() {
        let name = String::from("demo");
        delete_template(&name);
    }

    #[test]
    fn test_display_help() {
        display_help();
    }
}

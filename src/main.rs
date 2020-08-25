mod models;

use std::{env, fs};
use models::Settings;
use std::process::Command;
use crate::models::AppTemplate;
use std::fs::File;
use std::collections::HashMap;
use std::path::Path;
use std::io::Write;
use colored::*;

fn main() {
    let sub_command = env::args().nth(1);
    let settings = Settings::load();
    if let Some(command) = sub_command {
        match command.as_str() {
            "list" => {
                list_templates(&settings);
            }
            "clone" => {
                let template_name = String::from("spring-boot-java");
                //env::args().nth(2)
                let app_dir = String::from("temp/demo");
                clone_template(&template_name, &app_dir, &settings);
            }
            "sync" => {
                sync_template_variables();
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
        println!("Please use sub commands: list, clone, help etc");
    }
}

fn list_templates(settings: &Settings) {
    for template in settings.templates.iter() {
        println!("{} - {}", template.name, template.description);
    }
}

fn clone_template(template_name: &String, app_dir: &String, settings: &Settings) {
    let current_dir = std::env::current_dir().unwrap();
    let dest_dir = format!("{}/{}", current_dir.to_str().unwrap(), app_dir);
    if let Some(template) = settings.find_template(&template_name) {
        let dest_path = Path::new(&dest_dir);
        if !dest_path.exists() {
            println!("Beginning to clone {}", template.name);
            let args = vec!["clone", "https://github.com/linux-china/spring-boot-java-template.git", dest_dir.as_str()];
            if let Ok(stdout_text) = execute_command("git", &args) {
                println!("{}", stdout_text);
            }
        }
        // change work directory
        std::env::set_current_dir(Path::new(&dest_dir));
        prompt_input_variables(&settings, &dest_dir);
    } else {
        println!("Template not found: {}", template_name);
    }
}

fn sync_template_variables() {
    let dest_dir = String::from(std::env::current_dir().unwrap().to_str().unwrap());
    prompt_input_variables(&settings, &dest_dir);
}

fn display_help() {
    println!("Display help")
}

fn execute_command(command: &str, args: &Vec<&str>) -> Result<String, String> {
    let result = Command::new(command)
        .args(args.as_slice())
        .output();
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
        Err(e) => Err(e.to_string())
    }
}


fn prompt_input_variables(settings: &Settings, app_dest_dir: &String) {
    let template_json_file = format!("{}/template.json", app_dest_dir);
    let app_template = AppTemplate::new(&template_json_file);
    let mut variables = HashMap::<String, String>::new();
    for v in app_template.variables.iter() {
        println!("{}>", v.name.as_str().green());
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
    fn test_clone_template() {
        let settings = Settings::load();
        let template_name = String::from("spring-boot-java");
        let app_dir = String::from("temp/demo");
        clone_template(&template_name, &app_dir, &settings);
    }

    #[test]
    fn test_variables_replace() {
        let settings = Settings::load();
        let app_dest_dir = String::from("temp/demo");
    }

    #[test]
    fn test_display_help() {
        display_help();
    }
}

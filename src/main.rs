mod models;

use std::env;
use models::Settings;
use std::process::Command;
use std::path::Path;


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
            "help" => {
                display_help();
            }
            _ => {
                println!("Command not found: {}", command);
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
        println!("Beginning to clone {}", template.name);
        let args = vec!["clone", "https://github.com/linux-china/spring-boot-java-template.git", dest_dir.as_str()];
        if let Ok(stdout_text) = execute_command("git", &args) {
            println!("{}", stdout_text);
            // change work directory
            //std::env::set_current_dir(Path::new(&dest_dir));
        }
    } else {
        println!("Template not found: {}", template_name);
    }
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
    fn test_display_help() {
        display_help();
    }
}

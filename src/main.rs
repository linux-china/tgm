mod models;

use std::fs;
use std::env;
use models::{Settings, Template, Variable};

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
                clone_template(&template_name, &settings);
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

fn clone_template(template_name: &String, settings: &Settings) {
    if let Some(template) = settings.find_template(&template_name) {
        println!("Beginning to clone {}", template.name);
    } else {
        println!("Template not found: {}", template_name);
    }
}

fn display_help() {
    println!("Display help")
}

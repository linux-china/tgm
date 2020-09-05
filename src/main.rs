mod models;

use crate::models::AppTemplate;
use chrono::{DateTime, Datelike, Local};
use clap::{App, Arg, SubCommand};
use colored::*;
use models::Settings;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

const VERSION: &str = "0.3.0";

fn main() {
    let add_command = SubCommand::with_name("add")
        .about("Add template")
        .arg(
            Arg::with_name("name")
                .long("name") // allow --name
                .takes_value(true)
                .help("template name")
                .required(true),
        )
        .arg(
            Arg::with_name("repo")
                .long("repo") // allow --name
                .takes_value(true)
                .help("git repository url")
                .required(true),
        )
        .arg(
            Arg::with_name("desc")
                .long("desc") // allow --name
                .takes_value(true)
                .help("template description")
                .required(true),
        );
    let create_command = SubCommand::with_name("create")
        .about("create app from template")
        .arg(
            Arg::with_name("name")
                //.long("name") // allow --name
                .takes_value(true)
                .help("template name")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("dir")
                //.long("dir") // allow --name
                .takes_value(true)
                .help("App's directory")
                .required(true)
                .index(2),
        );
    let remove_command = SubCommand::with_name("remove")
        .about("remove template")
        .arg(
            Arg::with_name("name")
                .takes_value(true)
                .help("template name")
                .required(true),
        );
    let import_command = SubCommand::with_name("import")
        .about("import template from repository's template.json")
        .arg(
            Arg::with_name("name")
                .long("name")
                .takes_value(true)
                .help("github's repository name or absolute url")
                .required(true),
        );
    // init Clap
    let matches = App::new("tgm")
        .version(VERSION)
        .about("template generator manager: https://github.com/linux-china/tgm")
        .author("linux_china")
        .subcommand(SubCommand::with_name("list").about("list templates"))
        .subcommand(SubCommand::with_name("config").about("Config global variables"))
        .subcommand(add_command)
        .subcommand(remove_command)
        .subcommand(import_command)
        .subcommand(create_command)
        .get_matches();

    let settings = Settings::load();
    let (sub_command, args_match) = matches.subcommand();
    if sub_command == "list" {
        list_templates(&settings);
    } else if sub_command == "config" {
        config_global_variables();
    } else if sub_command == "add" {
        let args = args_match.unwrap();
        let name = args.value_of("name").unwrap();
        let repo = args.value_of("repo").unwrap();
        let desc = args.value_of("desc").unwrap();
        add_template(name, repo, desc);
    } else if sub_command == "import" {
        let args = args_match.unwrap();
        let mut url = String::from(args.value_of("name").unwrap());
        if !(url.starts_with("http://") || url.starts_with("https://")) {
            // github template repository
            url = format!(
                "https://raw.githubusercontent.com/{}/master/template.json",
                url
            );
        } else {
            println!(
                "{}",
                "repository's url should start with http:// or https://".red()
            );
        }
        if !url.ends_with("/template.json") {
            url = format!("{}/template.json", url);
        }
        match AppTemplate::with_remote(&url) {
            Ok(app_template) => {
                add_template(
                    &app_template.name,
                    &app_template.repository,
                    &app_template.description,
                );
            }
            Err(_e) => {
                println!(
                    "{}",
                    format!(
                        "Failed to load template from {}, please check the json data!",
                        url
                    )
                        .as_str()
                        .red()
                );
            }
        }
    } else if sub_command == "remove" {
        let args = args_match.unwrap();
        let name = args.value_of("name").unwrap();
        delete_template(name);
    } else if sub_command == "create" {
        let args = args_match.unwrap();
        let template_name = args.value_of("name").unwrap();
        let app_dir = args.value_of("dir").unwrap();
        println!("{}:{}", template_name, app_dir);
        let current_dir = String::from(std::env::current_dir().unwrap().to_str().unwrap());
        create_app(template_name, &current_dir, app_dir, &settings);
        //check app created or not
        let dest_dir = format!("{}/{}", current_dir, app_dir);
        let dest_path = Path::new(&dest_dir);
        if dest_path.exists() {
            println!(
                "{}",
                format!("app created successfully under {} directory!", app_dir)
                    .as_str()
                    .green()
            );
        }
    } else {
        println!("{}", "No subcommand was used".red());
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
        println!(
            "No template available! Please use '{}' commands to add new template.",
            "add or import".green()
        );
    } else {
        for template in settings.templates.iter() {
            println!(
                "{} - {} : {}",
                template.name.as_str().blue(),
                template.repository,
                template.description
            );
        }
    }
}

fn config_global_variables() {
    let variable_names = vec![
        ("author_name", "author name"),
        ("author_email", "your email"),
        ("github_user_name", "your Github user name"),
    ];
    let mut settings = Settings::load();
    for pair in variable_names.iter() {
        let global_variable = settings.find_variable_value(&pair.0);
        if let Some(variable_value) = global_variable.clone() {
            print!(
                "Define value for variable '{}'({}): {} : {}",
                pair.0.green(),
                pair.1,
                variable_value,
                ">".blue()
            );
        } else {
            print!(
                "Define value for variable '{}'({}){}",
                pair.0.green(),
                pair.1,
                ">".blue()
            );
        }
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() {
            if let Some(variable_value) = global_variable.clone() {
                input = variable_value.clone();
            }
        }
        settings.set_variable(pair.0, &input.trim(), pair.1);
    }
    settings.flush();
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
        println!(
            "{}",
            format!("Template not found: {}", template_name)
                .as_str()
                .red()
        );
    }
}

fn execute_command(command: &str, args: &[&str]) -> Result<String, String> {
    let result = Command::new(command).args(args).output();
    match result {
        Ok(output) => std::str::from_utf8(output.stdout.as_slice())
            .map(String::from)
            .map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

fn prompt_input_variables(settings: &Settings, app_dest_dir: &str) {
    let template_json_file = format!("{}/template.json", app_dest_dir);
    let app_template = AppTemplate::new(&template_json_file);
    let mut variables = HashMap::<String, String>::new();
    //default global variables
    let now: DateTime<Local> = Local::now();
    variables.insert(String::from("current_year"), now.year().to_string());
    variables.insert(String::from("current_date"), now.format("%m/%d/%Y").to_string());
    //os related variables
    variables.insert(String::from("os_name"), String::from(std::env::consts::OS));
    variables.insert(
        String::from("os_family"),
        String::from(std::env::consts::FAMILY),
    );
    variables.insert(
        String::from("os_arch"),
        String::from(std::env::consts::ARCH),
    );
    if !app_template.variables.is_empty() {
        println!("Please complete template variables.");
        for v in app_template.variables.iter() {
            let global_variable = settings.find_variable_value(&v.name);
            if let Some(variable_value) = global_variable.clone() {
                print!(
                    "Define value for variable '{}'({}): {} : {}",
                    v.name.as_str().green(),
                    v.description,
                    variable_value,
                    ">".blue()
                );
            } else {
                print!(
                    "Define value for variable '{}'({}){}",
                    v.name.as_str().green(),
                    v.description,
                    ">".blue()
                );
            }
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim().is_empty() {
                if let Some(variable_value) = global_variable.clone() {
                    input = variable_value.clone();
                }
            }
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
    if let Some(post_create) = app_template.post_create {
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
}

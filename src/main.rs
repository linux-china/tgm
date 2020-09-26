mod licenses;
mod models;

use crate::models::{AppTemplate, GithubRepo, Variable};
use chrono::{DateTime, Datelike, Local};
use clap::{App, Arg};
use clap_generate::generators::Bash;
use clap_generate::{generate, generators::Zsh};
use colored::*;
use licenses::get_license;
use models::Settings;
use regex::Regex;
use std::collections::HashMap;
use std::{fs, env};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

const VERSION: &str = "0.8.0";

fn build_app() -> App<'static> {
    let add_command = App::new("add")
        .about("Add template")
        .arg(
            Arg::new("name")
                .long("name") // allow --name
                .takes_value(true)
                .about("template name")
                .required(true),
        )
        .arg(
            Arg::new("repo")
                .long("repo") // allow --name
                .takes_value(true)
                .about("git repository url")
                .required(true),
        )
        .arg(
            Arg::new("desc")
                .long("desc") // allow --name
                .takes_value(true)
                .about("template description")
                .required(true),
        );
    let create_command = App::new("create")
        .about("Create app from template")
        .arg(
            Arg::new("name")
                //.long("name") // allow --name
                .takes_value(true)
                .about("template name")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("dir")
                //.long("dir") // allow --name
                .takes_value(true)
                .about("App's directory")
                .required(true)
                .index(2),
        );
    let remove_command = App::new("remove")
        .about("Remove template from local settings")
        .arg(
            Arg::new("name")
                .takes_value(true)
                .about("template name")
                .required(true),
        );
    let list_command = App::new("list").about("List templates").arg(
        Arg::new("remote")
            .long("remote")
            .takes_value(false)
            .about("remotes template")
            .required(false),
    );
    let config_command = App::new("config")
        .about("Show/config global variables")
        .arg(
            Arg::new("edit")
                .long("edit")
                .takes_value(false)
                .about("edit global variables ")
                .required(false),
        );
    let complete_command = App::new("complete")
        .about("Generate shell completion for zsh & bash")
        .arg(
            Arg::new("zsh")
                .long("zsh")
                .takes_value(false)
                .about("Zsh completion")
                .required(false),
        ).arg(
        Arg::new("oh_my_zsh")
            .long("oh_my_zsh")
            .takes_value(false)
            .about("Oh My Zsh")
            .required(false),
    )
        .arg(
            Arg::new("bash")
                .long("bash")
                .takes_value(false)
                .about("Bash completion")
                .required(false),
        );
    let import_command = App::new("import")
        .about("Import template from repository's template.json")
        .arg(
            Arg::new("name")
                .takes_value(true)
                .about("github's repository name or absolute url")
                .required(true),
        );
    let license_command = App::new("license")
        .about("Generate LICENSE file")
        .arg(
            Arg::new("apache2")
                .long("apache2")
                .takes_value(false)
                .about("Apache License 2.0")
                .required(false),
        )
        .arg(
            Arg::new("mit")
                .long("mit")
                .takes_value(false)
                .about("MIT License")
                .required(false),
        )
        .arg(
            Arg::new("isc")
                .long("isc")
                .takes_value(false)
                .about("ISC License")
                .required(false),
        )
        .arg(
            Arg::new("gplv3")
                .long("gplv3")
                .takes_value(false)
                .about("GNU GPLv3 ")
                .required(false),
        )
        .arg(
            Arg::new("lgplv3")
                .long("lgplv3")
                .takes_value(false)
                .about("GNU LGPLv3")
                .required(false),
        )
        .arg(
            Arg::new("mozilla2")
                .long("mozilla2")
                .takes_value(false)
                .about("Mozilla Public License 2.0")
                .required(false),
        )
        .arg(
            Arg::new("author")
                .long("author")
                .takes_value(true)
                .about("Author name")
                .required(true),
        );
    // init Clap
    App::new("tgm")
        .version(VERSION)
        .about("Template generator manager: https://github.com/linux-china/tgm")
        .subcommand(list_command)
        .subcommand(config_command)
        .subcommand(license_command)
        .subcommand(complete_command)
        .subcommand(add_command)
        .subcommand(remove_command)
        .subcommand(import_command)
        .subcommand(create_command)
}

fn main() {
    let app = build_app();
    let matches = app.get_matches();
    if matches.subcommand().is_none() {
        println!(
            "{}",
            "😂 Please use subcommand or --help to display help!".red()
        );
        return;
    }
    let settings = Settings::load();
    let (sub_command, args) = matches.subcommand().unwrap();
    if sub_command == "list" {
        if args.is_present("remote") {
            list_remote_templates(&settings);
        } else {
            list_templates(&settings);
        }
    } else if sub_command == "config" {
        if args.is_present("edit") {
            config_global_variables();
        } else {
            show_global_variables(&settings);
        }
    } else if sub_command == "license" {
        let author_name = args.value_of("author").unwrap_or("Anonymous");
        let license_type = if args.is_present("apache2") {
            "apache2"
        } else if args.is_present("mit") {
            "mit"
        } else if args.is_present("isc") {
            "isc"
        } else if args.is_present("gplv3") {
            "gplv3"
        } else if args.is_present("lgplv3") {
            "lgplv3"
        } else if args.is_present("mozilla2") {
            "mozilla2"
        } else {
            ""
        };
        if license_type.is_empty() {
            println!("{}", "Unknown license type".red());
            return;
        }
        let license_text = get_license(license_type, author_name);
        let mut license_file = File::create(Path::new("LICENSE")).unwrap();
        license_file.write_all(license_text.as_bytes()).unwrap();
        println!("📄 LICENSE file created!")
    } else if sub_command == "complete" {
        if args.is_present("zsh") {
            generate::<Zsh, _>(&mut build_app(), "tgm", &mut std::io::stdout());
        } else if args.is_present("bash") {
            generate::<Bash, _>(&mut build_app(), "tgm", &mut std::io::stdout());
        } else if args.is_present("oh_my_zsh") {
            let home = env::var("HOME").unwrap();
            let dest_dir = format!("{}/.oh-my-zsh/custom/plugins/tgm", home);
            let result = std::fs::create_dir_all(Path::new(&dest_dir));
            if result.is_ok() {
                let mut file = File::create("_tgm").unwrap();
                generate::<Zsh, _>(&mut build_app(), "tgm", &mut file);
            } else {
                println!("Failed to create directory: {}", &dest_dir);
            }
        }
    } else if sub_command == "add" {
        let name = args.value_of("name").unwrap();
        let repo = args.value_of("repo").unwrap();
        let desc = args.value_of("desc").unwrap();
        add_template(name, repo, desc);
    } else if sub_command == "import" {
        let mut url = String::from(args.value_of("name").unwrap());
        if !(url.starts_with("http://") || url.starts_with("https://")) {
            // github template repository
            if !url.contains("/") {
                // template from https://github.com/tgm-templates/
                url = format!("tgm-templates/{}", url);
            }
            url = format!(
                "https://raw.githubusercontent.com/{}/master/template.json",
                url
            );
        } else {
            println!(
                "{}",
                "😂 Repository's url should start with http:// or https://".red()
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
        let name = args.value_of("name").unwrap();
        delete_template(name);
    } else if sub_command == "create" {
        let template_name = args.value_of("name").unwrap();
        let app_dir = args.value_of("dir").unwrap();
        let current_dir = String::from(std::env::current_dir().unwrap().to_str().unwrap());
        let dest_dir = format!("{}/{}", current_dir, app_dir);
        let dest_path = Path::new(&dest_dir);
        if dest_path.exists() {
            println!("😂  '{}' directory exits already!", app_dir);
            return;
        }
        create_app(template_name, &current_dir, app_dir, &settings);
        //check app created or not
        if dest_path.exists() {
            println!(
                "{}",
                format!("💯 App created successfully under {} directory!", app_dir)
                    .as_str()
                    .green()
            );
        }
    } else {
        let hint = format!("😂 Unknown sub command: {}", sub_command);
        println!("{}", hint.as_str().red());
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
        let mut i = 1;
        for template in settings.templates.iter() {
            println!(
                "{}. {} - {} : {}",
                i,
                template.name.as_str().blue(),
                template.repository,
                template.description
            );
            i = i + 1;
        }
    }
}

fn list_remote_templates(settings: &Settings) {
    let org_name = get_central(settings);
    if let Ok(repos) = GithubRepo::fetch_tgm_template_repos(&org_name) {
        let mut i = 1;
        for repo in repos {
            println!(
                "{}. {} - {} : {}",
                i,
                repo.name.as_str().blue(),
                repo.html_url,
                repo.description
            );
            i = i + 1;
        }
    } else {
        println!("Failed to fetch remote templates");
    }
}

fn show_global_variables(settings: &Settings) {
    println!("======Global Variables=========");
    for variable in settings.variables.iter() {
        if variable.value.is_some() {
            println!("{}: {}", &variable.name, variable.value.clone().unwrap());
        }
    }
}

fn config_global_variables() {
    let variable_names = vec![
        ("author_name", "author's name"),
        ("author_email", "author's email"),
        ("github_user_name", "author's Github user name"),
        ("open_source_license", "Open Source License"),
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

fn get_central(settings: &Settings) -> String {
    let mut org_name = String::from("tgm-templates");
    if settings.central.is_some() {
        org_name = settings.central.clone().unwrap();
    }
    org_name
}

fn create_app(template_name: &str, workspace_dir: &str, app_dir: &str, settings: &Settings) {
    let dest_dir = format!("{}/{}", workspace_dir, app_dir);
    let mut repo_url: String = String::new();
    if let Some(template) = settings.find_template(&template_name) {
        repo_url = template.repository.clone();
    } else {
        // load template from https://github.com/tgm-templates/
        let org_name = get_central(settings);
        if let Ok(repos) = GithubRepo::fetch_tgm_template_repos(&org_name) {
            for repo in repos {
                if repo.name == template_name {
                    repo_url = repo.html_url.clone();
                    break;
                }
            }
        }
    }
    println!("repo: {}", repo_url);
    if !repo_url.is_empty() {
        println!("🚴 Beginning to create app from {}", template_name);
        let args = vec!["clone", "--depth", "1", repo_url.as_str(), app_dir];
        match execute_command("git", &args) {
            Ok(stdout_text) => {
                println!("{}", stdout_text);
            }
            Err(e) => {
                println!("{}", e.as_str().red());
            }
        }
        // template variables input
        prompt_input_variables(&settings, &dest_dir);
    } else {
        println!(
            "{}",
            format!("😂 Template not found: {}", template_name)
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
    variables.insert(
        String::from("current_date"),
        now.format("%m/%d/%Y").to_string(),
    );
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
    if app_template.variables.is_some() {
        println!("🤗 Please complete template variables.");
        for v in app_template.variables.unwrap().iter() {
            let mut value = prompt_input_variable(settings, v);
            // regex pattern match - only once
            if v.pattern.is_some() {
                let pattern = v.pattern.clone().unwrap();
                if let Ok(regex) = Regex::new(&pattern) {
                    if !regex.is_match(&value) {
                        let hint = format!(
                            "😅 '{}' is illegal, and should match with '{}' regex pattern!",
                            value, pattern
                        );
                        println!("{}", hint.as_str().red());
                        value = prompt_input_variable(settings, v);
                    }
                }
            }
            variables.insert(format!("@{}@", v.name), String::from(value));
        }
        if app_template.files.is_some() {
            for file in app_template.files.unwrap().iter() {
                let resource_file = format!("{}/{}", app_dest_dir, file);
                replace_variables(&resource_file, &variables);
            }
        }
    }
    std::env::set_current_dir(Path::new(app_dest_dir)).unwrap();
    // re-init
    execute_command("rm", &["-rf", ".git"]).unwrap_or(String::new());
    execute_command("git", &["init"]).unwrap_or(String::new());
    // post create
    if let Some(post_create) = app_template.post_create {
        if !post_create.is_empty() {
            let parts: Vec<&str> = post_create.split(' ').collect();
            println!("🏃 Begin to execute post_create: {}", post_create);
            let mut args: Vec<&str> = vec![];
            if parts.len() > 1 {
                args = parts[1..].to_vec();
            }
            let _ = Command::new(parts[0])
                .args(&args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .unwrap();
        }
    }
    // delete template.json
    execute_command("rm", &["-rf", "template.json"]).unwrap_or(String::new());
}

fn prompt_input_variable(settings: &Settings, v: &Variable) -> String {
    let global_variable = settings.find_variable_value(&v.name);
    let mut default_value = String::new();
    if global_variable.is_some() {
        default_value = global_variable.clone().unwrap();
    } else if v.value.is_some() {
        default_value = v.value.clone().unwrap();
    }
    if !default_value.is_empty() {
        print!(
            "👉 Define value for variable '{}'({}): {} : {}",
            v.name.as_str().green(),
            v.description,
            default_value,
            ">".blue()
        );
    } else {
        print!(
            "👉 Define value for variable '{}'({}){}",
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
    String::from(input.trim())
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

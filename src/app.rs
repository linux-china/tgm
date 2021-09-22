//! clap App for command cli
use clap::{App, Arg};

const VERSION: &str = "0.9.1";

pub fn build_app() -> App<'static> {
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
        )
        .arg(
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
        .license("Apache-2.0")
        .subcommand(list_command)
        .subcommand(config_command)
        .subcommand(license_command)
        .subcommand(complete_command)
        .subcommand(add_command)
        .subcommand(remove_command)
        .subcommand(import_command)
        .subcommand(create_command)
}

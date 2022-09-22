//! clap App for command cli
use clap::{Command, Arg, ArgAction};

const VERSION: &str = "0.10.0";

pub fn build_app() -> Command {
    let add_command = Command::new("add")
        .about("Add template")
        .arg(
            Arg::new("name")
                .long("name") // allow --name
                .num_args(1)
                .help("template name")
                .required(true),
        )
        .arg(
            Arg::new("repo")
                .long("repo") // allow --name
                .num_args(1)
                .help("git repository url")
                .required(true),
        )
        .arg(
            Arg::new("desc")
                .long("desc") // allow --name
                .num_args(1)
                .help("template description")
                .required(true),
        );
    let create_command = Command::new("create")
        .about("Create app from template")
        .arg(
            Arg::new("name")
                //.long("name") // allow --name
                .num_args(1)
                .help("template name")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("dir")
                //.long("dir") // allow --name
                .num_args(1)
                .help("App's directory")
                .required(true)
                .index(2),
        );
    let remove_command = Command::new("remove")
        .about("Remove template from local settings")
        .arg(
            Arg::new("name")
                .num_args(1)
                .help("template name")
                .required(true),
        );
    let list_command = Command::new("list").about("List templates").arg(
        Arg::new("remote")
            .long("remote")
            .num_args(1)
            .help("remotes template")
            .required(false),
    );
    let config_command = Command::new("config")
        .about("Show/config global variables")
        .arg(
            Arg::new("edit")
                .long("edit")
                .action(ArgAction::SetTrue)
                .help("edit global variables ")
                .required(false),
        );
    let complete_command = Command::new("complete")
        .about("Generate shell completion for zsh & bash")
        .arg(
            Arg::new("zsh")
                .long("zsh")
                .action(ArgAction::SetTrue)
                .help("Zsh completion")
                .required(false),
        )
        .arg(
            Arg::new("oh_my_zsh")
                .long("oh_my_zsh")
                .action(ArgAction::SetTrue)
                .help("Oh My Zsh")
                .required(false),
        )
        .arg(
            Arg::new("bash")
                .long("bash")
                .action(ArgAction::SetTrue)
                .help("Bash completion")
                .required(false),
        );
    let import_command = Command::new("import")
        .about("Import template from repository's template.json")
        .arg(
            Arg::new("name")
                .num_args(1)
                .help("github's repository name or absolute url")
                .required(true),
        );
    let license_command = Command::new("license")
        .about("Generate LICENSE file")
        .arg(
            Arg::new("apache2")
                .long("apache2")
                .action(ArgAction::SetTrue)
                .help("Apache License 2.0")
                .required(false),
        )
        .arg(
            Arg::new("mit")
                .long("mit")
                .action(ArgAction::SetTrue)
                .help("MIT License")
                .required(false),
        )
        .arg(
            Arg::new("isc")
                .long("isc")
                .action(ArgAction::SetTrue)
                .help("ISC License")
                .required(false),
        )
        .arg(
            Arg::new("gplv3")
                .long("gplv3")
                .action(ArgAction::SetTrue)
                .help("GNU GPLv3 ")
                .required(false),
        )
        .arg(
            Arg::new("lgplv3")
                .long("lgplv3")
                .action(ArgAction::SetTrue)
                .help("GNU LGPLv3")
                .required(false),
        )
        .arg(
            Arg::new("mozilla2")
                .long("mozilla2")
                .action(ArgAction::SetTrue)
                .help("Mozilla Public License 2.0")
                .required(false),
        )
        .arg(
            Arg::new("author")
                .long("author")
                .num_args(1)
                .help("Author name")
                .required(true),
        );
    // init Clap
    Command::new("tgm")
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

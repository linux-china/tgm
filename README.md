Template Generator Management
=============================

Generate project structure from git template repository

# tgm Settings

~/.tgm/settings.json 保存所有的配置信息，如模板列表，全局变量等。

# Template repository
在template repository中会包含一个template.json，包括模板的信息，目前主要是模板变量。

在项目模板的文件中，要被替换的变量格式为 @project@

#  commands:

* list: 列出所有项目模板
* clone: 生成项目模板
* sync: 和模板进行同步


# References

* Command line utilities: https://lib.rs/command-line-utilities
* Command-line apps in Rust: https://www.rust-lang.org/what/cli
* Serde: framework for serializing and deserializing Rust data structures https://serde.rs/

tgm - Template Generator Manager
================================
![Build Status](https://img.shields.io/github/workflow/status/linux-china/tgm/Rust)
[![tgm Crate](https://img.shields.io/crates/v/tgm)](https://crates.io/crates/tgm)

Generate project structure from git template repository.

Why tgm?

* Manage all template repositories
* Create app from template quickly
* Prompt for template variables

# Template repository
Please add template.json file in your template repository, code as following：

```json
{
  "name": "spring-boot-java",
  "repository": "https://github.com/linux-china/spring-boot-java-template",
  "description": "Spring Boot App Java",
  "post_create": "mvn -DskipTests compile",
  "variables": [
    {
      "name": "groupId",
      "description": "Maven groupId"
    },
    {
      "name": "artifactId",
      "description": "Maven artifactId"
    }
  ],
  "files": [
    "pom.xml","src/main/resources/application.properties"
  ]
}
```

**Attention:** You can add regex pattern validation for variable's value like following:

```json
    {
      "name": "email",
      "description": "author email",
      "pattern": "[\\w-\\.]+@([\\w-]+\\.)+[\\w-]{2,4}"
    }
```

In the resource files, such as pom.xml, use template variable as following:

```xml
<project>
  <groupId>@groupId@</groupId>
  <artifactId>@artifactId@</artifactId>
</project>
```

*default global variables:*

* current_year: current year, such as 2020
* current_date: current date, format like 08/30/2020
* os_name: linux, macos, ios, freebsd, dragonfly, netbsd, openbsd, solaris, android, windows
* os_family: unix, windows
* os_arch: x86_64, arm

# Install & Usage

```
$ cargo install tgm
$ tgm add linux-china/spring-boot-java-template
$ tgm list
$ tgm create spring-boot-java spring-app-demo
```

# tgm commands:

* list: list local templates
* list --remote:  list templates from https://github.com/tgm-templates/
* add: add new template from github template repository or manual

```
$ tgm add --name spring-boot-java --repo https://github.com/linux-china/spring-boot-java-template.git --desc "Spring Boot Java template"
```

* import template from github's repository

```
$ tgm import linux-china/spring-boot-kotlin-template
$ tgm import https://github.com/linux-china/spring-boot-java-template
```

* remove: remove template

```
$ tgm remove spring-boot-java
```

* create: create app from template

```
$ tgm create spring-boot-java spring-demo1 
```

# Shell completion
It's simple, just execute:

```
source < (tgm complete --zsh) 
```

### oh-my-zsh

```
$ tgm complete --oh_my_zsh
```

Modify .zshrc and add 'tgm' in plugins array.

```
plugins(tgm ...
```

### bash

```
$ tgm complete --bash > tgm-completion.bash
$ source ./tgm-completion.bash 
```

# References

* Command line utilities: https://lib.rs/command-line-utilities
* Command-line apps in Rust: https://www.rust-lang.org/what/cli
* Serde: framework for serializing and deserializing Rust data structures https://serde.rs/

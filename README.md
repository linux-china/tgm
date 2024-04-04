tgm - Template Generator Manager
================================
![Build Status](https://img.shields.io/github/workflow/status/linux-china/tgm/Rust)
[![tgm Crate](https://img.shields.io/crates/v/tgm)](https://crates.io/crates/tgm)
[![dependency status](https://deps.rs/crate/tgm/0.10.0/status.svg)](https://deps.rs/crate/tgm/0.10.0)

Generate project structure from git template repository, 
and ignited by [MicroServices Service Template Pattern](https://microservices.io/patterns/service-template.html)

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
* add: add new template from GitHub template repository or manual

```
$ tgm add --name spring-boot-java --repo https://github.com/linux-china/spring-boot-java-template.git --desc "Spring Boot Java template"
```

* import template from GitHub's repository

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

### oh-my-zsh

```
$ tgm complete --oh_my_zsh
```

tgm will add tgm to plugins in ~/.zshrc.

### bash

```
$ tgm complete --bash > tgm-completion.bash
$ source ./tgm-completion.bash 
```

# References

* Command line utilities: https://lib.rs/command-line-utilities
* Command-line apps in Rust: https://www.rust-lang.org/what/cli
* Serde: framework for serializing and deserializing Rust data structures https://serde.rs/

Template Generator Management
=============================

Generate project structure from git template repository.

# Template repository
Please add template.json file in your template repository, code as following：

```json
{
  "name": "spring-boot-java",
  "repository": "https://github.com/linux-china/spring-boot-java-template",
  "description": "Spring Boot App Java",
  "auto_run": "mvn -DskipTests compile",
  "variables": [
    {
      "name": "groupId",
      "value": "org.mvnsearch",
      "description": "Maven groupId"
    },
    {
      "name": "artifactId",
      "value": "spring-boot-demo",
      "description": "Maven artifactId"
    }
  ],
  "files": [
    "demo.txt",
    "pom.xml"
  ]
}
```

In the resource files, such as pom.xml, use template variable as following:

```xml
<project>
  <groupId>@groupId@</groupId>
  <artifactId>@artifactId@</artifactId>
</project>
```

# Usage

```
$ tgm add spring-boot-java https://github.com/linux-china/spring-boot-java-template.git
$ tgm list
$ tgm clone spring-boot-java  spring-app-demo
```


# tgm commands:

* list: list all templates
* add: add new template

```
$ tgm add spring-boot-java https://github.com/linux-china/spring-boot-java-template.git
```

* remove: remove template

```
$ tgm remove spring-boot-java
```

* create: create app from template

```
$ tgm create spring-boot-java spring-demo1 
```

# References

* Command line utilities: https://lib.rs/command-line-utilities
* Command-line apps in Rust: https://www.rust-lang.org/what/cli
* Serde: framework for serializing and deserializing Rust data structures https://serde.rs/

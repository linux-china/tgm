//! Open source licenses for Apache License 2.0, MIT License, ISC License, GNU GPLv3, GNU LGPLv3 and Mozilla Public License 2.0
use chrono::{DateTime, Datelike, Local};

pub fn get_license(license_name: &str, author_name: &str) -> String {
    let name = license_name.to_uppercase();
    let now: DateTime<Local> = Local::now();
    let year: &str = &now.year().to_string();
    return if name.contains("APACHE") {
        APACHE_LICENSE_20
            .replace("[yyyy]", year)
            .replace("[name of copyright owner]", author_name)
    } else if name.contains("ISC") {
        ISC_LICENSE
            .replace("[year]", year)
            .replace("[fullname]", author_name)
    } else if name.contains("MOZILLA") {
        String::from(MOZILLA_PUBLIC_LICENSE_20)
    } else if name.contains("MIT") {
        MIT.replace("[year]", year)
            .replace("[fullname]", author_name)
    } else if name.contains("LGPLv3") {
        String::from(GNU_LESSER_GENERAL_PUBLIC_LICENSE_V30)
    } else if name.contains("GPLV3") {
        String::from(GNU_GPL_V3)
    } else {
        String::new()
    };
}

const APACHE_LICENSE_20: &str = include_str!("apache2.txt");
const GNU_GPL_V3: &str = include_str!("gplv3.txt");
const MIT: &str = include_str!("mit.txt");
const ISC_LICENSE: &str = include_str!("isc.txt");
const GNU_LESSER_GENERAL_PUBLIC_LICENSE_V30: &str = include_str!("lgplv3.txt");
const MOZILLA_PUBLIC_LICENSE_20: &str = include_str!("mozilla2.txt");

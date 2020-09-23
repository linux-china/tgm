use regex::Regex;

#[test]
fn test_regex() {
    let pattern = r"[\w-\.]+@([\w-]+\.)+[\w-]{2,4}";
    let regex = Regex::new(&pattern).unwrap();
    let matched = regex.is_match("libing.chen@gmail.com");
    println!("{}", matched);
}

pub mod parser;
pub mod semantics;

pub fn remove_comment(input: &str) -> String {
    let re = regex::Regex::new(r"//.*\n").unwrap();
    re.replace_all(&input, "").to_string()
}

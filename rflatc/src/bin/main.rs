//! FlatBuffers compiler

use combine::Parser;
use rflatc::{parser::fbs, semantics::Buffer};

use std::io::Read;

fn main() {
    let mut input = String::new();
    let size = std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    if size == 0 {
        panic!("Input is empty");
    }

    // Remove comment
    let re = regex::Regex::new(r"//.*\n").unwrap();
    let input = re.replace_all(&input, "").to_string();

    let (stmt, res) = fbs().parse(input.as_str()).expect("Failed to parse");
    assert_eq!(res, "");

    println!("{:?}", stmt);

    let buffer = Buffer::new(stmt);
    println!("{:?}", buffer);
}

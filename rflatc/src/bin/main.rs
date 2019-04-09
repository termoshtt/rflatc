//! FlatBuffers compiler

use combine::Parser;
use rflatc::{parser::fbs, remove_comment, semantics::Buffer};

use std::io::Read;

fn main() {
    let mut input = String::new();
    let size = std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    if size == 0 {
        panic!("Input is empty");
    }

    let input = remove_comment(&input);

    let (stmt, res) = fbs().parse(input.as_str()).expect("Failed to parse");
    assert_eq!(res, "");

    println!("{:?}", stmt);

    let buffer = Buffer::new(stmt);
    println!("{:?}", buffer);
}

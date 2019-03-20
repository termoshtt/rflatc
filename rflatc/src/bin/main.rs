//! FlatBuffers compiler

use combine::Parser;
use rflatc::parser::fbs;

use std::io::Read;

fn main() {
    let mut input = String::new();
    let size = std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    if size == 0 {
        panic!("Input is empty");
    }

    let result = fbs().parse(input.as_str()).expect("Failed to parse");
    assert_eq!(result.1, "");

    println!("{:?}", result.0);
}

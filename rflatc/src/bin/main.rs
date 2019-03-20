//! FlatBuffers compiler

use combine::Parser;
use rflatc::parser::fbs;

fn main() {
    let result = fbs().parse(
        r#"
        namespace test_fbs;

        table A {
            a: Int32;
            b: Int32;
        }

        root_type A;
        "#,
    );
    println!("{:?}", result);
}

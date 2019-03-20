//! FlatBuffers compiler

fn main() {
    let result = identifier().parse("vim");
    println!("{:?}", result);
    let result = identifier().parse("emacs_vim");
    println!("{:?}", result);

    let result = ty().parse("bool");
    println!("{:?}", result);
    let result = ty().parse("long");
    println!("{:?}", result);

    let result = namespace().parse("namespace mad.magi;");
    println!("{:?}", result);

    let result = field().parse("a : uint32;");
    println!("{:?}", result);

    let result = field().parse("a : uint32;");
    println!("{:?}", result);

    let result = root().parse("root_type vim;");
    println!("{:?}", result);

    let result = table().parse(
        r#"table A {
        a: Int32;
        b: Int32;
        }"#,
    );
    println!("{:?}", result);

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

//! FlatBuffers compiler
//!
//! - [Grammar of the schema language](https://google.github.io/flatbuffers/flatbuffers_grammar.html)

use combine::{char::*, parser::Parser, *};

type Identifier = String;
type Scalar = Option<String>;
type Metadata = Option<Vec<String>>;

/// ident = [a-zA-Z_][a-zA-Z0-9_]*
fn identifier<I>() -> impl Parser<Input = I, Output = Identifier>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    letter()
        .and(many::<Vec<char>, _>(alpha_num().or(char('_'))))
        .map(|(l, a)| format!("{}{}", l, a.iter().collect::<String>()))
}

/// Use obviously sized type names
#[derive(Clone, Debug)]
enum Type {
    Bool,
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Float32,
    Float64,
    String_,
    UserDefined(Identifier),
}

/// type = bool | byte | ubyte | short | ushort | int | uint | float | long | ulong | double | int8
/// | uint8 | int16 | uint16 | int32 | uint32| int64 | uint64 | float32 | float64 | string |
/// [ type ] | ident
fn ty<I>() -> impl Parser<Input = I, Output = Type>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    identifier().map(|id| match id.as_ref() {
        "bool" => Type::Bool,
        "byte" => Type::Int8,
        "ubyte" => Type::UInt8,
        "short" => Type::Int16,
        "ushort" => Type::UInt16,
        "int" => Type::Int32,
        "uint" => Type::UInt32,
        "float" => Type::Float32,
        "long" => Type::Int64,
        "ulong" => Type::UInt64,
        "double" => Type::Float64,
        "int8" => Type::Int8,
        "uint8" => Type::UInt8,
        "int16" => Type::Int16,
        "uint16" => Type::UInt16,
        "int32" => Type::Int32,
        "uint32" => Type::UInt32,
        "int64" => Type::Int64,
        "uint64" => Type::UInt64,
        "float32" => Type::Float32,
        "float64" => Type::Float64,
        "string" => Type::String_,
        _ => Type::UserDefined(id.into()),
    })
}

/// metadata = [ ( commasep( ident [ : single_value ] ) ) ]
fn metadata<I>() -> impl Parser<Input = I, Output = Metadata>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    // FIXME
    value(None)
}

#[derive(Debug)]
struct Field {
    id: Identifier,
    ty: Type,
    scalar: Scalar,
    metadata: Metadata,
}

/// field_decl = ident : type [ = scalar ] metadata ;
fn field<I>() -> impl Parser<Input = I, Output = Field>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    identifier()
        .skip(spaces())
        .skip(token(':'))
        .skip(spaces())
        .and(ty())
        .skip(spaces())
        .and(metadata())
        .skip(spaces())
        .skip(token(';'))
        .skip(spaces())
        .map(|((id, ty), metadata)| Field {
            id,
            ty,
            scalar: None,
            metadata,
        })
}

#[derive(Debug)]
enum Stmt {
    Namespace(Vec<Identifier>),
    Root(Identifier),
    Table(Vec<Field>),
}

/// namespace_decl = namespace ident ( . ident )* ;
fn namespace<I>() -> impl Parser<Input = I, Output = Stmt>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    string("namespace")
        .skip(spaces())
        .and(sep_by1::<Vec<Identifier>, _, _>(identifier(), token('.')))
        .skip(spaces())
        .skip(token(';'))
        .skip(spaces())
        .map(|(_, id)| Stmt::Namespace(id))
}

/// root_decl = root_type ident ;
fn root<I>() -> impl Parser<Input = I, Output = Stmt>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    string("root_type")
        .skip(spaces())
        .and(identifier())
        .skip(spaces())
        .skip(token(';'))
        .skip(spaces())
        .map(|(_, id)| Stmt::Root(id))
}

fn paren<I, F>(f: F) -> impl Parser<Input = I, Output = F::Output>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    F: Parser<Input = I>,
{
    between(
        token('{'),
        token('}'),
        spaces().and(f).skip(spaces()).map(|x| x.1),
    )
}

fn table<I>() -> impl Parser<Input = I, Output = Stmt>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    string("table")
        .skip(spaces())
        .and(identifier())
        .skip(spaces())
        .and(paren(many1(field())))
        .skip(spaces())
        .map(|(_, fields)| Stmt::Table(fields))
}

fn fbs<I>() -> impl Parser<Input = I, Output = Vec<Stmt>>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    spaces()
        .and(many1(table().or(namespace()).or(root())))
        .map(|x| x.1)
}

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

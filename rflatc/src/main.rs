//! FlatBuffers compiler
//!
//! - [Grammar of the schema language](https://google.github.io/flatbuffers/flatbuffers_grammar.html)

use combine::{char::*, parser::Parser, *};

type Identifier = String;
type Scalar = ();
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

/// Use exactly sized type names
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
enum Stmt {
    Namespace(Vec<Identifier>),
    Field(Identifier, Type, Option<Scalar>, Metadata),
    Root(Identifier),
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
        .map(|(_, id)| Stmt::Namespace(id))
}

/// field_decl = ident : type [ = scalar ] metadata ;
fn field<I>() -> impl Parser<Input = I, Output = Stmt>
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
        .map(|((id, ty), metadata)| Stmt::Field(id, ty, None, metadata))
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
        .map(|(_, id)| Stmt::Root(id))
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

    let result = field().parse("a : uint32_t;");
    println!("{:?}", result);

    let result = root().parse("root_type vim;");
    println!("{:?}", result);
}

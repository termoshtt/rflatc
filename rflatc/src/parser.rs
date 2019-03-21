//! FlatBuffers Parser
//!
//! - [Grammar of the schema language](https://google.github.io/flatbuffers/flatbuffers_grammar.html)

use combine::{char::*, parser::Parser, *};

pub type Identifier = String;
pub type Scalar = Option<String>;
pub type Metadata = Option<Vec<String>>;

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
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
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

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub id: Identifier,
    pub ty: Type,
    pub scalar: Scalar,
    pub metadata: Metadata,
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

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub id: Identifier,
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Namespace(Vec<Identifier>),
    Root(Identifier),
    Table(Table),
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
        .map(|((_, id), fields)| Stmt::Table(Table { id, fields }))
}

/// Entry point of schema language
pub fn fbs<I>() -> impl Parser<Input = I, Output = Vec<Stmt>>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    spaces() // Drop head spaces
        .and(many(table().or(namespace()).or(root())))
        .map(|x| x.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier() {
        assert_eq!(
            identifier().parse("id_id_").unwrap(),
            ("id_id_".to_string(), "")
        );
    }

    #[test]
    fn test_type() {
        assert_eq!(ty().parse("bool").unwrap(), (Type::Bool, ""));
        assert_eq!(ty().parse("long").unwrap(), (Type::Int64, ""));
    }

    #[test]
    fn test_field() {
        assert_eq!(
            field().parse("a : uint32;").unwrap(),
            (
                Field {
                    id: "a".into(),
                    ty: Type::UInt32,
                    scalar: None,
                    metadata: None
                },
                ""
            )
        );
    }

    #[test]
    fn test_namespace() {
        assert_eq!(
            namespace().parse("namespace mad.magi;").unwrap(),
            (Stmt::Namespace(vec!["mad".into(), "magi".into()]), "")
        );
    }

    #[test]
    fn test_root() {
        assert_eq!(
            root().parse("root_type A;").unwrap(),
            (Stmt::Root("A".into()), "")
        );
    }

    #[test]
    fn test_table() {
        assert_eq!(
            table()
                .parse(
                    r#"table A {
                        a: int32;
                        b: int32;
                    }"#,
                )
                .unwrap(),
            (
                Stmt::Table(vec![
                    Field {
                        id: "a".into(),
                        ty: Type::Int32,
                        scalar: None,
                        metadata: None
                    },
                    Field {
                        id: "b".into(),
                        ty: Type::Int32,
                        scalar: None,
                        metadata: None
                    }
                ]),
                ""
            )
        );
    }
}

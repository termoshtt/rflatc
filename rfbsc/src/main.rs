//! FlatBuffers compiler
//!
//! - [Grammar of the schema language](https://google.github.io/flatbuffers/flatbuffers_grammar.html)

use combine::{char::*, parser::Parser, *};

type Identifier = String;
type Type = String;
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
        .and(identifier())
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
    let result = namespace().parse("namespace mad.magi;");
    println!("{:?}", result);
    let result = field().parse("a : uint32_t;");
    println!("{:?}", result);
    let result = root().parse("root_type vim;");
    println!("{:?}", result);
}

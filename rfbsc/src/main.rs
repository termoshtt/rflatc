//! FlatBuffers compiler

use combine::{char::*, parser::Parser, *};

#[derive(Debug)]
struct Identifier(String);

fn identifier<I>() -> impl Parser<Input = I, Output = Identifier>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    letter()
        .and(many::<Vec<char>, _>(alpha_num().or(char('_'))))
        .map(|(l, a)| Identifier(format!("{}{}", l, a.iter().collect::<String>())))
}

#[derive(Debug)]
enum Stmt {
    Namespace(Identifier),
}

fn namespace<I>() -> impl Parser<Input = I, Output = Stmt>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    string("namespace")
        .and(spaces())
        .and(identifier())
        .and(spaces())
        .and(char(';'))
        .map(|(((_, id), _), _)| Stmt::Namespace(id))
}

fn main() {
    let result = identifier().parse("vim");
    println!("{:?}", result);
    let result = identifier().parse("emacs_vim");
    println!("{:?}", result);
    let result = namespace().parse("namespace Vim;");
    println!("{:?}", result);
}

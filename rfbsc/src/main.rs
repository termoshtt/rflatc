//! FlatBuffers compiler

use combine::{char::*, parser::Parser, *};
use combine_language::*;

fn flatc_env<I>() -> LanguageEnv<'static, I>
where
    I: 'static + combine::stream::Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    LanguageEnv::new(LanguageDef {
        ident: Identifier {
            start: letter(),
            rest: alpha_num(),
            reserved: [
                "include",
                "namespace",
                "attribute",
                "table",
                "struct",
                "enum",
                "union",
                "root_type",
                "rpc_service",
                "file_extension",
                "file_identifier",
            ]
            .iter()
            .map(|x| (*x).into())
            .collect(),
        },
        op: Identifier {
            start: satisfy(|c| ":=".chars().any(|x| x == c)),
            rest: satisfy(|c| ":=".chars().any(|x| x == c)),
            reserved: Vec::new(),
        },
        comment_start: string("/*").map(|_| ()),
        comment_end: string("*/").map(|_| ()),
        comment_line: string("//").map(|_| ()),
    })
}

#[derive(Debug)]
enum Stmt {
    Namespace(String),
}

fn main() {
    let env = flatc_env();
    let mut ns = string("namespace")
        .and(spaces())
        .and(env.identifier())
        .and(char(';'))
        .map(|((_, id), _)| Stmt::Namespace(id));
    let result = ns.easy_parse("namespace Vim;");
    println!("{:?}", result);
}

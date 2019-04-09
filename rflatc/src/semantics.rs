//! Analyze semantics of input statements

use crate::parser::*;
use failure::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Entry {
    pub path: Vec<Identifier>,
    pub ty: Type,
}

#[derive(Debug)]
pub struct Buffer {
    pub root: Vec<Entry>,
    pub namespace: Vec<Identifier>,
}

fn seek_namespace(stmt: &[Stmt]) -> Fallible<Vec<Identifier>> {
    let ns: Vec<_> = stmt
        .iter()
        .filter_map(|st| match st {
            Stmt::Namespace(ns) => Some(ns),
            _ => None,
        })
        .collect();
    match ns.len() {
        0 => bail!("No namespaces are found"),
        1 => Ok(ns[0].clone()),
        _ => bail!("Duplicated namespaces: {:?}", ns),
    }
}

fn seek_root_type(stmt: &[Stmt]) -> Fallible<Identifier> {
    let root: Vec<_> = stmt
        .iter()
        .filter_map(|st| match st {
            Stmt::Root(id) => Some(id),
            _ => None,
        })
        .collect();
    match root.len() {
        0 => bail!("No root_type are found"),
        1 => Ok(root[0].clone()),
        _ => bail!("Duplicated root_type: {:?}", root),
    }
}

fn seek_tables(stmt: &[Stmt]) -> HashMap<Identifier, Vec<Entry>> {
    stmt.iter()
        .filter_map(|st| match st {
            Stmt::Table(table) => Some((
                table.id.clone(),
                table
                    .fields
                    .iter()
                    .map(|e| Entry {
                        path: vec![e.id.clone()],
                        ty: e.ty.clone(),
                    })
                    .collect(),
            )),
            _ => None,
        })
        .collect()
}

impl Buffer {
    pub fn new(stmt: Vec<Stmt>) -> Fallible<Self> {
        let root_type = seek_root_type(&stmt)?;
        let namespace = seek_namespace(&stmt)?;
        let mut tables = seek_tables(&stmt);
        let root = tables
            .remove(&root_type)
            .ok_or(format_err!("Cannot find table: {}", root_type))?;

        Ok(Buffer { root, namespace })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parser::fbs, remove_comment};
    use combine::Parser;
    use std::{fs, io::Read};

    fn read_example_fbs() -> Vec<Stmt> {
        let mut f =
            fs::File::open("example.fbs").expect("Cannot open example.fbs for semantics testing");
        let mut input = String::new();
        f.read_to_string(&mut input)
            .expect("Failed to load example.fbs");
        let input = remove_comment(&input);
        let (stmt, res) = fbs()
            .parse(input.as_str())
            .expect("Failed to parse example.fbs");
        assert_eq!(res, "");
        stmt
    }

    #[test]
    fn test_namespace() {
        let stmt = read_example_fbs();
        let ns = seek_namespace(&stmt).expect("Namespace cannot find");
        assert_eq!(vec!["example.fbs"], ns);
    }

    #[test]
    fn test_root_type() {
        let stmt = read_example_fbs();
        let root_type = seek_root_type(&stmt).expect("root_type cannot find");
        assert_eq!("A".to_string(), root_type);
    }
}

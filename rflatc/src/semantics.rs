//! Analyze semantics of input statements

use crate::parser::*;
use failure::*;
use std::collections::HashMap;

/// Each entry of `table`
#[derive(Debug)]
pub struct Entry {
    pub path: Vec<Identifier>,
    pub ty: Type,
}

/// Entire FlatBuffers definition
#[derive(Debug)]
pub struct Buffer {
    pub root: Vec<Entry>,
    pub namespace: Vec<Identifier>,
    pub file_identifier: Identifier,
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

fn seek_file_identifier(stmt: &[Stmt]) -> Fallible<Identifier> {
    let fid: Vec<_> = stmt
        .iter()
        .filter_map(|st| match st {
            Stmt::FileIdentifier(id) => Some(id),
            _ => None,
        })
        .collect();
    match fid.len() {
        0 => bail!("No file_identifier are found"),
        1 => Ok(fid[0].clone()),
        _ => bail!("Duplicated file_identifier: {:?}", fid),
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
        let file_identifier = seek_file_identifier(&stmt)?;
        let namespace = seek_namespace(&stmt)?;
        let mut tables = seek_tables(&stmt);
        let root = tables
            .remove(&root_type)
            .ok_or(format_err!("Cannot find table: {}", root_type))?;

        Ok(Buffer {
            root,
            file_identifier,
            namespace,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parser::fbs, remove_comment};
    use combine::Parser;
    use std::{fs, io::Read};

    // Read example.fbs from flatcc example (see its header)
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
        assert_eq!(vec!["Eclectic"], ns);
    }

    #[test]
    fn test_root_type() {
        let stmt = read_example_fbs();
        let root_type = seek_root_type(&stmt).expect("root_type cannot find");
        assert_eq!("FooBar".to_string(), root_type);
    }
}

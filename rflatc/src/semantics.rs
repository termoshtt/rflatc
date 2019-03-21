//! Analyze semantics of input statements

use crate::parser::*;
use failure::*;
use std::collections::HashMap;

pub struct Entry {
    pub path: Vec<Identifier>,
    pub ty: Type,
}

pub struct Buffer {
    pub root: Vec<Entry>,
    pub namespace: Vec<Identifier>,
}

fn seek_namespace(stmt: &[Stmt]) -> Fallible<Vec<Identifier>> {
    let mut namespace = None;
    for st in stmt {
        match st {
            Stmt::Namespace(ns) => {
                if namespace.is_some() {
                    bail!("duplicated namespace: {:?}, {:?}", namespace.unwrap(), ns);
                }
                namespace = Some(ns);
            }
            _ => continue,
        }
    }
    match namespace {
        Some(ns) => Ok(ns.clone()),
        None => bail!("No namespaces are found"),
    }
}

fn seek_root_type(stmt: &[Stmt]) -> Fallible<Identifier> {
    let mut root = None;
    for st in stmt {
        match st {
            Stmt::Root(id) => {
                if root.is_some() {
                    bail!("duplicated root_type: {:?}, {:?}", root.unwrap(), id);
                }
                root = Some(id);
            }
            _ => continue,
        }
    }
    match root {
        Some(id) => Ok(id.clone()),
        None => bail!("No root_type are found"),
    }
}

fn seek_tables(stmt: &[Stmt]) -> HashMap<Identifier, Vec<Entry>> {
    stmt.iter()
        .flat_map(|st| match st {
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

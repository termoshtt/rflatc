//! Analyze semantics of input statements

use crate::parser::*;

pub struct FBS {
    root: Vec<Field>,
    Namespace: Vec<Identifier>,
}

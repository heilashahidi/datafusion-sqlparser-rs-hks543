// src/ast/cypher.rs

use crate::ast::{Expr, Ident};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CypherStatement {
    pub match_clause: CypherMatch,
    pub where_clause: Option<Expr>,
    pub return_clause: CypherReturn,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CypherMatch {
    pub pattern: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CypherReturn {
    pub items: Vec<Ident>,
}

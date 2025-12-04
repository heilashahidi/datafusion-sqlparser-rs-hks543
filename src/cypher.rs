use crate::ast::{Statement, Query, SetExpr, Select, SelectItem, TableWithJoins, TableFactor, TableAlias, Ident};

use crate::parser::ParserError;

/// Public function you will call from your transformer.
/// For now, it only supports exactly "MATCH (n) RETURN n".
pub fn parse_cypher_to_sql(query: &str) -> Result<Statement, ParserError> {
    let trimmed = query.trim();

    if trimmed.eq_ignore_ascii_case("MATCH (n) RETURN n") {
        // Turn that Cypher into: SELECT * FROM nodes AS n;
        Ok(make_select_all_nodes())
    } else {
        Err(ParserError::ParserError(format!(
            "Unsupported Cypher query: {trimmed}"
        )))
    }
}

fn make_select_all_nodes() -> Statement {
    // SELECT * FROM nodes AS n;
    let select = Select {
        distinct: false,
        top: None,
        projection: vec![SelectItem::Wildcard],
        into: None,
        from: vec![TableWithJoins {
            relation: TableFactor::Table {
                name: "nodes".into(),
                alias: Some(TableAlias {
                    name: Ident::new("n"),
                    columns: vec![],
                }),
                args: vec![],
                with_hints: vec![],
            },
            joins: vec![],
        }],
        lateral_views: vec![],
        selection: None,
        group_by: vec![],
        cluster_by: vec![],
        distribute_by: vec![],
        sort_by: vec![],
        having: None,
        named_window: vec![],
        qualify: None,
    };

    Statement::Query(Box::new(Query {
        with: None,
        body: SetExpr::Select(Box::new(select)),
        order_by: vec![],
        limit: None,
        offset: None,
        fetch: None,
        locks: vec![],
    }))
}

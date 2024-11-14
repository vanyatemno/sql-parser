#[cfg(test)]
mod tests {
    use sql_parser_project::{Condition, Query, QueryParser};

    #[test]
    fn test_parse_select_basic() {
        let query = QueryParser::parse_query("SELECT \"column\" FROM \"table\";").unwrap();
        assert_eq!(
            query,
            Query {
                table: "\"table\"".to_string(),
                columns: vec!["\"column\"".to_string()],
                conditions: Vec::new(),
            }
        );
    }

    #[test]
    fn test_parse_select_with_where() {
        let query =
            QueryParser::parse_query("SELECT \"column\" FROM \"table\" WHERE \"column\" = 5;")
                .unwrap();
        assert_eq!(
            query,
            Query {
                table: "\"table\"".to_string(),
                columns: vec!["\"column\"".to_string()],
                conditions: vec![Condition {
                    column: "\"column\"".to_string(),
                    operator: "=".to_string(),
                    value: "5".to_string(),
                    logical: None,
                }],
            }
        );
    }

    #[test]
    fn test_parse_select_with_multiple_conditions() {
        let query = QueryParser::parse_query(
            "SELECT \"column1\" FROM \"table\" WHERE \"column1\" = 5 AND \"column2\" != 'value';",
        )
        .unwrap();
        assert_eq!(
            query,
            Query {
                table: "\"table\"".to_string(),
                columns: vec!["\"column1\"".to_string()],
                conditions: vec![
                    Condition {
                        column: "\"column1\"".to_string(),
                        operator: "=".to_string(),
                        value: "5".to_string(),
                        logical: None,
                    },
                    Condition {
                        column: "".to_string(),
                        operator: "".to_string(),
                        value: "".to_string(),
                        logical: Some("AND".to_string()),
                    },
                    Condition {
                        column: "\"column2\"".to_string(),
                        operator: "!=".to_string(),
                        value: "'value'".to_string(),
                        logical: None,
                    },
                ],
            }
        );
    }
}

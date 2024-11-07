use sql_parser_project::{parse_query, Query};

#[test]
fn test_parse_simple_select() {
    let query_str = "SELECT column1, column2 FROM my_table";
    let expected = Query {
        table: "my_table".to_string(),
        columns: vec!["column1".to_string(), "column2".to_string()],
    };

    let parsed = parse_query(query_str).expect("Failed to parse query");
    assert_eq!(parsed, expected);
}

#[test]
fn test_parse_single_column_select() {
    let query_str = "SELECT \"column1\" FROM my_table";
    let expected = Query {
        table: "my_table".to_string(),
        columns: vec!["\"column1\"".to_string()],
    };

    let parsed = parse_query(query_str).expect("Failed to parse query");
    assert_eq!(parsed, expected);
}

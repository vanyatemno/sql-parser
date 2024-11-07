# SQL Query Parser

This is a simple project to parse SQL queries using `pest` crate.\
Currently parser supports simple `select` queries.

## Project Description

The `SQL Query Parser` extracts from a basic SQL query:
- **Table Name**: The table from which data is selected.
- **Columns**: List of columns specified in the query.

## Parsing Process

The parser uses the following grammar to interpret SQL syntax:
- `select_stmt`: Matches the full SQL `SELECT` statement.
- `columns`: Parses a comma-separated list of columns.
- `table`: Parses the table name.
- `identifier`: Parses the name of a column or table.

## Example Usage

```rust
use sql_parser_project::parse_query;

fn main() {
    let query_str = "SELECT column1, column2 FROM my_table";
    let parsed_query = parse_query(query_str).unwrap();

    assert_eq!(parsed_query.table, "my_table");
    assert_eq!(parsed_query.columns, vec!["column1", "column2"]);
}
```

## Future improvements
- `WHERE` statement (operators >, <, =, &&, IN etc.)
- `LEFT JOIN` support
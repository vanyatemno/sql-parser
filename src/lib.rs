use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
struct SQLParser;

#[derive(Debug, PartialEq)]
pub struct Query {
    pub table: String,
    pub columns: Vec<String>,
}

pub fn parse_query(query: &str) -> Result<Query, pest::error::Error<Rule>> {
    let mut parsed = SQLParser::parse(Rule::select_stmt, query)?;
    let mut table = String::new();
    let mut columns = Vec::new();

    for record in parsed.next().unwrap().into_inner() {
        match record.as_rule() {
            Rule::table => table = record.as_str().to_string(),
            Rule::columns => {
                for column in record.into_inner() {
                    columns.push(column.as_str().to_string());
                }
            }
            _ => {}
        }
    }

    Ok(Query { table, columns })
}

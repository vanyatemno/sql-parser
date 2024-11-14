use pest::Parser;
use pest_derive::Parser;
use std::fmt;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct QueryParser;

#[derive(Debug, Error)]
pub enum QueryParserError {
    #[error("An error arose during parsing: {0}")]
    ParserError(String),
}

#[derive(Debug, PartialEq)]
pub struct Query {
    pub table: String,
    pub columns: Vec<String>,
    pub conditions: Vec<Condition>,
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Formatting the selected table
        writeln!(f, "Selected table: {}", self.table)?;

        // Formatting the selected columns
        writeln!(f, "Selected columns: {}", self.columns.join(", "))?;

        // Formatting the conditions
        writeln!(f, "Conditions:")?;
        if self.conditions.is_empty() {
            writeln!(f, "  None")?;
        } else {
            for condition in &self.conditions {
                writeln!(f, "  {}", condition)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Condition {
    pub column: String,
    pub operator: String,
    pub value: String,
    pub logical: Option<String>,
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Displaying the condition in a human-readable format
        let logical = if let Some(logical_op) = &self.logical {
            format!(" {} ", logical_op)
        } else {
            String::new()
        };

        if !logical.is_empty() {
            write!(f, "{}", logical)
        } else {
            write!(
                f,
                "Column: {}, Operator: {}, Value: {}{}",
                self.column, self.operator, self.value, logical
            )
        }
    }
}

impl QueryParser {
    pub fn parse_query(input: &str) -> Result<Query, QueryParserError> {
        let mut parsed = QueryParser::parse(Rule::query, input)
            .map_err(|e| QueryParserError::ParserError(e.to_string()))?;

        let mut table = String::new();
        let mut columns = Vec::new();
        let mut conditions = Vec::new();

        for pair in parsed.next().unwrap().into_inner() {
            match pair.as_rule() {
                Rule::columns => {
                    for column in pair.into_inner() {
                        columns.push(column.as_str().to_string());
                    }
                }

                Rule::from_clause => {
                    table = pair.into_inner().next().unwrap().as_str().to_string();
                }
                Rule::where_clause => {
                    for pair in pair.into_inner() {
                        let mut operator = String::new();
                        let mut value = String::new();
                        let mut column = String::new();
                        let mut logical: Option<String> = None;
                        match pair.as_rule() {
                            Rule::where_condition => {
                                for pair in pair.into_inner() {
                                    match pair.as_rule() {
                                        Rule::operator => {
                                            operator = pair.as_str().to_string();
                                        }
                                        Rule::identifier => {
                                            column = pair.as_str().to_string();
                                        }
                                        Rule::number => {
                                            value = pair.as_str().to_string();
                                        }
                                        Rule::string => {
                                            value = pair.as_str().to_string();
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            Rule::logical_exp => {
                                logical = Some(pair.as_str().to_string());
                            }
                            _ => {}
                        }
                        conditions.push(Condition {
                            operator,
                            column,
                            value,
                            logical,
                        });
                    }
                }
                _ => {}
            }
        }

        Ok(Query {
            columns,
            table,
            conditions,
        })
    }
}

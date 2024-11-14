use sql_parser_project::QueryParser;
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "--help" => print_help(),
        input_file => {
            let file_content =
                fs::read_to_string(input_file).expect("Can't open file with provided file path.");
            match QueryParser::parse_query(&file_content) {
                Ok(parsed_data) => println!("{}", parsed_data),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }

    Ok(())
}

fn print_help() {
    println!("SQL Query Parser help:\n  cargo run <file-path> -> parses given information to Query structure and prints to console\
    \n  cargo run --help -> displays available commands")
}

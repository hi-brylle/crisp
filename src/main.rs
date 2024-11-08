use std::fs::*;
use std::env::*;
use parser::build_program_ast;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

mod ast;
mod parser;

fn main() {
    let src = read_to_string(args().nth(1).unwrap()).unwrap();
    
    let parse_result = GrammarParser::parse(Rule::Program, &src);

    match parse_result {
        Ok(mut pairs) => {
            let root = pairs.next().unwrap();
            println!("{:?}", build_program_ast(root));
        },
        Err(e) => {
            eprintln!("Parsing error: {:?}", e);
        },
    }
}
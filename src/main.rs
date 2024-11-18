use std::fs::*;
use std::env::*;
use parser::build_program_ast;
use pest::Parser;
use pest::error::Error;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

mod ast;
mod parser;

fn main() {
    let src = read_to_string(args().nth(1).unwrap()).unwrap();

    match parse_source(src) {
        Ok(program_ast) => {
            println!("\nParse success AST:\n{:?}\n", program_ast)
        },
        Err(e) => { 
            eprintln!("Parse error: {:?}", e);
        },
    }
}

fn parse_source(source: String) -> Result<ast::Program, Error<Rule>> {
    let parse_result = GrammarParser::parse(Rule::Program, &source);

    match parse_result {
        Ok(mut pairs) => {
            let root = pairs.next().unwrap();
            Ok(build_program_ast(root))
        },
        Err(e) => { Err(e) },
    }
}
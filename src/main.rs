use std::fs::*;
use std::env::*;

use pest::Parser;
use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

use ast::Program;
use parser::build_program_ast;

mod ast;
mod parser;

fn main() {
    let src = read_to_string(args().nth(1).unwrap()).unwrap();

    let frontend_result = 
        parse_source(src);

    match frontend_result {
        Ok(result) => {
            println!("Parse and semantic analysis success!");
            println!("{:#?}", result);
        },
        Err(errors) => {
            for error in errors {
                println!("Semantic analysis error: {}", error);
            }
        },
    }
}

fn parse_source(source: String) -> Result<Program, Vec<String>> {
    let parse_result = GrammarParser::parse(Rule::Program, &source);

    match parse_result {
        Ok(mut pairs) => {
            let root = pairs.next().unwrap();
            Ok(build_program_ast(root))
        },
        Err(e) => {
            Err(vec![format!("Parse error: {:#?}", e)])
        },
    }
}
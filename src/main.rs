use std::fs::*;
use std::env::*;

use parser::build_program_ast;
use pest::Parser;
use pest_derive::Parser;

use ast::Program;
use scope::name_resolution;
use scope::Scope::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

mod ast;
mod parser;
mod scope;

fn main() {
    let src = read_to_string(args().nth(1).unwrap()).unwrap();

    let frontend_result = 
        parse_source(src)
        .and_then(resolve_usages);

    match frontend_result {
        Ok(ast) => {
            println!("Parse and semantic analysis success!");
            println!("{:?}", ast);
        },
        Err(errors) => {
            for error in errors {
                println!("Semantic analysis error: {}", error);
            }
        },
    }
}

fn parse_source(source: String) -> Result<ast::Program, Vec<String>> {
    let parse_result = GrammarParser::parse(Rule::Program, &source);

    match parse_result {
        Ok(mut pairs) => {
            let root = pairs.next().unwrap();
            Ok(build_program_ast(root))
        },
        Err(e) => {
            Err(vec![format!("Parse error: {:?}", e)])
        },
    }
}

fn resolve_usages(program_ast: Program) -> Result<ast::Program, Vec<String>>{
    let resolution_errors = name_resolution(&ProgramScope(&program_ast), &mut vec![]);
    if resolution_errors.is_empty() {
        Ok(program_ast)
    } else {
        Err(resolution_errors)
    }
}
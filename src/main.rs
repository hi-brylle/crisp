use std::fs::*;
use std::env::*;

use pest::Parser;
use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

use ast::Program;
use parser::build_program_ast;
use symbol_table::SymbolTable;
use symbol_table::build_program_symbol_table;

mod ast;
mod parser;
mod scope;
mod name_resolution;
mod symbol_table;

fn main() {
    let src = read_to_string(args().nth(1).unwrap()).unwrap();

    let frontend_result = 
        parse_source(src)
        .and_then(extract_symbol_table)
        .and_then(semantic_analysis);

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

fn extract_symbol_table(program_ast: Program) -> Result<(SymbolTable, Program), Vec<String>> {
    Ok((build_program_symbol_table(&program_ast), program_ast))
}

fn semantic_analysis(table_and_ast: (SymbolTable, Program)) -> Result<(SymbolTable, Program), Vec<String>> {
    let (symbol_table, program_ast) = table_and_ast;
    println!("\nSymbol table: {:#?}", program_ast);
    println!();
    println!("\nSymbol table: {:#?}", symbol_table);

    Err(vec!["Semantic analysis not fully implemented; TO-DO: implement compilation to SECD IR.".to_owned()])
}
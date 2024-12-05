use std::fs::*;
use std::env::*;

use name_resolution::resolve_names;
use pest::Parser;
use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

use ast::Program;
use parser::build_program_ast;
use symbol_table::SymbolTable;
use symbol_table::build_program_symbol_table;
use name_resolution::clean_up_symbol_table;
use name_resolution::get_program_usages;

mod ast;
mod parser;
mod scope;
mod symbol_table;
mod name_resolution;

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
    println!("Symbol table: {:#?}", symbol_table);

    let (valid_symbol_table, redeclarations, against_reserved) = clean_up_symbol_table(&symbol_table);
    println!("Redeclarations: {:#?}", redeclarations);
    println!("Reserved keyword errors: {:#?}", against_reserved);
    println!("Valid symbol table: {:#?}", valid_symbol_table);

    let usages = get_program_usages(&program_ast);
    println!("Usages: {:#?}", usages);
    println!("Resolved names: {:#?}", resolve_names(&usages, &valid_symbol_table));

    Err(vec!["Semantic analysis not fully implemented".to_owned()])
}
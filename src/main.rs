use std::fs::*;
use std::env::*;

use name_resolution::check_redeclarations;
use name_resolution::get_program_usages;
use parser::build_program_ast;
use pest::Parser;
use pest_derive::Parser;

use ast::Program;
use scope::name_resolution;
use scope::Scope::*;
use scope::NameResolutionErrorKind::*;
use symbol_table::build_program_symbol_table;
use symbol_table::SymbolTable;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

mod ast;
mod parser;
mod scope;
mod symbol_table;
mod name_resolution;

fn main() {
    let src = read_to_string(args().nth(1).unwrap()).unwrap();

    // let frontend_result = 
    //     parse_source(src)
    //     .and_then(resolve_names);

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
    println!("Redeclarations: {:#?}", check_redeclarations(&symbol_table));
    println!("Usages: {:#?}", get_program_usages(&program_ast));
    Err(vec!["Semantic analysis not fully implemented".to_owned()])
}

fn resolve_names(program_ast: Program) -> Result<ast::Program, Vec<String>>{
    let resolution_errors = name_resolution(&ProgramScope(&program_ast), &mut vec![]);

    if resolution_errors.is_empty() {
        println!("\nGLOBAL SYMBOL TABLE: {:#?}\n", build_program_symbol_table(&program_ast));
        Ok(program_ast)
    } else {
        Err(resolution_errors
            .into_iter()
            .map(|resolution_error|
                match resolution_error.error_kind {
                    Redeclaration(symbol) => {
                        format!("{:?} \"{}\" redeclared in \"{}\" scope.", symbol.kind, symbol.symbol, resolution_error.scope_name)
                    },
                    Undefined(usage) => {
                        format!("{:?} \"{}\" not defined in \"{}\" scope and beyond.", usage.kind, usage.symbol, resolution_error.scope_name)
                    },
                }
            ).collect())
    }
}
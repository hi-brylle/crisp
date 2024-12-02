use std::collections::HashSet;

use crate::ast::{FunctionDefinition, Program, TypeLiteral, Statement::*};
use crate::scope2::SymbolKind::*;

enum Scope {
    ProgramScope(Program),
    FunctionScope(FunctionDefinition)
}

#[derive(Debug)]
struct Symbol {
    pub symbol: String,
    pub kind: SymbolKind,

    // An empty vector means a variable with no type annotations or it's a usage,
    // A singleton means a variable with an explicit type annotation, and
    // A vector containing at least two type literals mean a function, with the
    // final type literal being the function return type.
    pub type_info: Vec<TypeLiteral>,

    pub start_pos: Option<usize>
}

#[derive(Debug, PartialEq)]
enum SymbolKind {
    Variable,
    Function,
    FunctionParameter
}

#[derive(Debug)]
struct Usage {
    pub symbol: String,
    pub kind: UsageKind,
    pub start_pos: Option<usize>
}

#[derive(Debug)]
enum UsageKind {
    Variable,
    FunctionCall
}

fn get_level_symbol_table(scope: &Scope) -> Vec<Symbol> {
    let mut symbol_table: Vec<Symbol> = vec![];

    match scope {
        Scope::ProgramScope(program) => {
            let statements = &program.statements;
            for s in statements {
                match s {
                    AssignmentStmt(assignment) => {
                        symbol_table.push(Symbol {
                            symbol: assignment.identifier.clone(),
                            kind: Variable,
                            type_info: match &assignment.type_annotation {
                                Some(type_literal) => vec![type_literal.clone()],
                                None => vec![],
                            },
                            start_pos: Some(assignment.start_pos)
                        });
                    },
                    FunctionDefStmt(function_definition_statement) => {
                        let mut type_info: Vec<TypeLiteral> = vec![];
                        for (_, type_literal) in &function_definition_statement.function_parameters {
                            type_info.push(type_literal.clone());                    
                        }
                        type_info.push(function_definition_statement.function_return_type.clone());
        
                        symbol_table.push(Symbol {
                            symbol: function_definition_statement.function_name.clone(),
                            kind: Function,
                            type_info,
                            start_pos: None
                        });
                    },
                }
            }
        },
        Scope::FunctionScope(function_definition) => {
            let parameters = &function_definition.function_parameters;
            for (parameter, type_literal) in parameters {
                symbol_table.push(Symbol {
                    symbol: parameter.clone(),
                    kind: FunctionParameter,
                    type_info: vec![type_literal.clone()],
                    start_pos: None
                });
            }

            let statements = &function_definition.function_body.statements;
            for s in statements {
                match s {
                    AssignmentStmt(assignment) => {
                        symbol_table.push(Symbol {
                            symbol: assignment.identifier.clone(),
                            kind: Variable,
                            type_info: match &assignment.type_annotation {
                                Some(type_literal) => vec![type_literal.clone()],
                                None => vec![],
                            },
                            start_pos: Some(assignment.start_pos)
                        });
                    },
                    FunctionDefStmt(function_definition_statement) => {
                        let mut type_info: Vec<TypeLiteral> = vec![];
                        for (_, type_literal) in &function_definition_statement.function_parameters {
                            type_info.push(type_literal.clone());                    
                        }
                        type_info.push(function_definition_statement.function_return_type.clone());
                        symbol_table.push(Symbol {
                            symbol: function_definition_statement.function_name.clone(),
                            kind: Function,
                            type_info,
                            start_pos: None
                        });
                    },
                }
            }
        },
    }

    symbol_table
}

fn check_for_redeclarations(scope: &Scope) -> Vec<String> {
    let mut errors: Vec<String> = vec![];
    let mut temp: HashSet<String> = HashSet::new();
    let symbol_table = get_level_symbol_table(scope);
    let scope_name = match scope {
        Scope::ProgramScope(_) => String::from("(program)"),
        Scope::FunctionScope(function_definition) => function_definition.function_name.clone(),
    };
    for symbol in symbol_table {
        if !temp.insert(symbol.symbol.clone()) {
            errors.push(format!("{:?} \"{}\" redeclared in \"{}\" scope.", symbol.kind, symbol.symbol, scope_name));
        }
    }
    
    errors
}

fn usage_is_defined(usage: &Usage, symbol_table: &Vec<Symbol>) -> bool {
    println!("\nTesting for usage {:?} with the following symbol table:", usage);

    match usage.kind {
        UsageKind::Variable => {
            symbol_table
            .iter()
            .filter(|s| {s.kind == Variable || s.kind == FunctionParameter})
            .map(|s| {println!("\t{:?}",s);s})
            .any(|s|
                match s.kind {
                    Variable => usage.symbol == s.symbol &&
                        usage.start_pos.unwrap() > s.start_pos.unwrap(), // Make sure Variable is defined before usage.
                    Function => unreachable!("Cannot check Variable usage against function!"),
                    FunctionParameter => usage.symbol == s.symbol, // Make sure FunctionParameter is within scope.
                }
            )
        },
        UsageKind::FunctionCall => {
            symbol_table
            .iter()
            .map(|s| {println!("{:?}",s);s})
            .any(|s| usage.symbol == s.symbol)
        },
    }
}


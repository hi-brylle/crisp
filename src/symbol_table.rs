use std::collections::HashMap;

use crate::ast::{FunctionDefinition, Program, Statement::*};

#[derive(Debug)]
pub struct SymbolTable {
    pub symbol_table: HashMap<String, Symbol>
}

#[derive(Debug)]
struct Symbol {
    pub symbol: String,
    pub kind: SymbolKind,
}

#[derive(Debug)]
enum SymbolKind {
    Variable(usize),
    FunctionDefinition
}

pub fn build_program_symbol_table(program: &Program) -> SymbolTable {
    let mut symbol_table: HashMap<String, Symbol> = HashMap::new();

    for statement in &program.statements {
        match statement {
            AssignmentStmt(assignment) => {
                symbol_table
                    .entry(assignment.scope_address.clone())
                    .or_insert(Symbol {
                        symbol: assignment.identifier.clone(),
                        kind: SymbolKind::Variable(assignment.start_pos),
                    });

            },
            FunctionDefStmt(function_definition) => {
                symbol_table.extend(build_function_def_symbol_table(function_definition));
            },
        }
    }

    SymbolTable {
        symbol_table
    }
}

fn build_function_def_symbol_table(function_definition: &FunctionDefinition) -> Vec<(String, Symbol)> {
    let mut symbol_table: Vec<(String, Symbol)> = vec![];

    symbol_table.push((function_definition.scope_address.clone(),
        Symbol {
            symbol: function_definition.function_name.clone(),
            kind: SymbolKind::FunctionDefinition,
        })
    );

    for statement in &function_definition.function_body.statements {
        match statement {
            AssignmentStmt(assignment) => {
                symbol_table.push((assignment.scope_address.clone(),
                    Symbol {
                        symbol: assignment.identifier.clone(),
                        kind: SymbolKind::Variable(assignment.start_pos),
                    })
                );
            },
            FunctionDefStmt(function_definition) => {
                symbol_table.append(&mut build_function_def_symbol_table(function_definition));
            },
        }
    }

    return symbol_table
}
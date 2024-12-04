use crate::ast::{Assignment, FunctionDefinition, Program, Statement::*};

#[derive(Debug)]
pub struct SymbolTable {
    pub symbol_table: Vec<(String, Symbol)>
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
    let mut symbol_table: Vec<(String, Symbol)> = vec![];

    for statement in &program.statements {
        match statement {
            AssignmentStmt(assignment) => {
                symbol_table.append(&mut build_assignment_symbol_table(assignment));
            },
            FunctionDefStmt(function_definition) => {
                symbol_table.append(&mut build_function_def_symbol_table(function_definition));
            },
        }
    }

    SymbolTable {
        symbol_table
    }
}

fn build_assignment_symbol_table(assignment: &Assignment) -> Vec<(String, Symbol)> {
    vec![(
        assignment.scope_address.clone(),
        Symbol {
            symbol: assignment.identifier.clone(),
            kind: SymbolKind::Variable(assignment.start_pos),
        }
    )]
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
                symbol_table.append(&mut build_assignment_symbol_table(assignment));
            },
            FunctionDefStmt(function_definition) => {
                symbol_table.append(&mut build_function_def_symbol_table(function_definition));
            },
        }
    }

    return symbol_table
}
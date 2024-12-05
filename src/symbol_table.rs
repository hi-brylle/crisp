use crate::ast::{Assignment, FunctionDefinition, Program, Statement::*, TypeLiteral};

#[derive(Debug)]
pub struct SymbolTable {
    pub symbol_table: Vec<Symbol>
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub symbol: String,
    pub scope_address: String,
    pub kind: SymbolKind,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    VariableDeclaration(usize),

    // Size of the type vector is the arity of the function.
    FunctionDefinition(Vec<TypeLiteral>),
    FunctionParameter(TypeLiteral)
}

pub fn build_program_symbol_table(program: &Program) -> SymbolTable {
    let mut symbol_table: Vec<Symbol> = vec![];

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

fn build_assignment_symbol_table(assignment: &Assignment) -> Vec<Symbol> {
    vec![
        Symbol {
            scope_address: assignment.scope_address.clone(),
            symbol: assignment.identifier.clone(),
            kind: SymbolKind::VariableDeclaration(assignment.start_pos),
        }
    ]
}

fn build_function_def_symbol_table(function_definition: &FunctionDefinition) -> Vec<Symbol> {
    let mut symbol_table: Vec<Symbol> = vec![];

    let mut type_vector: Vec<TypeLiteral> = vec![];
    for parameter in &function_definition.function_parameters {
        type_vector.push(parameter.parameter_type.clone());
    }
    type_vector.push(function_definition.function_return_type.clone());
    
    symbol_table.push(Symbol {
        scope_address: function_definition.scope_address.clone(),
        symbol: function_definition.function_name.clone(),
        kind: SymbolKind::FunctionDefinition(type_vector),
    });

    for parameter in &function_definition.function_parameters {
        symbol_table.push(Symbol {
            scope_address: parameter.scope_address.clone(),
            symbol: parameter.parameter_name.clone(),
            kind: SymbolKind::FunctionParameter(parameter.parameter_type.clone()),
        });
    }

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
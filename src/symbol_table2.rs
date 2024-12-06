use std::collections::HashMap;

use crate::ast::{Assignment, FunctionDefinition, Program, Statement::{self, *}, TypeLiteral};

#[derive(Debug)]
pub struct SymbolTable2 {
    pub symbol_table: HashMap<String, SymbolInfo>
}

#[derive(Debug)]
pub struct SymbolInfo {
    pub symbol: String,
    pub scope_address: String,
    pub kind: SymbolKind2,
    pub inner: Option<SymbolTable2>
}

#[derive(Debug)]
pub enum SymbolKind2 {
    VariableDeclaration(Option<TypeLiteral>, usize),    
    FunctionDefinition(Vec<TypeLiteral>), // Size of the type vector is the arity of the function.
    FunctionParameter(TypeLiteral)
}

pub fn build_program_symbol_table2(program: &Program) -> SymbolTable2 {
    let mut symbol_table: HashMap<String, SymbolInfo> = HashMap::new();

    for statement in &program.statements {
        for (name, info) in build_statement_symbol_table2(statement) {
            symbol_table.insert(name, info);
        }
    }

    SymbolTable2 {
        symbol_table,
    }
}

fn build_statement_symbol_table2(statement: &Statement) -> HashMap<String, SymbolInfo> {
    let mut symbol_table: HashMap<String, SymbolInfo> = HashMap::new();

    match statement {
        AssignmentStmt(assignment) => {
            for (name, info) in build_assignment_symbol_table2(assignment) {
                symbol_table.insert(name, info);
            }
        },
        FunctionDefStmt(function_definition) => {
            let mut type_vector: Vec<TypeLiteral> = vec![];
            for parameter in &function_definition.function_parameters {
                type_vector.push(parameter.parameter_type.clone());
            }
            type_vector.push(function_definition.function_return_type.clone());

            symbol_table.insert(
                function_definition.function_name.clone(),
                SymbolInfo {
                    symbol: function_definition.function_name.clone(),
                    scope_address: function_definition.scope_address.clone(),
                    kind: SymbolKind2::FunctionDefinition(type_vector),
                    inner: Some(SymbolTable2 {
                        symbol_table: build_function_def_symbol_table2(function_definition)
                    }),
                }
            );
        },
    }

    symbol_table
}

fn build_assignment_symbol_table2(assignment: &Assignment) -> HashMap<String, SymbolInfo> {
    HashMap::from([(
        assignment.identifier.clone(),
        SymbolInfo {
            symbol: assignment.identifier.clone(),
            scope_address: assignment.scope_address.clone(),
            kind: SymbolKind2::VariableDeclaration(assignment.type_annotation.clone(), assignment.start_pos),
            inner: None,
        }
    )])
}

fn build_function_def_symbol_table2(function_definition: &FunctionDefinition) -> HashMap<String, SymbolInfo> {
    let mut symbol_table: HashMap<String, SymbolInfo> = HashMap::new();

    for parameter in &function_definition.function_parameters {
        symbol_table.insert(
            parameter.parameter_name.clone(),
            SymbolInfo {
                symbol: parameter.parameter_name.clone(),
                scope_address: parameter.scope_address.clone(),
                kind: SymbolKind2::FunctionParameter(parameter.parameter_type.clone()),
                inner: None,
            }
        );
    }

    for statement in &function_definition.function_body.statements {
        for (name, info) in build_statement_symbol_table2(statement) {
            symbol_table.insert(name, info);
        }
    }

    symbol_table
}
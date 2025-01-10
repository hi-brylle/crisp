use std::collections::HashMap;

use crate::ast::{Assignment, FunctionDefinition, Program, Statement::{self, *}, TypeLiteral};

#[derive(Debug)]
pub struct SymbolTable {
    pub symbol_table: HashMap<String, SymbolInfo>
}

#[derive(Debug)]
pub struct SymbolInfo {
    pub scope_address: String,
    pub kind: SymbolKind,
    pub inner: Option<SymbolTable>
}

#[derive(Debug)]
pub enum SymbolKind {
    // Type declarations are optional;
    // usize refers to the location of the variable declaration (useful for shadowing).
    VariableDeclaration(Option<TypeLiteral>, usize),

    // The size of the type vector is the function's arity;
    // items are the parameter types except the final item which is the return type.
    FunctionDefinition(Vec<TypeLiteral>),

    // Type is not optional for function parameters.
    FunctionParameter(TypeLiteral)
}

static RESERVED_KEYWORDS: &[&str] = &[
    "let",
    "fun",
    "return",
    "if",
    "else",
    "true",
    "false",
    "Number",
    "Boolean",
    "String",
    "Unit",
];

pub fn build_program_symbol_table(program: &Program) -> SymbolTable {
    let mut symbol_table: HashMap<String, SymbolInfo> = HashMap::new();

    for statement in &program.statements {
        for (name, info) in build_statement_symbol_table(statement).symbol_table {
            symbol_table.insert(name, info);
        }
    }

    SymbolTable {
        symbol_table,
    }
}

fn build_statement_symbol_table(statement: &Statement) -> SymbolTable {
    let mut symbol_table: HashMap<String, SymbolInfo> = HashMap::new();

    match statement {
        AssignmentStmt(assignment) => {
            for (name, info) in build_assignment_symbol_table(assignment).symbol_table {
                // if RESERVED_KEYWORDS.contains(&name.as_str()) {
                //     eprintln!("Variable declaration \"{}\" is a reserved keyword.", name)
                // }
                symbol_table.insert(name, info);
            }
        },
        FunctionDefStmt(function_definition) => {
            let mut type_vector: Vec<TypeLiteral> = vec![];
            for parameter in &function_definition.function_parameters {
                type_vector.push(parameter.parameter_type.clone());
            }
            type_vector.push(function_definition.function_return_type.clone());

            if RESERVED_KEYWORDS.contains(&function_definition.function_name.as_str()) {
                eprintln!("Function \"{}\" is a reserved keyword.", function_definition.function_name)
            }
            symbol_table.insert(
                function_definition.function_name.clone(),
                SymbolInfo {
                    scope_address: function_definition.scope_address.clone(),
                    kind: SymbolKind::FunctionDefinition(type_vector),
                    inner: Some(SymbolTable {
                        symbol_table: build_function_def_symbol_table(function_definition).symbol_table
                    }),
                }
            );
        },
    }

    SymbolTable {
        symbol_table,
    }
}

fn build_assignment_symbol_table(assignment: &Assignment) -> SymbolTable {
    SymbolTable {
        symbol_table: HashMap::from([(
            assignment.identifier.clone(),
            SymbolInfo {
                scope_address: assignment.scope_address.clone(),
                kind: SymbolKind::VariableDeclaration(assignment.type_annotation.clone(), assignment.start_pos),
                inner: None,
            }
        )])
    }
}

fn build_function_def_symbol_table(function_definition: &FunctionDefinition) -> SymbolTable {
    let mut symbol_table: HashMap<String, SymbolInfo> = HashMap::new();

    for parameter in &function_definition.function_parameters {
        if RESERVED_KEYWORDS.contains(&parameter.parameter_name.as_str()) {
            eprintln!("Function parameter \"{}\" is a reserved keyword.", parameter.parameter_name)
        }
        symbol_table.insert(
            parameter.parameter_name.clone(),
            SymbolInfo {
                scope_address: parameter.scope_address.clone(),
                kind: SymbolKind::FunctionParameter(parameter.parameter_type.clone()),
                inner: None,
            }
        );
    }

    for statement in &function_definition.function_body.statements {
        for (name, info) in build_statement_symbol_table(statement).symbol_table {
            symbol_table.insert(name, info);
        }
    }

    SymbolTable {
        symbol_table,
    }
}
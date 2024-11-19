use std::collections::HashMap;

use crate::ast::{FunctionDefinitionStatement, Program, Statement::*};

pub struct Scope {
    pub symbol_table: Vec<String>,
    pub children_scopes: Vec<Scope>
}

pub fn build_program_scope(ast_node: &Program) -> Scope {

    let mut symbol_table: Vec<String> = vec![];
    let mut children_scopes: Vec<Scope> = vec![];
    
    let statements = &ast_node.statements;
    for s in statements {
        match s {
            AssignmentStmt(assignment) => {
                symbol_table.push(assignment.identifier.to_owned());
            },
            FunctionDefStmt(function_definition_statement) => {
                symbol_table.push(function_definition_statement.function_name.to_owned());
                children_scopes.push(build_function_scope(&function_definition_statement));
            },
        }
    }

    Scope {
        symbol_table,
        children_scopes,
    }
}

fn build_function_scope(function_definition_statement: &FunctionDefinitionStatement) -> Scope {

    let mut symbol_table: Vec<String> = vec![];
    let mut children_scopes: Vec<Scope> = vec![];

    let parameters = &function_definition_statement.function_parameters;

    for (parameter, _) in parameters {
        symbol_table.push(parameter.to_owned());
    }

    let statements = &function_definition_statement.function_body.statements;
    for s in statements {
        match s {
            AssignmentStmt(assignment) => {
                symbol_table.push(assignment.identifier.to_owned());
            },
            FunctionDefStmt(function_definition_statement) => {
                symbol_table.push(function_definition_statement.function_name.to_owned());
                children_scopes.push(build_function_scope(&function_definition_statement));
            },
        }
    }

    Scope {
        symbol_table,
        children_scopes,
    }
}
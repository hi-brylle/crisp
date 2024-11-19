use std::collections::HashMap;

use crate::ast::{FunctionDefinitionStatement, Program, Statement::*};

#[derive(Debug)]
pub struct Scope {
    pub scope_name: String,
    pub symbol_table: Vec<Symbol>,
    pub children_scopes: Vec<Scope>
}

#[derive(Debug)]
pub struct Symbol {
    pub symbol: String,
    pub kind: SymbolKind,
}

#[derive(Debug)]
pub enum SymbolKind {
    Variable,
    Function,
    FunctionParameter
}

pub fn build_program_scope(ast_node: &Program) -> Scope {

    let mut symbol_table: Vec<Symbol> = vec![];
    let mut children_scopes: Vec<Scope> = vec![];
    
    let statements = &ast_node.statements;
    for s in statements {
        match s {
            AssignmentStmt(assignment) => {
                symbol_table.push(Symbol {
                    symbol: assignment.identifier.to_owned(),
                    kind: SymbolKind::Variable,
                });
            },
            FunctionDefStmt(function_definition_statement) => {
                symbol_table.push(Symbol {
                    symbol: function_definition_statement.function_name.to_owned(),
                    kind: SymbolKind::Function,
                });
                children_scopes.push(build_function_scope(&function_definition_statement));
            },
        }
    }

    Scope {
        scope_name: "(program)".to_owned(),
        symbol_table,
        children_scopes,
    }
}

fn build_function_scope(function_definition_statement: &FunctionDefinitionStatement) -> Scope {

    let mut symbol_table: Vec<Symbol> = vec![];
    let mut children_scopes: Vec<Scope> = vec![];

    let parameters = &function_definition_statement.function_parameters;

    for (parameter, _) in parameters {
        symbol_table.push(Symbol {
            symbol: parameter.to_owned(),
            kind: SymbolKind::FunctionParameter,
        });
    }

    let statements = &function_definition_statement.function_body.statements;
    for s in statements {
        match s {
            AssignmentStmt(assignment) => {
                symbol_table.push(Symbol {
                    symbol: assignment.identifier.to_owned(),
                    kind: SymbolKind::Variable,
                });
            },
            FunctionDefStmt(function_definition_statement) => {
                symbol_table.push(Symbol {
                    symbol: function_definition_statement.function_name.to_owned(),
                    kind: SymbolKind::Function,
                });
                children_scopes.push(build_function_scope(&function_definition_statement));
            },
        }
    }

    Scope {
        scope_name: function_definition_statement.function_name.to_owned(),
        symbol_table,
        children_scopes,
    }
}
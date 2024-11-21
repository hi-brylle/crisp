use std::collections::HashMap;

use crate::ast::{Expression, FunctionDefinitionStatement, Program, Statement::*};
use SymbolKind::*;

#[derive(Debug)]
pub struct Scope {
    pub scope_name: String,
    pub symbol_table: Vec<Symbol>, // Definitions in a scope available to itself and its children.
    pub usages: Vec<Symbol>, // Symbol usages (variable references, function calls) found in this scope.
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
    let mut usages: Vec<Symbol> = vec![];
    let mut children_scopes: Vec<Scope> = vec![];
    
    let statements = &ast_node.statements;
    for s in statements {
        match s {
            AssignmentStmt(assignment) => {
                symbol_table.push(Symbol {
                    symbol: assignment.identifier.to_owned(),
                    kind: Variable,
                });
                usages.append(&mut extract_symbols(&assignment.rhs));
            },
            FunctionDefStmt(function_definition_statement) => {
                symbol_table.push(Symbol {
                    symbol: function_definition_statement.function_name.to_owned(),
                    kind: Function,
                });
                children_scopes.push(build_function_scope(&function_definition_statement));
            },
        }
    }

    Scope {
        scope_name: "(program)".to_owned(),
        symbol_table,
        usages,
        children_scopes,
    }
}

fn build_function_scope(function_definition_statement: &FunctionDefinitionStatement) -> Scope {

    let mut symbol_table: Vec<Symbol> = vec![];
    let mut usages: Vec<Symbol> = vec![];
    let mut children_scopes: Vec<Scope> = vec![];

    let parameters = &function_definition_statement.function_parameters;

    for (parameter, _) in parameters {
        symbol_table.push(Symbol {
            symbol: parameter.to_owned(),
            kind: FunctionParameter,
        });
    }

    let statements = &function_definition_statement.function_body.statements;
    for s in statements {
        match s {
            AssignmentStmt(assignment) => {
                symbol_table.push(Symbol {
                    symbol: assignment.identifier.to_owned(),
                    kind: Variable,
                });
                usages.append(&mut extract_symbols(&assignment.rhs));
            },
            FunctionDefStmt(function_definition_statement) => {
                symbol_table.push(Symbol {
                    symbol: function_definition_statement.function_name.to_owned(),
                    kind: Function,
                });
                children_scopes.push(build_function_scope(&function_definition_statement));
            },
        }
    }

    Scope {
        scope_name: function_definition_statement.function_name.to_owned(),
        symbol_table,
        usages,
        children_scopes,
    }
}

fn extract_symbols(expression_node: &Expression) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = vec![];

    match expression_node {
        Expression::Identifier(identifier) => {
            symbols.push(Symbol {
                symbol: identifier.to_owned(),
                kind: Variable,
            });
        },
        Expression::Negative(expression) => {
            symbols.append(&mut extract_symbols(&**expression));
        },
        Expression::Plus(expression, expression1) => {
            symbols.append(&mut extract_symbols(&**expression));
            symbols.append(&mut extract_symbols(&**expression1));
        },
        Expression::Minus(expression, expression1) => {
            symbols.append(&mut extract_symbols(&**expression));
            symbols.append(&mut extract_symbols(&**expression1));
        },
        Expression::Times(expression, expression1) => {
            symbols.append(&mut extract_symbols(&**expression));
            symbols.append(&mut extract_symbols(&**expression1));
        },
        Expression::Divide(expression, expression1) => {
            symbols.append(&mut extract_symbols(&**expression));
            symbols.append(&mut extract_symbols(&**expression1));
        },
        Expression::IsEqual(expression, expression1) => {
            symbols.append(&mut extract_symbols(&**expression));
            symbols.append(&mut extract_symbols(&**expression1));
        },
        Expression::NotEqual(expression, expression1) => {
            symbols.append(&mut extract_symbols(&**expression));
            symbols.append(&mut extract_symbols(&**expression1));
        },
        Expression::LessThan(expression, expression1) => {
            symbols.append(&mut extract_symbols(&**expression));
            symbols.append(&mut extract_symbols(&**expression1));
        },
        Expression::LessThanOrEqual(expression, expression1) => {
            symbols.append(&mut extract_symbols(&**expression));
            symbols.append(&mut extract_symbols(&**expression1));
        },
        Expression::GreaterThan(expression, expression1) => {
            symbols.append(&mut extract_symbols(&**expression));
            symbols.append(&mut extract_symbols(&**expression1));
        },
        Expression::GreaterThanOrEqual(expression, expression1) => {
            symbols.append(&mut extract_symbols(&**expression));
            symbols.append(&mut extract_symbols(&**expression1));
        },
        Expression::Not(expression) => {
            symbols.append(&mut extract_symbols(&**expression));
        },
        Expression::Or(expression, expression1) => {
            symbols.append(&mut extract_symbols(&**expression));
            symbols.append(&mut extract_symbols(&**expression1));
        },
        Expression::And(expression, expression1) => {
            symbols.append(&mut extract_symbols(&**expression));
            symbols.append(&mut extract_symbols(&**expression1));
        },
        Expression::IfElseExpression(if_else_expression) => {
            symbols.append(&mut extract_symbols(&*if_else_expression.predicate));
            symbols.append(&mut extract_symbols(&*&if_else_expression.true_branch));
            symbols.append(&mut extract_symbols(&*&if_else_expression.false_branch));
        },
        Expression::FunctionCall(function_call) => {
            symbols.push(Symbol {
                symbol: function_call.function_name.to_owned(),
                kind: Function,
            });
            let function_arguments = &function_call.function_arguments;
            for args in function_arguments {
                symbols.append(&mut extract_symbols(&args));
            }
        },
        _ => {},
    }

    symbols
}
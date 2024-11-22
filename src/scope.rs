use std::collections::{HashMap, HashSet};

use crate::ast::{Expression, FunctionDefinitionStatement, Program, Statement::*};
use SymbolKind::*;

#[derive(Debug)]
pub struct Scope {
    pub scope_name: String,
    pub symbol_table: Vec<Symbol>, // Definitions in a scope available to itself and its children.
    pub usages: Vec<Symbol>, // Symbol usages (variable references, function calls) found in this scope.
    pub inner_scopes: Vec<Scope>
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Symbol {
    pub symbol: String,
    pub kind: SymbolKind,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum SymbolKind {
    Variable,
    Function,
    FunctionParameter
}

pub fn build_program_scope(ast_node: &Program) -> Scope {

    let mut symbol_table: Vec<Symbol> = vec![];
    let mut usages: Vec<Symbol> = vec![];
    let mut inner_scopes: Vec<Scope> = vec![];
    
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
                inner_scopes.push(build_function_scope(&function_definition_statement));
            },
        }
    }

    Scope {
        scope_name: "(program)".to_owned(),
        symbol_table,
        usages,
        inner_scopes,
    }
}

fn build_function_scope(function_definition_statement: &FunctionDefinitionStatement) -> Scope {

    let mut symbol_table: Vec<Symbol> = vec![];
    let mut usages: Vec<Symbol> = vec![];
    let mut inner_scopes: Vec<Scope> = vec![];

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
                inner_scopes.push(build_function_scope(&function_definition_statement));
            },
        }
    }

    Scope {
        scope_name: function_definition_statement.function_name.to_owned(),
        symbol_table,
        usages,
        inner_scopes,
    }
}

fn extract_symbols(expression_node: &Expression) -> Vec<Symbol> {
    let mut symbols: HashSet<Symbol> = HashSet::new();

    match expression_node {
        Expression::Identifier(identifier) => {
            symbols.insert(Symbol {
                symbol: identifier.to_owned(),
                kind: Variable,
            });
        },
        Expression::Negative(expression) => {
            symbols.extend(extract_symbols(&**expression));
        },
        Expression::Plus(expression, expression1) => {
            symbols.extend(extract_symbols(&**expression));
            symbols.extend(extract_symbols(&**expression1));
        },
        Expression::Minus(expression, expression1) => {
            symbols.extend(extract_symbols(&**expression));
            symbols.extend(extract_symbols(&**expression1));
        },
        Expression::Times(expression, expression1) => {
            symbols.extend(extract_symbols(&**expression));
            symbols.extend(extract_symbols(&**expression1));
        },
        Expression::Divide(expression, expression1) => {
            symbols.extend(extract_symbols(&**expression));
            symbols.extend(extract_symbols(&**expression1));
        },
        Expression::IsEqual(expression, expression1) => {
            symbols.extend(extract_symbols(&**expression));
            symbols.extend(extract_symbols(&**expression1));
        },
        Expression::NotEqual(expression, expression1) => {
            symbols.extend(extract_symbols(&**expression));
            symbols.extend(extract_symbols(&**expression1));
        },
        Expression::LessThan(expression, expression1) => {
            symbols.extend(extract_symbols(&**expression));
            symbols.extend(extract_symbols(&**expression1));
        },
        Expression::LessThanOrEqual(expression, expression1) => {
            symbols.extend(extract_symbols(&**expression));
            symbols.extend(extract_symbols(&**expression1));
        },
        Expression::GreaterThan(expression, expression1) => {
            symbols.extend(extract_symbols(&**expression));
            symbols.extend(extract_symbols(&**expression1));
        },
        Expression::GreaterThanOrEqual(expression, expression1) => {
            symbols.extend(extract_symbols(&**expression));
            symbols.extend(extract_symbols(&**expression1));
        },
        Expression::Not(expression) => {
            symbols.extend(extract_symbols(&**expression));
        },
        Expression::Or(expression, expression1) => {
            symbols.extend(extract_symbols(&**expression));
            symbols.extend(extract_symbols(&**expression1));
        },
        Expression::And(expression, expression1) => {
            symbols.extend(extract_symbols(&**expression));
            symbols.extend(extract_symbols(&**expression1));
        },
        Expression::IfElseExpression(if_else_expression) => {
            symbols.extend(extract_symbols(&*if_else_expression.predicate));
            symbols.extend(extract_symbols(&*&if_else_expression.true_branch));
            symbols.extend(extract_symbols(&*&if_else_expression.false_branch));
        },
        Expression::FunctionCall(function_call) => {
            symbols.insert(Symbol {
                symbol: function_call.function_name.to_owned(),
                kind: Function,
            });
            let function_arguments = &function_call.function_arguments;
            for args in function_arguments {
                symbols.extend(extract_symbols(&args));
            }
        },
        _ => {},
    }

    symbols.into_iter().collect()
}

fn usage_is_defined(usage: &Symbol, symbol_table: &Vec<Symbol>) -> bool {
    println!("Testing for usage {}...", usage.symbol);
    println!("Working with this symbol table: {:?}\n", 
        symbol_table.clone().into_iter().map(|s|s.symbol).collect::<Vec<String>>());

    symbol_table
        .iter()
        .any(|s|usage.symbol == s.symbol)
}

pub fn scope_resolution(scope: &Scope, symbol_table_stack: &mut Vec<Vec<Symbol>>) -> Vec<String> {
    let mut errors: Vec<String> = vec![];

    symbol_table_stack.push(scope.symbol_table.clone());

    for usage in &scope.usages {
        // Clone the symbol table stack because checking for valid usages
        // requires popping of the stack indenpendently per usage.
        let mut symbol_table_stack_copy = symbol_table_stack.clone();

        loop {
            let top_symbol_table = symbol_table_stack_copy.pop();
            match top_symbol_table {
                Some(innermost_symbol_table) => {
                    if usage_is_defined(usage, &innermost_symbol_table) {
                        break;
                    }
                },
                None => {
                    errors.push(format!("{:?} \"{}\" not defined in \"{}\" scope.", usage.kind, usage.symbol, scope.scope_name));
                    break;
                },
            }
        }
    }

    for inner_scope in &scope.inner_scopes {
        errors.append(&mut scope_resolution(inner_scope, symbol_table_stack));
    }

    errors.reverse();
    errors
}
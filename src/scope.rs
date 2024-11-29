use std::collections::HashSet;

use crate::ast::{Expression, FunctionDefinitionStatement, Program, Statement::*, TypeLiteral};
use SymbolKind::*;

#[derive(Debug)]
pub struct Scope {
    pub scope_name: String,
    pub symbol_table: Vec<Symbol>, // Definitions in a scope available to itself and its children.
    pub usages: Vec<Usage>, // Usages (variable references, function calls) found in this scope.
    pub inner_scopes: Vec<Scope>
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Symbol {
    pub symbol: String,
    pub kind: SymbolKind,

    // An empty vector means a variable with no type annotations or it's a usage,
    // A singleton means a variable with an explicit type annotation, and
    // A vector containing at least two type literals mean a function, with the
    // final type literal being the function return type.
    pub type_info: Vec<TypeLiteral>,

    pub start_pos: Option<usize>
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum SymbolKind {
    Variable,
    Function,
    FunctionParameter
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Usage {
    pub symbol: String,
    pub kind: UsageKind,
    pub start_pos: Option<usize>
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum UsageKind {
    Variable,
    FunctionCall
}

pub fn build_program_scope(ast_node: &Program) -> Scope {
    let mut symbol_table: Vec<Symbol> = vec![];
    let mut usages: Vec<Usage> = vec![];
    let mut inner_scopes: Vec<Scope> = vec![];
    
    let statements = &ast_node.statements;
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
                usages.append(&mut extract_usages(&assignment.rhs));
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
                inner_scopes.push(build_function_scope(&function_definition_statement));
            },
        }
    }

    Scope {
        scope_name: "(program)".to_owned(), // TO-DO: this should be the file name
        symbol_table,
        usages,
        inner_scopes,
    }
}

fn build_function_scope(function_definition_statement: &FunctionDefinitionStatement) -> Scope {
    let mut symbol_table: Vec<Symbol> = vec![];
    let mut usages: Vec<Usage> = vec![];
    let mut inner_scopes: Vec<Scope> = vec![];

    let parameters = &function_definition_statement.function_parameters;

    for (parameter, type_literal) in parameters {
        symbol_table.push(Symbol {
            symbol: parameter.clone(),
            kind: FunctionParameter,
            type_info: vec![type_literal.clone()],
            start_pos: None
        });
    }

    let statements = &function_definition_statement.function_body.statements;
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
                usages.append(&mut extract_usages(&assignment.rhs));
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
                inner_scopes.push(build_function_scope(&function_definition_statement));
            },
        }
    }

    let return_expression = &function_definition_statement.function_body.return_expression;
    match return_expression {
        Some(return_expression) => {
            usages.append(&mut extract_usages(&return_expression));
        },
        None => {},
    }

    Scope {
        scope_name: function_definition_statement.function_name.clone(),
        symbol_table,
        usages,
        inner_scopes,
    }
}

fn extract_usages(expression_node: &Expression) -> Vec<Usage> {
    let mut usages: Vec<Usage> = vec![];

    match expression_node {
        Expression::Ident(identifier) => {
            usages.push(Usage {
                symbol: identifier.identifier_name.clone(),
                kind: UsageKind::Variable,
                start_pos: Some(identifier.start_pos)
            });
        },
        Expression::Negative(expression) => {
            usages.append(&mut extract_usages(&**expression));
        },
        Expression::Plus(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        Expression::Minus(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        Expression::Times(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        Expression::Divide(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        Expression::IsEqual(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        Expression::NotEqual(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        Expression::LessThan(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        Expression::LessThanOrEqual(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        Expression::GreaterThan(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        Expression::GreaterThanOrEqual(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        Expression::Not(expression) => {
            usages.append(&mut extract_usages(&**expression));
        },
        Expression::Or(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        Expression::And(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        Expression::IfElseExpression(if_else_expression) => {
            usages.append(&mut extract_usages(&*if_else_expression.predicate));
            usages.append(&mut extract_usages(&*&if_else_expression.true_branch));
            usages.append(&mut extract_usages(&*&if_else_expression.false_branch));
        },
        Expression::FunctionCall(function_call) => {
            usages.push(Usage {
                symbol: function_call.function_name.clone(),
                kind: UsageKind::FunctionCall,
                start_pos: None
            });
            let function_arguments = &function_call.function_arguments;
            for args in function_arguments {
                usages.append(&mut extract_usages(&args));
            }
        },
        _ => {},
    }

    usages
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

fn check_for_redeclarations(scope: &Scope) -> Vec<String> {
    let mut errors: Vec<String> = vec![];
    let mut temp: HashSet<String> = HashSet::new();
    for symbol in &scope.symbol_table {
        if !temp.insert(symbol.symbol.clone()) {
            errors.push(format!("{:?} \"{}\" redeclared in \"{}\" scope.", symbol.kind, symbol.symbol, scope.scope_name));
        }
    }
    
    errors
}

pub fn scope_resolution(scope: &Scope, symbol_table_stack: &mut Vec<Vec<Symbol>>) -> Vec<String> {
    let mut errors: Vec<String> = vec![];

    errors.append(&mut check_for_redeclarations(&scope));

    // Push symbol tables into this stack for use in inner scopes.
    symbol_table_stack.push(scope.symbol_table.clone()); 

    for usage in &scope.usages {
        // Clone the symbol table stack because checking for valid usages
        // requires popping of the stack indenpendently per usage.
        let mut symbol_table_stack_copy = symbol_table_stack.clone();

        loop {
            let innermost_symbol_table = symbol_table_stack_copy.pop();
            match innermost_symbol_table {
                Some(innermost_symbol_table) => {
                    if usage_is_defined(usage, &innermost_symbol_table) {
                        break;
                    }
                },
                None => {
                    errors.push(format!("{:?} \"{}\" not defined in \"{}\" scope and beyond.", usage.kind, usage.symbol, scope.scope_name));
                    break;
                },
            }
        }
    }

    for inner_scope in &scope.inner_scopes {
        errors.append(&mut scope_resolution(inner_scope, symbol_table_stack));
    }

    errors
}
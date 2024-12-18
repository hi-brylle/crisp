use std::collections::HashSet;

use crate::ast::{Expression::{self, *}, FunctionDefinition, Program, Statement::*, TypeLiteral};
use Scope::*;
use SymbolKind::*;

pub enum Scope<'a> {
    ProgramScope(&'a Program),
    FunctionScope(&'a FunctionDefinition)
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub symbol: String,
    pub kind: SymbolKind,
    pub start_pos: Option<usize>
}

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolKind {
    Variable,
    Function,
    FunctionParameter
}

#[derive(Debug, Clone)]
pub struct Usage {
    pub symbol: String,
    pub kind: UsageKind,
    pub start_pos: Option<usize>
}

#[derive(Debug, Clone)]
pub enum UsageKind {
    Variable,
    FunctionCall
}

pub struct NameResolutionError {
    pub scope_name: String,
    pub error_kind: NameResolutionErrorKind
}

pub enum NameResolutionErrorKind {
    Redeclaration(Symbol),
    Undefined(Usage)
}

fn get_scope_name(scope: &Scope) -> String {
    match scope {
        ProgramScope(_) => String::from("(program)"),
        FunctionScope(function_definition) => function_definition.function_name.clone(),
    }
}

fn get_level_symbol_table(scope: &Scope) -> Vec<Symbol> {
    let mut symbol_table: Vec<Symbol> = vec![];

    match scope {
        ProgramScope(program) => {
            let statements = &program.statements;
            for s in statements {
                match s {
                    AssignmentStmt(assignment) => {
                        symbol_table.push(Symbol {
                            symbol: assignment.identifier.clone(),
                            kind: Variable,
                            start_pos: Some(assignment.start_pos)
                        });
                    },
                    FunctionDefStmt(function_definition_statement) => {
                        symbol_table.push(Symbol {
                            symbol: function_definition_statement.function_name.clone(),
                            kind: Function,
                            start_pos: None
                        });
                    },
                }
            }
        },
        FunctionScope(function_definition) => {
            let parameters = &function_definition.function_parameters;
            for parameter in parameters {
                symbol_table.push(Symbol {
                    symbol: parameter.parameter_name.clone(),
                    kind: FunctionParameter,
                    start_pos: None
                });
            }

            let statements = &function_definition.function_body.statements;
            for s in statements {
                match s {
                    AssignmentStmt(assignment) => {
                        symbol_table.push(Symbol {
                            symbol: assignment.identifier.clone(),
                            kind: Variable,
                            start_pos: Some(assignment.start_pos)
                        });
                    },
                    FunctionDefStmt(function_definition_statement) => {
                        symbol_table.push(Symbol {
                            symbol: function_definition_statement.function_name.clone(),
                            kind: Function,
                            start_pos: None
                        });
                    },
                }
            }
        },
    }

    symbol_table
}

fn check_for_redeclarations(scope: &Scope) -> Vec<NameResolutionError> {
    let mut errors: Vec<NameResolutionError> = vec![];
    let mut temp: HashSet<String> = HashSet::new();
    let symbol_table = get_level_symbol_table(scope);
    for symbol in symbol_table {
        if !temp.insert(symbol.symbol.clone()) {
            errors.push(NameResolutionError {
                scope_name: get_scope_name(&scope),
                error_kind: NameResolutionErrorKind::Redeclaration(symbol),
            });
        }
    }
    
    errors
}

fn extract_usages(expression_node: &Expression) -> Vec<Usage> {
    let mut usages: Vec<Usage> = vec![];

    match expression_node {
        Ident(identifier) => {
            usages.push(Usage {
                symbol: identifier.identifier_name.clone(),
                kind: UsageKind::Variable,
                start_pos: Some(identifier.start_pos)
            });
        },
        Negative(expression) => {
            usages.append(&mut extract_usages(&**expression));
        },
        Plus(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        Minus(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        Times(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        Divide(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        IsEqual(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        NotEqual(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        LessThan(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        LessThanOrEqual(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        GreaterThan(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        GreaterThanOrEqual(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        Not(expression) => {
            usages.append(&mut extract_usages(&**expression));
        },
        Or(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        And(expression, expression1) => {
            usages.append(&mut extract_usages(&**expression));
            usages.append(&mut extract_usages(&**expression1));
        },
        IfElseExpression(if_else_expression) => {
            usages.append(&mut extract_usages(&*if_else_expression.predicate));
            usages.append(&mut extract_usages(&*&if_else_expression.true_branch));
            usages.append(&mut extract_usages(&*&if_else_expression.false_branch));
        },
        FunctionCall(function_call) => {
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

fn get_level_usages(scope: &Scope) -> Vec<Usage> {
    let mut usages: Vec<Usage> = vec![];

    match scope {
        ProgramScope(program) => {
            let statements = &program.statements;
            for s in statements {
                match s {
                    AssignmentStmt(assignment) => {
                        usages.append(&mut extract_usages(&assignment.rhs.expression));
                    },
                    FunctionDefStmt(_) => {},
                }
            }
        },
        FunctionScope(function_definition) => {
            let statements = &function_definition.function_body.statements;
            for s in statements {
                match s {
                    AssignmentStmt(assignment) => {
                        usages.append(&mut extract_usages(&assignment.rhs.expression));
                    },
                    FunctionDefStmt(_) => {},
                }
            }

            let return_expression = &function_definition.function_body.return_expression_term;
            match return_expression {
                Some(return_expression) => {
                    usages.append(&mut extract_usages(&return_expression.expression));
                },
                None => {},
            }
        },
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

pub fn name_resolution(scope: &Scope, symbol_table_stack: &mut Vec<Vec<Symbol>>) -> Vec<NameResolutionError> {
    let mut errors: Vec<NameResolutionError> = vec![];

    errors.append(&mut check_for_redeclarations(&scope));

    // Push symbol tables into this stack for use in inner scopes.
    symbol_table_stack.push(get_level_symbol_table(&scope));

    for usage in &get_level_usages(scope) {
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
                    errors.push(NameResolutionError {
                        scope_name: get_scope_name(&scope),
                        error_kind: NameResolutionErrorKind::Undefined(usage.clone()),
                    });
                    break;
                },
            }
        }
    }

    match scope {
        ProgramScope(program) => {
            let statements = &program.statements;
            for s in statements {
                match s {
                    AssignmentStmt(_) => {},
                    FunctionDefStmt(function_definition) => {
                        errors.append(&mut name_resolution(&FunctionScope(function_definition), symbol_table_stack));
                    },
                }
            }
        },
        FunctionScope(function_definition) => {
            let statements = &function_definition.function_body.statements;
            for s in statements {
                match s {
                    AssignmentStmt(_) => {},
                    FunctionDefStmt(function_definition) => {
                        errors.append(&mut name_resolution(&FunctionScope(function_definition), symbol_table_stack));
                    },
                }
            }
        },
    }

    errors
}


use std::collections::HashSet;
use std::vec;

use crate::ast::{Assignment, Expression::{self, *}, ExpressionTerm, FunctionDefinition, Program, Statement};
use crate::symbol_table::{Symbol, SymbolTable};

#[derive(Debug, Clone)]
pub struct Usage {
    pub enclosing_scope_address: String,
    pub symbol: String,
    pub kind: UsageKind
}

#[derive(Debug, Clone)]
pub enum UsageKind {
    VariableReference,
    FunctionCall
}

/// Removes redeclarations and symbols that belong to reserved keywords.
pub fn clean_up_symbol_table(symbol_table: &SymbolTable) -> (SymbolTable, Vec<Symbol>, Vec<Symbol>)  {
    let mut valid_symbols: Vec<Symbol> = vec![];
    let mut temp: HashSet<String> = HashSet::new();
    let mut redeclared_symbols: Vec<Symbol> = vec![];

    let mut clashes_against_reserved: Vec<Symbol> = vec![];
    let mut reserved: HashSet<String> = HashSet::new();
    reserved.insert("let".to_string());
    reserved.insert("fun".to_string());
    reserved.insert("return".to_string());
    reserved.insert("if".to_string());
    reserved.insert("else".to_string());
    reserved.insert("true".to_string());
    reserved.insert("false".to_string());
    reserved.insert("Number".to_string());
    reserved.insert("Boolean".to_string());
    reserved.insert("String".to_string());
    reserved.insert("Unit".to_string());

    let symbol_table = &symbol_table.symbol_table;
    
    for symbol in symbol_table {
        if !temp.insert(symbol.scope_address.clone()) {
            redeclared_symbols.push(symbol.clone());
        } else
        if reserved.contains(&symbol.symbol) {
            clashes_against_reserved.push(symbol.clone());
        } else {
            valid_symbols.push(symbol.clone());
        }
    }
    
    (SymbolTable { symbol_table: valid_symbols }, redeclared_symbols, clashes_against_reserved)
}

pub fn get_program_usages(program_ast: &Program) -> Vec<Usage> {
    let mut usages: Vec<Usage> = vec![];

    for statement in &program_ast.statements {
        usages.append(&mut get_statement_usages(&statement));
    }

    usages
}

fn get_statement_usages(statement: &Statement) -> Vec<Usage> {
    let mut usages: Vec<Usage> = vec![];

    match statement {
        Statement::AssignmentStmt(assignment) => {
            usages.append(&mut get_assignment_usages(&assignment));
        },
        Statement::FunctionDefStmt(function_definition) => {
            usages.append(&mut get_function_def_usages(&function_definition));
        },
    }

    usages
}

fn get_assignment_usages(assignment: &Assignment) -> Vec<Usage> {
    let mut usages: Vec<Usage> = vec![];

    usages.append(&mut get_expression_term_usages(&assignment.rhs));

    usages
}

fn get_function_def_usages(function_definition: &FunctionDefinition) -> Vec<Usage> {
    let mut usages: Vec<Usage> = vec![];

    for statement in &function_definition.function_body.statements {
        usages.append(&mut get_statement_usages(&statement));
    }

    let return_expression_term = &function_definition.function_body.return_expression_term;
    match return_expression_term {
        Some(return_expr_term) => {
            usages.append(&mut get_expression_term_usages(&return_expr_term));
        },
        None => {},
    }

    usages
}

fn get_expression_term_usages(expression_term: &ExpressionTerm) -> Vec<Usage> {
    let mut usages: Vec<Usage> = vec![];

    let enclosing_scope_address = expression_term.enclosing_scope_address.clone();

    fn get_expression_usages(enclosing_scope_address: &str, expression_node: &Expression) -> Vec<Usage> {
        let mut usages: Vec<Usage> = vec![];

        match expression_node {
            Ident(identifier) => {
                usages.push(Usage {
                    enclosing_scope_address: enclosing_scope_address.to_owned(),
                    symbol: identifier.identifier_name.clone(),
                    kind: UsageKind::VariableReference,
                });
            },
            Negative(expression) => {
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
            },
            Plus(expression, expression1) => {
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
            },
            Minus(expression, expression1) => {
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
            },
            Times(expression, expression1) => {
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
            },
            Divide(expression, expression1) => {
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
            },
            IsEqual(expression, expression1) => {
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
            },
            NotEqual(expression, expression1) => {
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
            },
            LessThan(expression, expression1) => {
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
            },
            LessThanOrEqual(expression, expression1) => {
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
            },
            GreaterThan(expression, expression1) => {
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
            },
            GreaterThanOrEqual(expression, expression1) => {
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
            },
            Not(expression) => {
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
            },
            Or(expression, expression1) => {
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
            },
            And(expression, expression1) => {
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
                usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
            },
            IfElseExpression(if_else_expression) => {
                usages.append(&mut get_expression_usages(enclosing_scope_address, &*if_else_expression.predicate));
                usages.append(&mut get_expression_usages(enclosing_scope_address, &*&if_else_expression.true_branch));
                usages.append(&mut get_expression_usages(enclosing_scope_address, &*&if_else_expression.false_branch));
            },
            FunctionCall(function_call) => {
                usages.push(Usage {
                    enclosing_scope_address: enclosing_scope_address.to_owned(),
                    symbol: function_call.function_name.clone(),
                    kind: UsageKind::FunctionCall,
                });

                let function_arguments = &function_call.function_arguments;
                for args in function_arguments {
                    usages.append(&mut get_expression_usages(enclosing_scope_address, &args));
                }
            },
            _ => {},
        }

        usages
    }

    usages.append(&mut get_expression_usages(&enclosing_scope_address, &expression_term.expression));

    usages
}


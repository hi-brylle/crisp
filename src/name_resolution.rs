// use std::{collections::HashSet, ops::Bound};
// use std::vec;

// use crate::{ast::{Assignment, Expression::{self, *}, ExpressionTerm, FunctionDefinition, Program, Statement}, symbol_table};
// use crate::symbol_table::{SymbolTable, Symbol, SymbolKind};

// #[derive(Debug, Clone)]
// pub struct Usage {
//     pub symbol: String,
//     pub enclosing_scope_address: String,
//     pub kind: UsageKind
// }

// #[derive(Debug, Clone)]
// pub enum UsageKind {
//     VariableReference(usize),
//     FunctionCall
// }

// #[derive(Debug)]
// pub struct BoundUsage {
//     pub usage: Usage,
//     pub associated_symbol: Symbol
// }

// /// Removes redeclarations and symbols that belong to reserved keywords.
// pub fn clean_up_symbol_table(symbol_table: &SymbolTable) -> (SymbolTable, Vec<Symbol>, Vec<Symbol>)  {
//     let mut valid_symbols: Vec<Symbol> = vec![];
//     let mut temp: HashSet<String> = HashSet::new();
//     let mut redeclared_symbols: Vec<Symbol> = vec![];

//     let mut clashes_against_reserved: Vec<Symbol> = vec![];
//     let mut reserved: HashSet<String> = HashSet::new();
//     reserved.insert("let".to_string());
//     reserved.insert("fun".to_string());
//     reserved.insert("return".to_string());
//     reserved.insert("if".to_string());
//     reserved.insert("else".to_string());
//     reserved.insert("true".to_string());
//     reserved.insert("false".to_string());
//     reserved.insert("Number".to_string());
//     reserved.insert("Boolean".to_string());
//     reserved.insert("String".to_string());
//     reserved.insert("Unit".to_string());

//     let symbol_table = &symbol_table.symbol_table;
    
//     for symbol in symbol_table {
//         if !temp.insert(symbol.scope_address.clone()) {
//             match &symbol.kind {
//                 SymbolKind::VariableDeclaration(_) => {
//                     redeclared_symbols.push(symbol.clone());
//                 },
//                 SymbolKind::FunctionDefinition(_) => {
//                     redeclared_symbols.push(symbol.clone());
//                 },
//                 SymbolKind::FunctionParameter(_) => {
//                     /*
//                     Exclude function parameters from redeclarations
//                     to prevent redundant error reporting when semantic analysis
//                     detects function definition redeclarations for
//                     similarly named parameters.

//                     Example:

//                     ...
//                     fun add(addend1: Number, addend2: Number): Number {
//                         let temp = (+ addend1 addend2);
//                         return temp
//                     };

//                     fun add(addend1: Number): Number {
//                         let res = (- addend1);
//                         return res
//                     };
//                     ...

//                     Second add function should only be tagged as a redeclaration,
//                     not its addend1 parameter.
//                     */
//                 },
//             }
//         } else
//         if reserved.contains(&symbol.symbol) {
//             clashes_against_reserved.push(symbol.clone());
//         } else {
//             valid_symbols.push(symbol.clone());
//         }
//     }
    
//     (SymbolTable { symbol_table: valid_symbols }, redeclared_symbols, clashes_against_reserved)
// }

// pub fn get_program_usages(program_ast: &Program) -> Vec<Usage> {
//     let mut usages: Vec<Usage> = vec![];

//     for statement in &program_ast.statements {
//         usages.append(&mut get_statement_usages(&statement));
//     }

//     usages
// }

// fn get_statement_usages(statement: &Statement) -> Vec<Usage> {
//     let mut usages: Vec<Usage> = vec![];

//     match statement {
//         Statement::AssignmentStmt(assignment) => {
//             usages.append(&mut get_assignment_usages(&assignment));
//         },
//         Statement::FunctionDefStmt(function_definition) => {
//             usages.append(&mut get_function_def_usages(&function_definition));
//         },
//     }

//     usages
// }

// fn get_assignment_usages(assignment: &Assignment) -> Vec<Usage> {
//     let mut usages: Vec<Usage> = vec![];

//     usages.append(&mut get_expression_term_usages(&assignment.rhs));

//     usages
// }

// fn get_function_def_usages(function_definition: &FunctionDefinition) -> Vec<Usage> {
//     let mut usages: Vec<Usage> = vec![];

//     for statement in &function_definition.function_body.statements {
//         usages.append(&mut get_statement_usages(&statement));
//     }

//     let return_expression_term = &function_definition.function_body.return_expression_term;
//     match return_expression_term {
//         Some(return_expr_term) => {
//             usages.append(&mut get_expression_term_usages(&return_expr_term));
//         },
//         None => {},
//     }

//     usages
// }

// fn get_expression_term_usages(expression_term: &ExpressionTerm) -> Vec<Usage> {
//     let mut usages: Vec<Usage> = vec![];

//     let enclosing_scope_address = expression_term.enclosing_scope_address.clone();

//     fn get_expression_usages(enclosing_scope_address: &str, expression_node: &Expression) -> Vec<Usage> {
//         let mut usages: Vec<Usage> = vec![];

//         match expression_node {
//             Ident(identifier) => {
//                 usages.push(Usage {
//                     enclosing_scope_address: enclosing_scope_address.to_owned(),
//                     symbol: identifier.identifier_name.clone(),
//                     kind: UsageKind::VariableReference(identifier.start_pos),
//                 });
//             },
//             Negative(expression) => {
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
//             },
//             Plus(expression, expression1) => {
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
//             },
//             Minus(expression, expression1) => {
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
//             },
//             Times(expression, expression1) => {
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
//             },
//             Divide(expression, expression1) => {
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
//             },
//             IsEqual(expression, expression1) => {
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
//             },
//             NotEqual(expression, expression1) => {
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
//             },
//             LessThan(expression, expression1) => {
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
//             },
//             LessThanOrEqual(expression, expression1) => {
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
//             },
//             GreaterThan(expression, expression1) => {
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
//             },
//             GreaterThanOrEqual(expression, expression1) => {
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
//             },
//             Not(expression) => {
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
//             },
//             Or(expression, expression1) => {
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
//             },
//             And(expression, expression1) => {
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression));
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &**expression1));
//             },
//             IfElseExpression(if_else_expression) => {
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &*if_else_expression.predicate));
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &*&if_else_expression.true_branch));
//                 usages.append(&mut get_expression_usages(enclosing_scope_address, &*&if_else_expression.false_branch));
//             },
//             FunctionCall(function_call) => {
//                 usages.push(Usage {
//                     enclosing_scope_address: enclosing_scope_address.to_owned(),
//                     symbol: function_call.function_name.clone(),
//                     kind: UsageKind::FunctionCall,
//                 });

//                 let function_arguments = &function_call.function_arguments;
//                 for args in function_arguments {
//                     usages.append(&mut get_expression_usages(enclosing_scope_address, &args));
//                 }
//             },
//             _ => {},
//         }

//         usages
//     }

//     usages.append(&mut get_expression_usages(&enclosing_scope_address, &expression_term.expression));

//     usages
// }

// pub fn resolve_names(usages: &Vec<Usage>, valid_symbol_table: &SymbolTable) -> Vec<BoundUsage> {
//     let mut bound_usages: Vec<BoundUsage> = vec![];

//     for usage in usages {
//         match resolve_usage(usage, valid_symbol_table) {
//             Some(bound_usage) => bound_usages.push(bound_usage),
//             None => {},
//         }
//     }

//     bound_usages
// }

// fn get_depth(scope_address: &String) -> usize {
//     scope_address.split("::").collect::<Vec<&str>>().len()
// }

// fn resolve_usage(usage: &Usage, valid_symbol_table: &SymbolTable) -> Option<BoundUsage> {
//     match usage.kind {
//         UsageKind::VariableReference(usage_start_pos) => {
//             // TO-DO: FIX THIS BUGGY MESS
//             for symbol in &valid_symbol_table.symbol_table {
//                 match &symbol.kind {
//                     SymbolKind::VariableDeclaration(symbol_start_pos) => {
//                         if symbol.symbol == usage.symbol &&
//                             get_depth(&symbol.scope_address) < get_depth(&usage.enclosing_scope_address) + 1 && 
//                             *symbol_start_pos < usage_start_pos {
//                             return Some(BoundUsage {
//                                 usage: usage.clone(),
//                                 associated_symbol: symbol.clone(),
//                             })
//                         }
//                     },
//                     SymbolKind::FunctionParameter(_) => {
//                         if symbol.symbol == usage.symbol 
//                             && get_depth(&symbol.scope_address) < get_depth(&usage.enclosing_scope_address) + 1 {
//                             return Some(BoundUsage {
//                                 usage: usage.clone(),
//                                 associated_symbol: symbol.clone(),
//                             })
//                         }
//                     },
//                     _ => {},
//                 }
//             }
//         },
//         UsageKind::FunctionCall => {
//             for symbol in &valid_symbol_table.symbol_table {
//                 match &symbol.kind {
//                     SymbolKind::FunctionDefinition(_) => {
//                         if symbol.symbol == usage.symbol 
//                             && get_depth(&symbol.scope_address) < get_depth(&usage.enclosing_scope_address) + 1 {
//                             return Some(BoundUsage {
//                                 usage: usage.clone(),
//                                 associated_symbol: symbol.clone(),
//                             })
//                         }
//                     },
//                     _ => {}
//                 }
//             }
//         },
//     }

//     None
// }

// fn get_candidate_bindings(usage: &Usage, valid_symbol_table: &SymbolTable) -> Vec<BoundUsage> {
//     let mut candidate_bindings: Vec<BoundUsage> = vec![];

//     match usage.kind {
//         UsageKind::VariableReference(_) => { /* to-do */ },
//         UsageKind::FunctionCall => {
//             for symbol in &valid_symbol_table.symbol_table {
//                 if symbol.symbol == usage.symbol &&
//                     get_depth(&symbol.scope_address) < get_depth(&usage.enclosing_scope_address) + 1 {
//                     candidate_bindings.push(BoundUsage {
//                         usage: usage.clone(),
//                         associated_symbol: symbol.clone(),
//                     });
//                 }
//             }
//         },
//     }

//     candidate_bindings
// }

// pub fn resolve_candidate_bindings(usages: &Vec<Usage>, valid_symbol_table: &SymbolTable) -> Vec<BoundUsage> {
//     let mut candidate_bindings: Vec<BoundUsage> = vec![];

//     for usage in usages {
//         candidate_bindings.append(&mut get_candidate_bindings(usage, valid_symbol_table));
//     }

//     candidate_bindings
// }
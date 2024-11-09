use crate::{ast::*, Rule};

pub fn build_program_ast(pair: pest::iterators::Pair<Rule>) -> Program {
    debug_pair(&pair);
    
    let mut statements: Vec<Statement> = vec![];

    match pair.as_rule() {
        Rule::Program => {
            // Contains one or more Statements and the EOI because pest includes it.
            let mut children = pair.into_inner();
            
            for c in children {
                match c.as_rule() {
                    Rule::EOI => { println!("Skip EOI.") },
                    Rule::Statement => {
                        statements.push(build_statement_ast(c));
                    },
                    _ => unreachable!("Unexpected Program node child! (found {:?})", c.as_rule())
                }
            }
        },
        _ => {}
    }

    Program {
        statements
    }
}

fn build_statement_ast(pair: pest::iterators::Pair<Rule>) -> Statement {
    debug_pair(&pair);

    // Always contains one item: the type of the Statement.
    let mut children = pair.into_inner();
    let only_child = children.next().unwrap();
    
    match only_child.as_rule() {
        Rule::Assignment => { 
            Statement::AssignmentStmt(build_assignment_ast(only_child))
        },
        _ => todo!("Handle other Statement types! (found {:?})", only_child.as_rule())
    } 
}

fn build_assignment_ast(pair: pest::iterators::Pair<Rule>) -> Assignment {
    debug_pair(&pair);

    // Contains possibly three items: the identifier, an optional type, and the rhs.
    let mut children = pair.into_inner();

    match children.len() {
        2 => {
            let identifier = children.next().unwrap().as_str().to_owned();
            let rhs = build_expression_ast(children.next().unwrap());
        
            Assignment {
                identifier,
                type_string: TypeString::Unspecified,
                rhs,
            }   
        },
        3 => {
            let identifier = children.next().unwrap().as_str().to_owned();
            let type_string_raw = children.next().unwrap().as_str();
            let rhs = build_expression_ast(children.next().unwrap());
        
            let type_string: TypeString = match type_string_raw {
                "Number" => TypeString::Number,
                "Boolean" => TypeString::Boolean,
                _ => todo!("Some type string has not been accounted for: {}", type_string_raw)
            };

            Assignment {
                identifier,
                type_string,
                rhs,
            }            
        }
        _ => todo!("Some Assignment AST builder children have not been accounted for! Unexpected children size: {}", children.len())
    }
}

fn build_expression_ast(pair: pest::iterators::Pair<Rule>) -> Expression {
    debug_pair(&pair);
    
    match pair.as_rule() {
        Rule::Expression => {
            // IM NOT SURE ABOUT THIS, KEEP DEBUGGING
            let mut children = pair.into_inner();
            println!("\nExpression recursive call debug, children of length {}: {:?}\n", children.len(), children);
            build_expression_ast(children.next().unwrap())
        }
        Rule::IntegerLiteral => {
            let integer = pair.as_str().parse::<i64>().unwrap();
            Expression::Number(integer)
        },
        Rule::Identifier => {
            let identifier = pair.as_str().parse::<String>().unwrap();
            Expression::Identifier(identifier)
        }
        Rule::FunctionCall => {
            let mut children = pair.into_inner();
            println!("\nFunction call debug, children of length {}: {:?}\n", children.len(), children);

            let function_name = children.next().unwrap().as_str().parse::<String>().unwrap();
            let mut function_arguments: Vec<Expression> = vec![];

            for c in children {
                match c.as_rule() {
                    Rule::Expression => {
                        function_arguments.push(build_expression_ast(c));                        
                    },
                    _ => unreachable!("Function call included an argument that isn't an Expression! (found {:?})", c.as_rule())
                }
            }
            
            Expression::FunctionCall(FunctionCall { function_name, function_arguments })
        }
        Rule::BooleanLiteral => {
            match pair.as_str() {
                "true" => Expression::Boolean(true),
                "false" => Expression::Boolean(false),
                _ => unreachable!("There is no third boolean value!")
            } 
        }
        Rule::BinaryExpression => {
            // Always contains three items: the binary operator and the lhs and rhs operands.
            let mut children = pair.into_inner();
            // println!("{}", children.len());
            let binary_operator = children.next().unwrap().as_str();
            
            match binary_operator {
                "+" => Expression::Plus(Box::new(build_expression_ast(children.next().unwrap())), Box::new(build_expression_ast(children.next().unwrap()))),
                "-" => Expression::Minus(Box::new(build_expression_ast(children.next().unwrap())), Box::new(build_expression_ast(children.next().unwrap()))),
                "*" => Expression::Times(Box::new(build_expression_ast(children.next().unwrap())), Box::new(build_expression_ast(children.next().unwrap()))),
                "/" => Expression::Divide(Box::new(build_expression_ast(children.next().unwrap())), Box::new(build_expression_ast(children.next().unwrap()))),
                "==" => Expression::IsEqual(Box::new(build_expression_ast(children.next().unwrap())), Box::new(build_expression_ast(children.next().unwrap()))),
                "!=" => Expression::NotEqual(Box::new(build_expression_ast(children.next().unwrap())), Box::new(build_expression_ast(children.next().unwrap()))),
                "<" => Expression::LessThan(Box::new(build_expression_ast(children.next().unwrap())), Box::new(build_expression_ast(children.next().unwrap()))),
                "<=" => Expression::LessThanOrEqual(Box::new(build_expression_ast(children.next().unwrap())), Box::new(build_expression_ast(children.next().unwrap()))),
                ">" => Expression::GreaterThan(Box::new(build_expression_ast(children.next().unwrap())), Box::new(build_expression_ast(children.next().unwrap()))),
                ">=" => Expression::GreaterThanOrEqual(Box::new(build_expression_ast(children.next().unwrap())), Box::new(build_expression_ast(children.next().unwrap()))),
                "or" => Expression::Or(Box::new(build_expression_ast(children.next().unwrap())), Box::new(build_expression_ast(children.next().unwrap()))),
                "and" => Expression::And(Box::new(build_expression_ast(children.next().unwrap())), Box::new(build_expression_ast(children.next().unwrap()))),
                _ => unreachable!("Some binary operator has not been accounted for: {}", binary_operator)
            }
        }
        Rule::UnaryExpression => {
            // Always contains two items: the unary operator and its operand.
            let mut children = pair.into_inner();
            // println!("{}", children.len());
            let unary_operator = children.next().unwrap().as_str();

            match unary_operator {
                "-" => Expression::Negative(Box::new(build_expression_ast(children.next().unwrap()))),
                "not" => Expression::Not(Box::new(build_expression_ast(children.next().unwrap()))),
                _ => unreachable!("Some unary operator has not been accounted for: {}", unary_operator)
            }
        }
        _ => todo!("Add other expression types! (found {:?})", pair.as_rule())
    }
}

fn debug_pair(pair: &pest::iterators::Pair<Rule>) {
    println!("\nPARENT: {:?}\n", pair);
    for c in pair.clone().into_inner() {
        println!("CHILD: {:?}\n", c)
    }
    println!("=========");
}
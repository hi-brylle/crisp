use crate::{ast::*, Rule};

pub fn build_ast(pair: pest::iterators::Pair<Rule>) -> Program {
    debug_pair(&pair);

    match pair.as_rule() {
        Rule::statement => {
            Program {
                statement: build_statement_ast(pair.into_inner().next().unwrap())
            }
        }
        _ => unreachable!("[build_ast] we fucked up tryna parse this: {:?}\n", pair)
    }
}

fn build_statement_ast(pair: pest::iterators::Pair<Rule>) -> Statement {
    debug_pair(&pair);

    Statement::AssignmentStmt(build_assignment_ast(pair))    
}

fn build_assignment_ast(pair: pest::iterators::Pair<Rule>) -> Assignment {
    debug_pair(&pair);

    let mut children = pair.clone().into_inner();

    Assignment {
        identifier: children.next().unwrap().as_str().to_owned(),
        rhs: build_expr_ast(children.next().unwrap()),
    }
}

fn build_expr_ast(pair: pest::iterators::Pair<Rule>) -> Expression {
    debug_pair(&pair);

    match pair.as_rule() {
        Rule::expression => {
            let mut children = pair.into_inner();
            match children.len() == 1 {
                true => {
                    build_expr_ast(children.next().unwrap())
                }
                false => {
                    let operator = children.next().unwrap().as_str();
                    let left = build_expr_ast(children.next().unwrap());
                    let right = build_expr_ast(children.next().unwrap());
        
                    match operator {
                        "+" => Expression::Add(Box::new(left), Box::new(right)),
                        "-" => Expression::Sub(Box::new(left), Box::new(right)),
                        "*" => Expression::Mul(Box::new(left), Box::new(right)),
                        "/" => Expression::Div(Box::new(left), Box::new(right)),
                        "==" => Expression::IsEq(Box::new(left), Box::new(right)),
                        "!=" => Expression::NotEq(Box::new(left), Box::new(right)),
                        "<" => Expression::Less(Box::new(left), Box::new(right)),
                        "<=" => Expression::LessEq(Box::new(left), Box::new(right)),
                        ">" => Expression::Greater(Box::new(left), Box::new(right)),
                        ">=" => Expression::GreaterEq(Box::new(left), Box::new(right)),
                        _ => unreachable!()
                    }
                }
            }
        }
        Rule::number => {
            let number = pair.as_str().parse::<i64>().unwrap();
            Expression::Number(number)
        }
        Rule::boolean => {
            match pair.as_str() {
                "true" => Expression::Boolean(true),
                "false" => Expression::Boolean(false),
                _ => unreachable!("[build_expr_ast] we fucked up tryna parse this: {:?}\n", pair)
            }
        }
        _ => unreachable!("[build_expr_ast] we fucked up tryna parse this: {:?}\n", pair)
    }
}

fn debug_pair(pair: &pest::iterators::Pair<Rule>) {
    println!("Rule:    {:?}", pair.as_rule());
    println!("Span:    {:?}", pair.as_span());
    println!("Text:    {}\n", pair.as_str());
}
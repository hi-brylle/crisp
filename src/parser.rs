use crate::{ast::*, Rule};

pub fn build_program_ast(pair: pest::iterators::Pair<Rule>) -> Program {
    debug_pair(&pair);

    match pair.as_rule() {
        Rule::Statement => {
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
        Rule::Expression => {
            let mut children = pair.into_inner();
            build_expr_ast(children.next().unwrap())
        }
        Rule::UnaryExpression => {
            let mut children = pair.into_inner();
            let unary_op = children.next().unwrap().as_str();
            let operand = build_expr_ast(children.next().unwrap());

            match unary_op {
                "-" => Expression::Negative(Box::new(operand)),
                "not" => Expression::Not(Box::new(operand)),
                _ => unreachable!("Some unary operator not reached.")
            }
        }
        Rule::BinaryExpression => {
            let mut children = pair.into_inner();
            let binary_op = children.next().unwrap().as_str();
            let lhs = build_expr_ast(children.next().unwrap());
            let rhs = build_expr_ast(children.next().unwrap());

            match binary_op {
                "+" => Expression::Plus(Box::new(lhs), Box::new(rhs)),
                "-" => Expression::Minus(Box::new(lhs), Box::new(rhs)),
                "*" => Expression::Times(Box::new(lhs), Box::new(rhs)),
                "/" => Expression::Divide(Box::new(lhs), Box::new(rhs)),
                "==" => Expression::IsEqual(Box::new(lhs), Box::new(rhs)),
                "!=" => Expression::NotEqual(Box::new(lhs), Box::new(rhs)),
                "<" => Expression::LessThan(Box::new(lhs), Box::new(rhs)),
                "<=" => Expression::LessThanOrEqual(Box::new(lhs), Box::new(rhs)),
                ">" => Expression::GreaterThan(Box::new(lhs), Box::new(rhs)),
                ">=" => Expression::GreaterThanOrEqual(Box::new(lhs), Box::new(rhs)),
                "or" => Expression::Or(Box::new(lhs), Box::new(rhs)),
                "and" => Expression::And(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!("Some binary operator not reached.")
            }
        }
        Rule::IntegerLiteral => {
            let number = pair.as_str().parse::<i64>().unwrap();
            Expression::Number(number)
        }
        Rule::BooleanLiteral => {
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
use crate::{ast2::*, Rule};

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
                    _ => {}
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

    // Always contains one item, which is the type of the Statement.
    let mut children = pair.into_inner();
    let only_child = children.next().unwrap();
    
    match only_child.as_rule() {
        Rule::Assignment => { 
            Statement::AssignmentStmt(build_assignment_ast(only_child))
        },
        _ => todo!("Handle other Statement types!")
    } 
}

fn build_assignment_ast(pair: pest::iterators::Pair<Rule>) -> Assignment {
    debug_pair(&pair);

    // Always contains two items: the identifier and the rhs.
    let mut children = pair.into_inner();
    
    let identifier = children.next().unwrap().as_str().to_owned();
    let rhs = build_expression_ast(children.next().unwrap());

    Assignment {
        identifier,
        rhs,
    }
}

fn build_expression_ast(pair: pest::iterators::Pair<Rule>) -> Expression {
    debug_pair(&pair);
    
    match pair.as_rule() {
        Rule::Expression => {
            // IM NOT SURE ABOUT THIS, KEEP DEBUGGING
            let mut children = pair.into_inner();
            println!("\nExpression recursive call debug, children: {:?}\n", children);
            build_expression_ast(children.next().unwrap())
        }
        Rule::IntegerLiteral => {
            let integer = pair.as_str().parse::<i64>().unwrap();
            Expression::Number(integer)
        },
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
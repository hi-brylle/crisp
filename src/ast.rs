#[derive(Debug)]
pub struct Program {
    pub statement: Statement
}

#[derive(Debug)]
pub enum Statement {
    AssignmentStmt(Assignment)
}

#[derive(Debug)]
pub struct Assignment {
    pub identifier: String,
    pub rhs: Expression
}

#[derive(Debug)]
pub enum Expression {
    Number(i64),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
}
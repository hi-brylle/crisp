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
    Negative(Box<Expression>),
    Plus(Box<Expression>, Box<Expression>),
    Minus(Box<Expression>, Box<Expression>),
    Times(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    IsEqual(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
    LessThanOrEqual(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    GreaterThanOrEqual(Box<Expression>, Box<Expression>),

    Boolean(bool),
    Not(Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
}
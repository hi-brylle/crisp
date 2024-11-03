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
    Boolean(bool),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    IsEq(Box<Expression>, Box<Expression>),
    NotEq(Box<Expression>, Box<Expression>),
    Less(Box<Expression>, Box<Expression>),
    LessEq(Box<Expression>, Box<Expression>),
    Greater(Box<Expression>, Box<Expression>),
    GreaterEq(Box<Expression>, Box<Expression>)
}
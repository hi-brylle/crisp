#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>
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

    Boolean(bool),
    Not(Box<Expression>)
}
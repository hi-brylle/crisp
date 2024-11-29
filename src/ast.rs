#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>
}

#[derive(Debug)]
pub enum Statement {
    AssignmentStmt(Assignment),
    FunctionDefStmt(FunctionDefinition)
}

#[derive(Debug)]
pub struct Assignment {
    pub identifier: String,
    pub type_annotation: Option<TypeLiteral>,
    pub rhs: Expression,
    pub start_pos: usize
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub function_name: String,
    pub function_parameters: Vec<(String, TypeLiteral)>,
    pub function_return_type: TypeLiteral,
    pub function_body: FunctionBody
}

#[derive(Debug)]
pub struct FunctionBody {
    pub statements: Vec<Statement>,
    pub return_expression: Option<Expression>
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TypeLiteral {
    Number,
    Boolean,
    String,
    Unit
}

#[derive(Debug)]
pub enum Expression {
    Number(f64),
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

    Ident(Identifier),

    FunctionCall(FunctionCall),

    IfElseExpression(IfElseExpression),

    StringLiteral(String)
}

#[derive(Debug)]
pub struct Identifier {
    pub identifier_name: String,
    pub start_pos: usize
}

#[derive(Debug)]
pub struct FunctionCall {
    pub function_name: String,
    pub function_arguments: Vec<Expression>
}

#[derive(Debug)]
pub struct IfElseExpression {
    pub predicate: Box<Expression>,
    pub true_branch: Box<Expression>,
    pub false_branch: Box<Expression>,
}
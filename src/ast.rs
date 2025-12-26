use crate::token::Token;
use crate::token::TokenKind;

#[derive(Debug, Clone)]

pub enum Expression {
    Assign(Token, Box<Expression>),
    Binary(Box<Expression>, Token , Box<Expression>),
    Unary(Token, Box<Expression>), 
    Logical(Box<Expression>, Token, Box<Expression>),
    Literal(Value), 
    Grouping(Box<Expression>),
    Variable(Token)
}

#[derive(Debug, Clone)]

pub enum Value {
    String(String),
    Int(i128),
    Float(f64),
    Bool(bool),
    None
}

#[derive(Debug, Clone)]

pub enum Statement {
    Var(String, Expression), 
    If(Expression, Box<Statement>, Box<Statement>), //For the case of no else, just set to some useless expression. 
    Print(Expression),
    Expression(Expression),
    Block(Box<Vec<Statement>>)
}


use crate::token::Token;
use crate::token::TokenKind;

#[derive(Debug, Clone)]

pub enum Expression {
    Assign(Token, Box<Expression>),
    Binary(Box<Expression>, Token , Box<Expression>),
    Unary(Token, Box<Expression>), 
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
    Print(Expression),
    Expression(Expression),
    Var(String, Expression), 
    Block(Box<Vec<Statement>>)
}


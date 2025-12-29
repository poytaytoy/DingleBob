use crate::interpreter;
use crate::token::Token;
use crate::token::TokenKind;
use crate::interpreter::Interpreter;
use crate::environment::Environment;
use crate::func::Func;
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;
use std::cell::RefCell; 

#[derive(Debug, Clone)]

pub enum Expression {
    Assign(Box<Expression>, Token, Box<Expression>),
    Binary(Box<Expression>, Token , Box<Expression>),
    Unary(Token, Box<Expression>), 
    Call(Box<Expression>, Token, Box<Vec<Expression>>),
    Logical(Box<Expression>, Token, Box<Expression>),
    Literal(Value), 
    Grouping(Box<Expression>),
    Variable(Token),
    List(Box<Vec<Expression>>, Token),
    Lambda(Vec<Token>, Box<Vec<Statement>>), 
    Index(Box<Expression>, Token,  Box<Expression>)
}

#[derive(Clone)]
 
pub enum Value
{
    String(String),
    Int(i128),
    Float(f64),
    Bool(bool),
    Call(Rc<dyn Func>, Rc<RefCell<Environment>>),
    List(Rc<RefCell<Vec<Value>>>),
    None
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Value::String(s) => f.debug_tuple("String").field(s).finish(),
            Value::Int(i) => f.debug_tuple("Int").field(i).finish(),
            Value::Float(fl) => f.debug_tuple("Float").field(fl).finish(),
            Value::Bool(b) => f.debug_tuple("Bool").field(b).finish(),
            Value::Call(callee, env) => write!(f, "{}", format!("Call(<{}>)", callee.toString())),
            Value::List(lst) => f.debug_tuple("List").field(lst).finish(),
            Value::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone)]

pub enum Statement {
    Var(Token, Expression),
    Expression(Expression), 
    Function(Token, Vec<Token>, Box<Vec<Statement>>),
    If(Expression, Box<Statement>, Box<Statement>), //For the case of no else, just set to some useless expression. 
    Print(Expression),
    Return(Token, Expression),
    While(Expression, Box<Statement>),
    Break(Token), 
    Block(Box<Vec<Statement>>)
}

#[derive(Debug, Clone)]

pub enum BreakResult {
    Return(Token, Value), 
    Error(String), 
    Break(Token), 
}
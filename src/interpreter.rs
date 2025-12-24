use crate::ast::Expression;
use crate::ast::Value;
use crate::token::TokenKind; 
use crate::token::Token; 
use std::process; 

//TODO FIX THE PRIVACY LEVELS OF THE INTERPRET 


pub trait InterpretExpression {

    fn new()->Self;

    fn evaluate_binary(&mut self, l: Box<Expression>, o:Token, r:Box<Expression>) -> Value;
    fn evaluate_unary(&mut self,  o: Token, r: Box<Expression>) -> Value; 
    fn evaluate_literal(&mut self, v: Value) -> Value; 
    fn evaluate_grouping(&mut self, exp: Box<Expression>) -> Value; 

    fn evaluate(&mut self, expression: Expression) -> Value{
        match expression{ 
            Expression::Binary(l, o, r) => self.evaluate_binary(l, o, r), 
            Expression::Unary(o, r) => self.evaluate_unary(o, r), 
            Expression::Literal(v) => self.evaluate_literal(v),
            Expression::Grouping(exp) => self.evaluate_grouping(exp), 
        }
    }
    
}

pub struct Interpreter{

}

impl Interpreter { 
    
}

impl InterpretExpression for Interpreter {

    fn new() -> Self {
        Interpreter{}
    }
    
    fn evaluate_binary(&mut self, l: Box<Expression>, o: Token, r: Box<Expression>) -> Value {
        let l_ev: Value = self.evaluate(*l);
        let r_ev: Value = self.evaluate(*r);

        match o.kind {
            TokenKind::PLUS => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Value::Int(m + n),
                (Value::Float(m), Value::Float(n)) => Value::Float(m + n),
                (Value::Int(m), Value::Float(n)) => Value::Float((m as f64) + n),
                (Value::Float(m), Value::Int(n)) => Value::Float(m + (n as f64)),
                (Value::String(m), Value::Int(n)) => Value::String(m.clone() + &n.to_string()),
                (Value::String(m), Value::Float(n)) => Value::String(m.clone() + &n.to_string()),
                (Value::String(m), Value::String(n)) => Value::String(m.clone() + &n),
                _ => {
                    handle_error(&format!("Invalid operation performed with '+'"), o.line);
                    Value::None
                }
            },

            TokenKind::MINUS => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Value::Int(m - n),
                (Value::Float(m), Value::Float(n)) => Value::Float(m - n),
                (Value::Int(m), Value::Float(n)) => Value::Float((m as f64) - n),
                (Value::Float(m), Value::Int(n)) => Value::Float(m - (n as f64)),
                _ => {
                    handle_error(&format!("Invalid operation performed with '-'"), o.line);
                    Value::None
                }
            },

            TokenKind::STAR => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Value::Int(m * n),
                (Value::Float(m), Value::Float(n)) => Value::Float(m * n),
                (Value::Int(m), Value::Float(n)) => Value::Float((m as f64) * n),
                (Value::Float(m), Value::Int(n)) => Value::Float(m * (n as f64)),
                _ => {
                    handle_error(&format!("Invalid operation performed with '*'"), o.line);
                    Value::None
                }
            },

            TokenKind::SLASH => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => {
                    if n == 0 {
                        handle_error(&format!("Division by 0"), o.line);
                        Value::None
                    } else {
                        Value::Int(m / n)
                    }
                }
                (Value::Float(m), Value::Float(n)) => {
                    if n == 0.0 {
                        handle_error(&format!("Division by 0"), o.line);
                        Value::None
                    } else {
                        Value::Float(m / n)
                    }
                }
                (Value::Int(m), Value::Float(n)) => {
                    if n == 0.0 {
                        handle_error(&format!("Division by 0"), o.line);
                        Value::None
                    } else {
                        Value::Float((m as f64) / n)
                    }
                }
                (Value::Float(m), Value::Int(n)) => {
                    if n == 0 {
                        handle_error(&format!("Division by 0"), o.line);
                        Value::None
                    } else {
                        Value::Float(m / (n as f64))
                    }
                }
                _ => {
                    handle_error(&format!("Invalid operation performed with '/'"), o.line);
                    Value::None
                }
            },

            TokenKind::GREATER => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Value::Bool(m > n),
                (Value::Float(m), Value::Float(n)) => Value::Bool(m > n),
                (Value::Int(m), Value::Float(n)) => Value::Bool((m as f64) > n),
                (Value::Float(m), Value::Int(n)) => Value::Bool(m > (n as f64)),
                _ => {
                    handle_error(&format!("Invalid operation performed with '>'"), o.line);
                    Value::None
                }
            },

            TokenKind::LESS => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Value::Bool(m < n),
                (Value::Float(m), Value::Float(n)) => Value::Bool(m < n),
                (Value::Int(m), Value::Float(n)) => Value::Bool((m as f64) < n),
                (Value::Float(m), Value::Int(n)) => Value::Bool(m < (n as f64)),
                _ => {
                    handle_error(&format!("Invalid operation performed with '<'"), o.line);
                    Value::None
                }
            },

            TokenKind::GREATER_EQUAL => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Value::Bool(m >= n),
                (Value::Float(m), Value::Float(n)) => Value::Bool(m >= n),
                (Value::Int(m), Value::Float(n)) => Value::Bool((m as f64) >= n),
                (Value::Float(m), Value::Int(n)) => Value::Bool(m >= (n as f64)),
                _ => {
                    handle_error(&format!("Invalid operation performed with '>='"), o.line);
                    Value::None
                }
            },

            TokenKind::LESS_EQUAL => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Value::Bool(m <= n),
                (Value::Float(m), Value::Float(n)) => Value::Bool(m <= n),
                (Value::Int(m), Value::Float(n)) => Value::Bool((m as f64) <= n),
                (Value::Float(m), Value::Int(n)) => Value::Bool(m <= (n as f64)),
                _ => {
                    handle_error(&format!("Invalid operation performed with '<='"), o.line);
                    Value::None
                }
            },

            TokenKind::EQUAL_EQUAL => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Value::Bool(m == n),
                (Value::Float(m), Value::Float(n)) => Value::Bool(m == n),
                (Value::Int(m), Value::Float(n)) => Value::Bool((m as f64) == n),
                (Value::Float(m), Value::Int(n)) => Value::Bool(m == (n as f64)),
                (Value::Bool(m), Value::Bool(n)) => Value::Bool(m == n),
                _ => {
                    handle_error(&format!("Invalid operation performed with '=='"), o.line);
                    Value::None
                }
            },

            TokenKind::BANG_EQUAL => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Value::Bool(m != n),
                (Value::Float(m), Value::Float(n)) => Value::Bool(m != n),
                (Value::Int(m), Value::Float(n)) => Value::Bool((m as f64) != n),
                (Value::Float(m), Value::Int(n)) => Value::Bool(m != (n as f64)),
                (Value::Bool(m), Value::Bool(n)) => Value::Bool(m != n),
                _ => {
                    handle_error(&format!("Invalid operation performed with '!='"), o.line);
                    Value::None
                }
            },



            _ => Value::None,
        }
    }

    fn evaluate_unary(&mut self,  o: Token, r: Box<Expression>) -> Value {

        let r_ev: Value = self.evaluate(*r); 

        match o.kind{
            TokenKind::MINUS => {
                if let Value::Int(m) = r_ev { 
                    return Value::Int(- m);
                } else if let Value::Float(m) = r_ev {
                    return Value::Float(- m);
                }
                else {
                    handle_error(&format!("Invalid operation performed with '-'"), o.line); 
                }
            }

            TokenKind::BANG => {
                if let Value::Bool(m) = r_ev { 
                    return Value::Bool(!m);
                } 
                else {
                    handle_error(&format!("Invalid operation performed with '!'"), o.line); 
                }
            }
            _ => {}
        }

        return Value::None; 
    }

    fn evaluate_literal(&mut self, v: Value) -> Value{
        return v; 
    }

    fn evaluate_grouping(&mut self, exp: Box<Expression>) -> Value{
        self.evaluate(*exp)
    }
}

fn handle_error(msg: &str, line: i32) {

        eprintln!("[Line {}] Interpreter Error: {}", line, msg);
        process::exit(1);
    }
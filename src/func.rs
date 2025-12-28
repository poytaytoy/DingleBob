use crate::interpreter;
use crate::interpreter::Interpreter;
use crate::ast::Value; 
use crate::ast::Statement;
use crate::ast::Expression;
use crate::token::Token;
use std::env::args_os;
use std::io::LineWriter;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};


pub trait Func { 
    fn toString(&self) -> String; 
    fn expect(&self, args: Value, value_type: &str) -> Result<Value, String>{
        match value_type{
            "String" => {if matches!(args, Value::String(_)) {Ok(args)} else {Err(format!("Expected type {} but got {:?}", value_type, args))}},
            "Int" => {if matches!(args, Value::Int(_)) {Ok(args)} else {Err(format!("Expected type {} but got {:?}", value_type, args))}},
            "Float" => {
                if matches!(args, Value::Float(_)) || matches!(args, Value::Int(_)) { 
                    match args {
                        Value::Int(n) => {return Ok(Value::Float(n as f64))}, 
                        Value::Float(n) => {return Ok(Value::Float(n))},
                        _ => {unreachable!()}
                    }
                } 
                else {Err(format!("Expected type {} but got {:?}", value_type, args))}
            },
            "Bool" => {if matches!(args, Value::Bool(_)) {Ok(args)} else {Err(format!("Expected type {} but got {:?}", value_type, args))}},
            "None" => {if matches!(args, Value::None) {Ok(args)} else {Err(format!("Expected type {} but got {:?}", value_type, args))}},
            "Call" => {if matches!(args, Value::Call(_)) {Ok(args)} else {Err(format!("Expected type {} but got {:?}", value_type, args))}},
            _=>{unreachable!()}
        }
    }
    fn call(&self, interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, String>; 
}

pub struct Timeit; 

impl Func for Timeit {
    fn toString(&self ) -> String {
        return String::from("timeit")
    }

    fn call(&self, _interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, String> {

        if input_args.len() != 0 { 
            return Err(String::from("Argument size mismatch; Expected 0 argument(s)"));
        }

        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
            
        Ok(Value::Float(since_the_epoch.as_secs_f64()))
    }
}

pub struct Abs; 

impl Func for Abs { 

    fn toString(&self ) -> String {
        return String::from("abs")
    }

    fn call(&self, interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, String>{
        if input_args.len() != 1 { 
            return Err(String::from("Argument size mismatch; Expected 1 argument(s)"));
        }

        let mut input_num: f64; 

        match self.expect(input_args[0].clone(), "Float"){
            Ok(Value::Float(n)) => {input_num = n;},
            Err(e) => {return Err(e)},
            _=> {unreachable!()}
        }
        
        return Ok(Value::Float(input_num.abs()));
    }
}

pub struct Function{
    pub name: Token, 
    pub args_list: Vec<Token>, 
    pub statement_list: Vec<Statement>
}

impl Func for Function {

    fn toString(&self) -> String {
        return self.name.lexeme.clone();
    }
    
    fn call(&self, mut interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, String>  {
        if input_args.len() != self.args_list.len() { 
            return Err(String::from("Argument size mismatch; Expected 1 argument(s)"));
        }

        let mut var_list: Vec<Statement> = Vec::new(); 
        for n in 0..input_args.len(){
            var_list.push(Statement::Var((&self.args_list[n]).clone(), Expression::Literal(input_args[n].clone())))
        }

        var_list.push(Statement::Block(Box::new(self.statement_list.clone())));

        interpreter.interpret(var_list);
        
        return Ok(Value::None);
    }
    
}








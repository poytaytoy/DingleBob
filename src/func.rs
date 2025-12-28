use crate::interpreter::Interpreter;
use crate::ast::Value; 
use crate::ast::Statement;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};


pub trait Func { 
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
    fn call(&self, _interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, String> {

        if input_args.len() > 0 { 
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

    fn call(&self, interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, String>{
        if input_args.len() > 1 { 
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



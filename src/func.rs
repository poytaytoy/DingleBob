use crate::interpreter;
use crate::interpreter::Interpreter;
use crate::ast::Value; 
use crate::ast::Statement;
use crate::ast::Expression;
use crate::ast::BreakResult;
use crate::token::Token;
use std::cell::Ref;
use std::cell::RefCell;
use std::env::args_os;
use std::io::LineWriter;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};


pub trait Func { 
    fn toString(&self) -> String; 
    fn expect(&self, args: Value, value_type: &str) -> Result<Value, BreakResult> {
        let err = |got: Value| {
            Err(BreakResult::Error(format!(
                "Type error: expected {}, got {:?}.",
                value_type, got
            )))
        };

        match value_type {
            "String" => if matches!(args, Value::String(_)) { Ok(args) } else { err(args) },
            "Int" => if matches!(args, Value::Int(_)) { Ok(args) } else { err(args) },
            "Float" => {
                match args {
                    Value::Int(n) => Ok(Value::Float(n as f64)),
                    Value::Float(n) => Ok(Value::Float(n)),
                    other => err(other),
                }
            },
            "Bool" => if matches!(args, Value::Bool(_)) { Ok(args) } else { err(args) },
            "None" => if matches!(args, Value::None) { Ok(args) } else { err(args) },
            "Call" => if matches!(args, Value::Call(..)) { Ok(args) } else { err(args) },
            "List" => if matches!(args, Value::List(_)) { Ok(args) } else { err(args) },
            _ => unreachable!(),
        }
    }
    fn call(&self, interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, BreakResult>; 
}

pub struct Timeit; 

impl Func for Timeit {
    fn toString(&self ) -> String {
        return String::from("timeit")
    }

    fn call(&self, _interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, BreakResult> {

        if input_args.len() != 0 { 
            return Err(BreakResult::Error(format!(
                "Arity error: 'timeit' takes 0 arguments, but got {}.",
                input_args.len()
            )));
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

    fn call(&self, interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, BreakResult>{
        if input_args.len() != 1 { 
            return Err(BreakResult::Error(format!(
                "Arity error: 'abs' takes 1 argument, but got {}.",
                input_args.len()
            )));
        }

        let Value::Float(input_num) = self.expect(input_args[0].clone(), "Float")? else {unreachable!()}; 
        
        return Ok(Value::Float(input_num.abs()));
    }
}

pub struct Len; 

impl Func for Len { 

    fn toString(&self ) -> String {
        return String::from("len")
    }

    fn call(&self, interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, BreakResult>{
        if input_args.len() != 1 { 
            return Err(BreakResult::Error(format!(
                "Arity error: 'len' takes 1 argument, but got {}.",
                input_args.len()
            )));
        }

        let Value::List(lst) = self.expect(input_args[0].clone(), "List")? else {unreachable!()};

        return Ok(Value::Int(lst.borrow().len() as i128));
    }
}

pub struct Copy; 

impl Func for Copy { 

    fn toString(&self ) -> String {
        return String::from("copy")
    }

    fn call(&self, interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, BreakResult>{
        if input_args.len() != 1 { 
            return Err(BreakResult::Error(format!(
                "Arity error: 'copy' takes 1 argument, but got {}.",
                input_args.len()
            )));
        }

        let Value::List(lst) = self.expect(input_args[0].clone(), "List")? else {unreachable!()};

        return Ok(Value::List(Rc::new(RefCell::new(lst.borrow().clone()))));
    }
}

pub struct Append; 

impl Func for Append { 

    fn toString(&self) -> String {
        return String::from("append")
    }

    fn call(&self, interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, BreakResult>{
        if input_args.len() != 2 { 
            return Err(BreakResult::Error(format!(
                "Arity error: 'append' takes 2 arguments (list, value), but got {}.",
                input_args.len()
            )));
        }

        let Value::List(lst) = self.expect(input_args[0].clone(), "List")? else {unreachable!()};
        let val = input_args[1].clone();

        lst.borrow_mut().push(val);

        return Ok(Value::List(Rc::clone(&lst)));
    }
}

pub struct Concat; 

impl Func for Concat { 

    fn toString(&self) -> String {
        return String::from("concat")
    }

    fn call(&self, interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, BreakResult>{
        if input_args.len() != 2 { 
            return Err(BreakResult::Error(format!(
                "Arity error: 'concat' takes 2 arguments (list, list), but got {}.",
                input_args.len()
            )));
        }

        let Value::List(lst1) = self.expect(input_args[0].clone(), "List")? else {unreachable!()};
        let Value::List(lst2) = self.expect(input_args[1].clone(), "List")? else {unreachable!()};

        let mut concat_lst = lst1.borrow().clone();
        concat_lst.append(&mut lst2.borrow().clone());

        return Ok(Value::List(Rc::new(RefCell::new(concat_lst))));
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

    fn call(&self, mut interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, BreakResult>  {
        if input_args.len() != self.args_list.len() { 
            return Err(BreakResult::Error(format!(
                "Arity error: function '{}' expects {} argument(s), but got {}.",
                self.name.lexeme,
                self.args_list.len(),
                input_args.len()
            )));
        }

        let mut var_list: Vec<Statement> = Vec::new(); 
        for n in 0..input_args.len(){
            var_list.push(Statement::Var((&self.args_list[n]).clone(), Expression::Literal(input_args[n].clone())))
        }

        var_list.push(Statement::Block(Box::new(self.statement_list.clone())));

        match interpreter.interpret(vec![Statement::Block(Box::new(var_list))]) {
            Ok(_) => {return Ok(Value::None);},
            Err(BreakResult::Return(_t,v )) => {return Ok(v)},
            Err(br) => {return Err(br)}
        }
    }
    
}

pub struct Lambda{
    pub args_list: Vec<Token>, 
    pub statement_list: Vec<Statement>
}

impl Func for Lambda {

    fn toString(&self) -> String {
        let mut text = String::from("Lambda(");

        for i in 0..self.args_list.len(){
            
            text += &self.args_list[i].lexeme ; 

            if i != self.args_list.len() -1 {
                text += ",";
            }
        }

        text += ")";

        return text; 
    }

    fn call(&self, mut interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, BreakResult>  {
        if input_args.len() != self.args_list.len() { 
            return Err(BreakResult::Error(format!(
                "Arity error: lambda expects {} argument(s), but got {}.",
                self.args_list.len(),
                input_args.len()
            )));
        }

        let mut var_list: Vec<Statement> = Vec::new(); 
        for n in 0..input_args.len(){
            var_list.push(Statement::Var((&self.args_list[n]).clone(), Expression::Literal(input_args[n].clone())))
        }

        var_list.push(Statement::Block(Box::new(self.statement_list.clone())));

        match interpreter.interpret(vec![Statement::Block(Box::new(var_list))]) {
            Ok(_) => {return Ok(Value::None);},
            Err(BreakResult::Return(_t,v )) => {return Ok(v)},
            Err(br) => {return Err(br)}
        }
    }
    
}

use crate::ast::BreakResult;
use crate::ast::Expression;
use crate::ast::Statement;
use crate::ast::Value;
use crate::environment::Environment;
use crate::interpreter;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::resolver::Resolver;
use crate::scanner::*;
use crate::token::Token;
use std::cell::Ref;
use std::cell::RefCell;
use std::fs;
use std::io::LineWriter;
use std::process::Command;

use std::env;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub trait Func {
    fn isDefault(&self) -> bool;
    fn toString(&self) -> String;
    fn expect(&self, args: Value, value_type: &str) -> Result<Value, BreakResult> {
        let err = |got: Value| {
            Err(BreakResult::Error(format!(
                "Type error: expected {}, got {:?}.",
                value_type, got
            )))
        };

        match value_type {
            "String" => {
                if matches!(args, Value::String(_)) {
                    Ok(args)
                } else {
                    err(args)
                }
            }
            "Int" => {
                if matches!(args, Value::Int(_)) {
                    Ok(args)
                } else {
                    err(args)
                }
            }
            "Float" => match args {
                Value::Int(n) => Ok(Value::Float(n as f64)),
                Value::Float(n) => Ok(Value::Float(n)),
                other => err(other),
            },
            "Bool" => {
                if matches!(args, Value::Bool(_)) {
                    Ok(args)
                } else {
                    err(args)
                }
            }
            "None" => {
                if matches!(args, Value::None) {
                    Ok(args)
                } else {
                    err(args)
                }
            }
            "Call" => {
                if matches!(args, Value::Call(..)) {
                    Ok(args)
                } else {
                    err(args)
                }
            }
            "List" => {
                if matches!(args, Value::List(_)) {
                    Ok(args)
                } else {
                    err(args)
                }
            }
            _ => unreachable!(),
        }
    }
    fn call(&self, interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, BreakResult>;
}

pub struct Timeit;

impl Func for Timeit {
    fn isDefault(&self) -> bool {
        true
    }

    fn toString(&self) -> String {
        return String::from("timeit");
    }

    fn call(
        &self,
        _interpreter: Interpreter,
        input_args: Vec<Value>,
    ) -> Result<Value, BreakResult> {
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
    fn isDefault(&self) -> bool {
        true
    }

    fn toString(&self) -> String {
        return String::from("abs");
    }

    fn call(&self, interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, BreakResult> {
        if input_args.len() != 1 {
            return Err(BreakResult::Error(format!(
                "Arity error: 'abs' takes 1 argument, but got {}.",
                input_args.len()
            )));
        }

        let Value::Float(input_num) = self.expect(input_args[0].clone(), "Float")? else {
            unreachable!()
        };

        return Ok(Value::Float(input_num.abs()));
    }
}

pub struct Len;

impl Func for Len {
    fn isDefault(&self) -> bool {
        true
    }

    fn toString(&self) -> String {
        return String::from("len");
    }

    fn call(&self, interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, BreakResult> {
        if input_args.len() != 1 {
            return Err(BreakResult::Error(format!(
                "Arity error: 'len' takes 1 argument, but got {}.",
                input_args.len()
            )));
        }

        let Value::List(lst) = self.expect(input_args[0].clone(), "List")? else {
            unreachable!()
        };

        return Ok(Value::Int(lst.borrow().len() as i128));
    }
}

pub struct Copy;

impl Func for Copy {
    fn isDefault(&self) -> bool {
        true
    }

    fn toString(&self) -> String {
        return String::from("copy");
    }

    fn call(&self, interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, BreakResult> {
        if input_args.len() != 1 {
            return Err(BreakResult::Error(format!(
                "Arity error: 'copy' takes 1 argument, but got {}.",
                input_args.len()
            )));
        }

        let Value::List(lst) = self.expect(input_args[0].clone(), "List")? else {
            unreachable!()
        };

        return Ok(Value::List(Rc::new(RefCell::new(lst.borrow().clone()))));
    }
}

pub struct Append;

impl Func for Append {
    fn isDefault(&self) -> bool {
        true
    }

    fn toString(&self) -> String {
        return String::from("append");
    }

    fn call(&self, interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, BreakResult> {
        if input_args.len() != 2 {
            return Err(BreakResult::Error(format!(
                "Arity error: 'append' takes 2 arguments (list, value), but got {}.",
                input_args.len()
            )));
        }

        let Value::List(lst) = self.expect(input_args[0].clone(), "List")? else {
            unreachable!()
        };
        let val = input_args[1].clone();

        lst.borrow_mut().push(val);

        return Ok(Value::List(Rc::clone(&lst)));
    }
}

pub struct Concat;

impl Func for Concat {
    fn isDefault(&self) -> bool {
        true
    }

    fn toString(&self) -> String {
        return String::from("concat");
    }

    fn call(&self, interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, BreakResult> {
        if input_args.len() != 2 {
            return Err(BreakResult::Error(format!(
                "Arity error: 'concat' takes 2 arguments (list, list), but got {}.",
                input_args.len()
            )));
        }

        let Value::List(lst1) = self.expect(input_args[0].clone(), "List")? else {
            unreachable!()
        };
        let Value::List(lst2) = self.expect(input_args[1].clone(), "List")? else {
            unreachable!()
        };

        let mut concat_lst = lst1.borrow().clone();
        concat_lst.append(&mut lst2.borrow().clone());

        return Ok(Value::List(Rc::new(RefCell::new(concat_lst))));
    }
}

pub struct Import;

impl Func for Import {
    fn isDefault(&self) -> bool {
        false
    }

    fn toString(&self) -> String {
        return String::from("import");
    }

    fn call(&self, interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, BreakResult> {
        let mut resolver_exe = Resolver::new(false);
        let mut interpreter_exe = Interpreter::new(true, resolver_exe.give_local(), false);

        if input_args.len() != 1 {
            return Err(BreakResult::Error(format!(
                "Arity error: 'import' takes 1 arguments String, but got {}.",
                input_args.len()
            )));
        }

        let Value::String(path) = self.expect(input_args[0].clone(), "String")? else {
            unreachable!()
        };

        let contents = fs::read_to_string(&path).unwrap_or_else(|_| {
            eprintln!("Could not read file '{}'", path);
            std::process::exit(1);
        });

        let token_result = scan(&contents, false, path, false);

        if let Err(msg) = token_result {
            return Err(BreakResult::Error(format!("")));
        }

        let mut parser = Parser::new(token_result.unwrap(), false);
        let parsed_result: Result<Vec<Statement>, String> = parser.parse();

        if let Err(msg) = parsed_result {
            return Err(BreakResult::Error(format!("")));
        }

        let resolver_result = resolver_exe.resolve((&parsed_result).clone().unwrap());

        if let Err(msg) = resolver_result {
            return Err(BreakResult::Error(format!("")));
        }

        let interpreter_result = interpreter_exe.prime_interpret(parsed_result.unwrap());

        if let Err(msg) = interpreter_result {
            return Err(BreakResult::Error(format!("")));
        }

        for (k, v) in interpreter_exe
            .global_environment
            .borrow()
            .hashMap
            .clone()
            .into_iter()
        {
            if !k.starts_with('_') {
                interpreter
                    .global_environment
                    .borrow_mut()
                    .define_from_execute(k, v);
            }
        }

        for (k, v) in interpreter_exe.locals.borrow().clone().into_iter() {
            interpreter.locals.borrow_mut().insert(k, v);
        }

        return Ok(Value::None);
    }
}

pub struct Function {
    pub name: Token,
    pub args_list: Vec<Token>,
    pub statement_list: Vec<Statement>,
}

impl Func for Function {
    fn isDefault(&self) -> bool {
        false
    }

    fn toString(&self) -> String {
        return self.name.lexeme.clone();
    }

    fn call(
        &self,
        mut interpreter: Interpreter,
        input_args: Vec<Value>,
    ) -> Result<Value, BreakResult> {
        if input_args.len() != self.args_list.len() {
            return Err(BreakResult::Error(format!(
                "Arity error: function '{}' expects {} argument(s), but got {}.",
                self.name.lexeme,
                self.args_list.len(),
                input_args.len()
            )));
        }

        let mut var_list: Vec<Statement> = Vec::new();
        for n in 0..input_args.len() {
            var_list.push(Statement::Var(
                (&self.args_list[n]).clone(),
                Expression::Literal(input_args[n].clone()),
            ))
        }

        var_list.push(Statement::Block(Box::new(self.statement_list.clone())));

        match interpreter.interpret(vec![Statement::Block(Box::new(var_list))]) {
            Ok(_) => {
                return Ok(Value::None);
            }
            Err(BreakResult::Return(_t, v)) => return Ok(v),
            Err(br) => return Err(br),
        }
    }
}

pub struct Lambda {
    pub args_list: Vec<Token>,
    pub statement_list: Vec<Statement>,
}

impl Func for Lambda {
    fn isDefault(&self) -> bool {
        false
    }

    fn toString(&self) -> String {
        let mut text = String::from("Lambda(");

        for i in 0..self.args_list.len() {
            text += &self.args_list[i].lexeme;

            if i != self.args_list.len() - 1 {
                text += ",";
            }
        }

        text += ")";

        return text;
    }

    fn call(
        &self,
        mut interpreter: Interpreter,
        input_args: Vec<Value>,
    ) -> Result<Value, BreakResult> {
        if input_args.len() != self.args_list.len() {
            return Err(BreakResult::Error(format!(
                "Arity error: lambda expects {} argument(s), but got {}.",
                self.args_list.len(),
                input_args.len()
            )));
        }

        let mut var_list: Vec<Statement> = Vec::new();
        for n in 0..input_args.len() {
            var_list.push(Statement::Var(
                (&self.args_list[n]).clone(),
                Expression::Literal(input_args[n].clone()),
            ))
        }

        var_list.push(Statement::Block(Box::new(self.statement_list.clone())));

        match interpreter.interpret(vec![Statement::Block(Box::new(var_list))]) {
            Ok(_) => {
                return Ok(Value::None);
            }
            Err(BreakResult::Return(_t, v)) => return Ok(v),
            Err(br) => return Err(br),
        }
    }
}

pub struct Struct {
    pub name: Token,
    pub fields: Vec<Token>,
}

impl Func for Struct {
    fn isDefault(&self) -> bool {
        false
    }
    fn toString(&self) -> String {
        self.name.lexeme.clone()
    }
    fn call(&self, interpreter: Interpreter, input_args: Vec<Value>) -> Result<Value, BreakResult> {
        if input_args.len() != self.fields.len() {
            return Err(BreakResult::Error(format!(
                "Arity error: Struct '{}' expects {} argument(s), but got {}.",
                self.name.lexeme,
                self.fields.len(),
                input_args.len()
            )));
        }

        let mut instance_env = Environment::new(None, interpreter.repl);

        for (i, token) in self.fields.iter().enumerate() {
            instance_env.define(token.clone(), input_args[i].clone());
        }

        Ok(Value::Instance(Rc::new(RefCell::new(instance_env))))
    }
}

pub struct Read;

impl Func for Read {
    fn isDefault(&self) -> bool {
        true
    }
    fn toString(&self) -> String {
        String::from("read")
    }
    fn call(
        &self,
        _interpreter: Interpreter,
        input_args: Vec<Value>,
    ) -> Result<Value, BreakResult> {
        if input_args.len() != 1 {
            return Err(BreakResult::Error(format!(
                "Arity error: 'read' takes 1 argument, got {}.",
                input_args.len()
            )));
        }
        let Value::String(path) = self.expect(input_args[0].clone(), "String")? else {
            unreachable!()
        };

        match fs::read_to_string(&path) {
            Ok(content) => Ok(Value::String(content)),
            Err(e) => Err(BreakResult::Error(format!(
                "IO Error: could not read file '{}': {}",
                path, e
            ))),
        }
    }
}

pub struct Write;

impl Func for Write {
    fn isDefault(&self) -> bool {
        true
    }
    fn toString(&self) -> String {
        String::from("write")
    }
    fn call(
        &self,
        _interpreter: Interpreter,
        input_args: Vec<Value>,
    ) -> Result<Value, BreakResult> {
        if input_args.len() != 2 {
            return Err(BreakResult::Error(format!(
                "Arity error: 'write' takes 2 arguments, got {}.",
                input_args.len()
            )));
        }
        let Value::String(path) = self.expect(input_args[0].clone(), "String")? else {
            unreachable!()
        };
        let Value::String(content) = self.expect(input_args[1].clone(), "String")? else {
            unreachable!()
        };

        match fs::write(&path, content) {
            Ok(_) => Ok(Value::None),
            Err(e) => Err(BreakResult::Error(format!(
                "IO Error: could not write to file '{}': {}",
                path, e
            ))),
        }
    }
}

pub struct Getenv;

impl Func for Getenv {
    fn isDefault(&self) -> bool {
        true
    }
    fn toString(&self) -> String {
        String::from("getenv")
    }
    fn call(
        &self,
        _interpreter: Interpreter,
        input_args: Vec<Value>,
    ) -> Result<Value, BreakResult> {
        if input_args.len() != 1 {
            return Err(BreakResult::Error(format!(
                "Arity error: 'getenv' takes 1 argument, got {}.",
                input_args.len()
            )));
        }
        let Value::String(key) = self.expect(input_args[0].clone(), "String")? else {
            unreachable!()
        };

        match env::var(&key) {
            Ok(val) => Ok(Value::String(val)),
            Err(_) => Ok(Value::None),
        }
    }
}

pub struct Sleep;

impl Func for Sleep {
    fn isDefault(&self) -> bool {
        true
    }
    fn toString(&self) -> String {
        String::from("sleep")
    }
    fn call(
        &self,
        _interpreter: Interpreter,
        input_args: Vec<Value>,
    ) -> Result<Value, BreakResult> {
        if input_args.len() != 1 {
            return Err(BreakResult::Error(format!(
                "Arity error: 'sleep' takes 1 argument (ms), got {}.",
                input_args.len()
            )));
        }

        let ms = match input_args[0] {
            Value::Int(n) => n as u64,
            Value::Float(f) => f as u64,
            _ => {
                return Err(BreakResult::Error(format!(
                    "Type error: 'sleep' expects Int or Float (ms), got {:?}.",
                    input_args[0]
                )))
            }
        };

        thread::sleep(Duration::from_millis(ms));
        Ok(Value::None)
    }
}

use crate::ast::Expression;
use crate::ast::Value;
use crate::ast::Statement; 
use crate::ast::BreakResult;
use crate::func;
use crate::repl;
use crate::token::TokenKind; 
use crate::token::Token; 
use crate::environment::Environment; 
use core::borrow;
use std::cell::Ref;
use std::clone;
use std::collections::HashMap;
use std::env::args;
use std::hash::Hash;
use std::process; 
use std::rc::Rc; 
use std::cell::RefCell; 
use crate::func::*; 
use ariadne::{Color, ColorGenerator, Label, Report, ReportKind, Source};
use std::fs;

pub struct Interpreter {
    pub global_environment: Rc<RefCell<Environment>>,
    pub is_prime: bool, 
    pub locals: Rc<RefCell<HashMap<Token, i32>>>,
    pub repl: bool
}

impl Interpreter {

    pub fn clone(&mut self, locals: Rc<RefCell<HashMap<Token, i32>>> ) -> Self {
        Interpreter {
            global_environment: Rc::new(RefCell::new(self.global_environment.borrow().clone())),
            is_prime: self.is_prime, 
            locals: locals,
            repl: self.repl
        }
    }

    pub fn new(is_prime: bool, locals: Rc<RefCell<HashMap<Token, i32>>>, repl: bool ) -> Self {
        let mut environment = Rc::new(RefCell::new(Environment::new(None, repl))); 
        let mut edittable_env = environment.borrow_mut();

        // Built-in functions 
        let mut define = |name: &str, func: Box<dyn Func>| {
            edittable_env.define_default(
                name, 
                Value::Call(Rc::from(func), Rc::clone(&environment))
            )
        };

        define("timeit", Box::new(Timeit {}));
        define("abs", Box::new(Abs {}));
        define("len", Box::new(Len {}));
        define("copy", Box::new(Copy {}));
        define("append", Box::new(Append {}));
        define("concat", Box::new(Concat {}));
        define("import", Box::new(Import {}));

        Interpreter {
            global_environment: Rc::clone(&environment),
            is_prime: true,
            locals: locals,
            repl: repl
        }
    }

    /// Helper to get a human-readable string of a Value's type
    fn get_type_name(&self, val: &Value) -> String {
        match val {
            Value::Int(_) => "Int".to_string(),
            Value::Float(_) => "Float".to_string(),
            Value::Bool(_) => "Bool".to_string(),
            Value::String(_) => "String".to_string(),
            Value::None => "None".to_string(),
            Value::List(_) => "List".to_string(),
            Value::Call(_, _) => "Function".to_string(),
        }
    }

    pub fn prime_interpret(&mut self, statements: Vec<Statement>) -> Result<(), String> {
        let interpret_result = self.interpret(statements);
        match interpret_result {
            Err(BreakResult::Error(msg)) => return Err(format!("{}", msg)),
            Err(BreakResult::Return(t, _val)) => {
                self.handle_error(
                    "'return' can only be used inside a function body.",
                    t
                );

                return Err(String::from("Interpreter Error: 'return' can only be used inside a function body."));
            },
            Err(BreakResult::Break(t)) => {
                self.handle_error(
                    "'break' can only be used inside a loop body.",
                    t
                );
                
                return Err(String::from("Interpretter Error: 'break' can only be used inside a loop body."));
            },
            _ => {}
        }
        Ok(())
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<Value, BreakResult> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        Ok(Value::None)
    }

    fn execute(&mut self, stmt: Statement) -> Result<Value, BreakResult> {
        match stmt {
            Statement::Expression(exp) => self.evaluate(exp),
            Statement::If(exp, then_s, else_s) => self.execute_if(exp, *then_s, *else_s),
            Statement::Function(t, vt, vs) => self.execute_function(t, vt, *vs),
            Statement::Print(exp) => self.execute_print(exp),     
            Statement::Return(t, val) => self.execute_return(t, val),
            Statement::Var(var, value) => self.execute_var(var, value),
            Statement::Block(statements) => self.execute_block(*statements),
            Statement::While(exp, s) => self.execute_while(exp, *s),
            Statement::Break(t) => self.execute_break(t),
            _ => unreachable!()
        }
    }

    fn to_bool(&self, val: &Value) -> bool {
        match *val { 
            Value::Bool(t) => t,
            Value::Int(n) => n != 0,
            Value::Float(n) => n != 0.0,
            Value::None => false,
            _ => true
        }
    }

    fn execute_if(&mut self, exp: Expression, then_s: Statement, else_s: Statement) -> Result<Value, BreakResult> {
        let val = self.evaluate(exp)?;
        if self.to_bool(&val) {
            self.execute(then_s)
        } else {
            self.execute(else_s)
        }
    }

    fn execute_function(&mut self, t: Token, vt: Vec<Token>, vs: Vec<Statement>) -> Result<Value, BreakResult> {
        let t_clone = t.clone();
        let function_call = Function {
            name: t, args_list: vt, statement_list: vs
        };
        self.global_environment.borrow_mut().define(t_clone, Value::Call(Rc::new(function_call), Rc::clone(&self.global_environment)))
    }

    fn execute_print(&mut self, expression: Expression) -> Result<Value, BreakResult> {
        let value = self.evaluate(expression)?; 
        match value { 
            Value::Int(m) => println!("{}", m),
            Value::Float(m) => println!("{}", m),
            Value::Bool(m) => println!("{}", m),
            Value::None => println!("none"),
            Value::String(m) => println!("{}", m),
            Value::Call(callee, _) => println!("<fn {}>", callee.toString()),
            Value::List(vec) => println!("{:?}", vec.borrow_mut())
        }
        Ok(Value::None)
    }

    fn execute_return(&mut self, t: Token, val: Expression) -> Result<Value, BreakResult> {
        let val_ev = self.evaluate(val)?;
        Err(BreakResult::Return(t, val_ev))
    }

    fn execute_var(&mut self, var: Token, value: Expression) -> Result<Value, BreakResult> { 
        let evaluated_var = self.evaluate(value)?; 
        self.global_environment.borrow_mut().define(var, evaluated_var)
    }

    fn execute_block(&mut self, statements: Vec<Statement>) -> Result<Value, BreakResult> {
        let curr_env = Rc::clone(&self.global_environment); 
        self.global_environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&self.global_environment)), self.repl)));
        
        for statement in statements {
            match self.execute(statement) {
                Ok(_) => (),
                Err(e) => {
                    self.global_environment = curr_env;
                    return Err(e);
                }
            }
        }
        
        self.global_environment = curr_env;
        Ok(Value::None)
    }

    fn execute_while(&mut self, exp: Expression, s: Statement) -> Result<Value, BreakResult> {

        let mut result = self.evaluate(exp.clone())?;
        while self.to_bool(&result) {
            match self.execute(s.clone()) {
                Err(BreakResult::Break(_t)) => break,
                Err(br) => return Err(br),
                _ => {result = self.evaluate(exp.clone())?},
            }
        }
        Ok(Value::None)
    }

    fn execute_break(&mut self, t: Token) -> Result<Value, BreakResult> { 
        Err(BreakResult::Break(t))
    }

    fn evaluate(&mut self, expression: Expression) -> Result<Value, BreakResult> {
        match expression { 
            Expression::Assign(i, eq, a) => self.evaluate_assign(*i, eq, *a),
            Expression::Binary(l, o, r) => self.evaluate_binary(l, o, r), 
            Expression::Unary(o, r) => self.evaluate_unary(o, r),
            Expression::Call(callee, paren, args) => self.evaluate_call(*callee, paren, *args),
            Expression::Logical(l, o, r) => self.evaluate_logical(*l, o, *r),
            Expression::Literal(v) => Ok(v),
            Expression::Grouping(exp) => self.evaluate( *exp), 
            Expression::Variable(t) => self.evaluate_variable(t),
            Expression::Lambda(args, stmt ) => self.evaluate_lambda(args, *stmt),
            Expression::Index(ls, rb, i) => self.evaluate_index(*ls, rb, *i),
            Expression::List(content, t) => self.evaluate_list(*content, t)
        }
    }
    
    fn evaluate_assign(&mut self, i: Expression, eq: Token, a: Expression) -> Result<Value, BreakResult> {
        let a_ev = self.evaluate(a)?; 

        if let Expression::Variable(t) = i {
            self.global_environment.borrow_mut().assign(t, a_ev);
            return Ok(Value::None); 
        }

        if let Expression::Index(l, t, i) = i {
            let l_ev = self.evaluate(*l)?;
            let i_ev = self.evaluate(*i)?;

            let Value::List(ls) = l_ev else {
                return Err(self.handle_error(
                    &format!("Invalid assignment: expected a List for indexing, but got {}.", self.get_type_name(&l_ev)),
                    t
                ));
            };

            let Value::Int(index) = i_ev else {
                return Err(self.handle_error(
                    &format!("Invalid list index: indices must be Int, but got {}.", self.get_type_name(&i_ev)),
                    t
                ));
            };

            if index < 0 || index >= (ls.borrow().len() as i128) {
                return Err(self.handle_error(
                    &format!("Index out of bounds: index {} is not in [0, {}).", index, ls.borrow().len()),
                    t
                ));
            }

            ls.borrow_mut()[index as usize] = a_ev; 
            return Ok(Value::None); 
        };

        Err(self.handle_error(
            "Invalid assignment target: expected a variable or list index.",
            eq
        ))
    }

    fn evaluate_binary(&mut self, l: Box<Expression>, o: Token, r: Box<Expression>) -> Result<Value, BreakResult> {
        let l_ev = self.evaluate(*l)?;
        let r_ev = self.evaluate(*r)?;

        match o.kind {
            TokenKind::PLUS => match (l_ev.clone(), r_ev.clone()) {
                (Value::Int(m), Value::Int(n)) => Ok(Value::Int(m + n)),
                (Value::Float(m), Value::Float(n)) => Ok(Value::Float(m + n)),
                (Value::Int(m), Value::Float(n)) => Ok(Value::Float((m as f64) + n)),
                (Value::Float(m), Value::Int(n)) => Ok(Value::Float(m + (n as f64))),
                (Value::String(m), Value::Int(n)) => Ok(Value::String(m + &n.to_string())),
                (Value::String(m), Value::Float(n)) => Ok(Value::String(m + &n.to_string())),
                (Value::String(m), Value::String(n)) => Ok(Value::String(m + &n)),
                _ => Err(self.handle_error(
                    &format!("Type error: '+' expects numbers or strings, but got {} and {}.", 
                    self.get_type_name(&l_ev), self.get_type_name(&r_ev)), 
                    o
                )),
            },

            TokenKind::MINUS | TokenKind::STAR | TokenKind::SLASH | TokenKind::PERCENT => {
                match (l_ev.clone(), r_ev.clone()) {
                    (Value::Int(m), Value::Int(n)) => {
                        if o.kind == TokenKind::SLASH && n == 0 { return Err(self.handle_error("Division by zero.", o)); }
                        let res = match o.kind {
                            TokenKind::MINUS => m - n,
                            TokenKind::STAR => m * n,
                            TokenKind::SLASH => m / n,
                            _ => m % n,
                        };
                        Ok(Value::Int(res))
                    }
                    (Value::Float(m), Value::Float(n)) => {
                        if o.kind == TokenKind::SLASH && n == 0.0 { return Err(self.handle_error("Division by zero.", o)); }
                        let res = match o.kind {
                            TokenKind::MINUS => m - n,
                            TokenKind::STAR => m * n,
                            TokenKind::SLASH => m / n,
                            _ => m % n,
                        };
                        Ok(Value::Float(res))
                    }
                    (Value::Int(m), Value::Float(n)) | (Value::Float(n), Value::Int(m)) => {
                        let m_f = m as f64;
                        if o.kind == TokenKind::SLASH && n == 0.0 { return Err(self.handle_error("Division by zero.", o)); }
                        let res = match o.kind {
                            TokenKind::MINUS => if matches!(l_ev, Value::Int(_)) { m_f - n } else { n - m_f },
                            TokenKind::STAR => m_f * n,
                            TokenKind::SLASH => if matches!(l_ev, Value::Int(_)) { m_f / n } else { n / m_f },
                            _ => if matches!(l_ev, Value::Int(_)) { m_f % n } else { n % m_f },
                        };
                        Ok(Value::Float(res))
                    }
                    _ => Err(self.handle_error(
                        &format!("Type error: '{}' expects numeric operands, but got {} and {}.", 
                        o.lexeme, self.get_type_name(&l_ev), self.get_type_name(&r_ev)), 
                        o
                    )),
                }
            },

            TokenKind::GREATER | TokenKind::GREATER_EQUAL | TokenKind::LESS | TokenKind::LESS_EQUAL => {
                match (l_ev.clone(), r_ev.clone()) {
                    (Value::Int(m), Value::Int(n)) => {
                        let res = match o.kind {
                            TokenKind::GREATER => m > n,
                            TokenKind::GREATER_EQUAL => m >= n,
                            TokenKind::LESS => m < n,
                            _ => m <= n,
                        };
                        Ok(Value::Bool(res))
                    }
                    (Value::Float(m), Value::Float(n)) => {
                        let res = match o.kind {
                            TokenKind::GREATER => m > n,
                            TokenKind::GREATER_EQUAL => m >= n,
                            TokenKind::LESS => m < n,
                            _ => m <= n,
                        };
                        Ok(Value::Bool(res))
                    }
                    _ => Err(self.handle_error(
                        &format!("Type error: Comparison '{}' expects numeric operands of the same type, but got {} and {}.", 
                        o.lexeme, self.get_type_name(&l_ev), self.get_type_name(&r_ev)), 
                        o
                    )),
                }
            }

            TokenKind::EQUAL_EQUAL | TokenKind::BANG_EQUAL => {
                let is_eq = match (l_ev, r_ev) {
                    (Value::Int(m), Value::Int(n)) => m == n,
                    (Value::Float(m), Value::Float(n)) => m == n,
                    (Value::Bool(m), Value::Bool(n)) => m == n,
                    (Value::String(m), Value::String(n)) => m == n,
                    (Value::None, Value::None) => true,
                    _ => false,
                };
                let result = if o.kind == TokenKind::EQUAL_EQUAL { is_eq } else { !is_eq };
                Ok(Value::Bool(result))
            }
            _ => Err(self.handle_error("Internal error: unknown binary operator.", o)),
        }
    }

    fn evaluate_unary(&mut self, o: Token, r: Box<Expression>) -> Result<Value, BreakResult> {
        let r_ev = self.evaluate(*r)?; 
        match o.kind {
            TokenKind::MINUS => match r_ev {
                Value::Int(m) => Ok(Value::Int(-m)),
                Value::Float(m) => Ok(Value::Float(-m)),
                _ => Err(self.handle_error(
                    &format!("Type error: unary '-' expects a number, but got {}.", self.get_type_name(&r_ev)), 
                    o
                )),
            }
            TokenKind::BANG => match r_ev {
                Value::Bool(m) => Ok(Value::Bool(!m)),
                _ => Err(self.handle_error(
                    &format!("Type error: '!' expects a boolean, but got {}.", self.get_type_name(&r_ev)), 
                    o
                )),
            }
            _ => Ok(Value::None)
        }
    }

    fn evaluate_call(&mut self, callee: Expression, paren: Token, args: Vec<Expression>) -> Result<Value, BreakResult> {
        let callee_ev = self.evaluate(callee)?; 
        let mut processed_args = Vec::new();

        for arg in args {
            processed_args.push(self.evaluate(arg)?);
        }

        match callee_ev {
            Value::Call(call, env) => {
                let result = call.call(
                    Interpreter { 
                        global_environment: Rc::clone(&env), 
                        is_prime: false, 
                        locals: Rc::clone(&self.locals),
                        repl: self.repl
                    }, 
                    processed_args
                );
                
                match result {
                    Ok(v) => Ok(v),
                    Err(BreakResult::Error(e)) => {
                        
                        if call.isDefault() || self.repl {
                            return Err(self.handle_error(
                            &format!("Error inside function call '{}': {}", call.toString(), &e),
                            paren));
                        } else {
                            return Err(self.handle_error(
                            &format!("Error inside function call '{}'", call.toString()),
                            paren));
                        }
                        
                        
                    },
                    Err(a) => Err(a)
                }
            },
            _ => Err(self.handle_error(
                &format!("Type error: expected a function to call, but got {}.", self.get_type_name(&callee_ev)),
                paren
            ))
        }
    }
    
    fn evaluate_logical(&mut self, l: Expression, o: Token, r: Expression) -> Result<Value, BreakResult> {
        let l_ev = self.evaluate(l)?;
        if o.kind == TokenKind::OR {
            if self.to_bool(&l_ev) { return Ok(l_ev); }
        } else {
            if !self.to_bool(&l_ev) { return Ok(l_ev); }
        }
        self.evaluate(r)
    }

    fn evaluate_variable(&mut self, token: Token) -> Result<Value, BreakResult> {
        let binding = self.locals.borrow();
        let Some(steps) = binding.get(&token) else {
            return self.global_environment.borrow_mut().get(token);
        };
        self.global_environment.borrow_mut().get_at(token, *steps)
    }

    fn evaluate_lambda(&mut self, args_list: Vec<Token>, bdy: Vec<Statement>) -> Result<Value, BreakResult> {
        let function_call = Lambda { args_list, statement_list: bdy };
        Ok(Value::Call(Rc::new(function_call), Rc::clone(&self.global_environment)))
    }
    
    fn evaluate_index(&mut self, l: Expression, t: Token, i: Expression) -> Result<Value, BreakResult> {
        let l_ev = self.evaluate(l)?;
        let i_ev = self.evaluate(i)?;

        let Value::List(ls) = l_ev else {
            return Err(self.handle_error(
                &format!("Type error: indexing ('[...]') expects a List, but got {}.", self.get_type_name(&l_ev)),
                t
            ));
        };

        let Value::Int(index) = i_ev else {
            return Err(self.handle_error(
                &format!("Type error: list index must be an Int, but got {}.", self.get_type_name(&i_ev)),
                t
            ));
        };

        if index < 0 || index >= (ls.borrow().len() as i128) {
            return Err(self.handle_error(
                &format!("Index out of bounds: index {} is not in [0, {}).", index, ls.borrow().len()),
                t
            ));
        }

        Ok(ls.borrow()[index as usize].clone())
    }

    fn evaluate_list(&mut self, content: Vec<Expression>, _t: Token) -> Result<Value, BreakResult> {
        let mut list = Vec::new();
        for item in content {
            list.push(self.evaluate(item)?);
        }
        Ok(Value::List(Rc::new(RefCell::new(list))))
    }

    fn handle_error(&self, msg: &str, token: Token) -> BreakResult {
        let line = token.line;
        let file = token.file.clone();
        let start = token.id as usize;
        let end = token.id_end as usize;

        if !self.repl{
            let mut colors = ColorGenerator::new();
            let a = colors.next();

            let src = fs::read_to_string(&file).unwrap_or_else(|_| "<could not read source file>".to_string());

            Report::build(ReportKind::Error, (&file, (line - 1) as usize..3))
                .with_message("Interpreter Error")
                .with_label(
                    Label::new((&file, start..end))
                        .with_message(msg)
                        .with_color(a),
                )
                .finish()
                .print((&file, Source::from(&src)))
                .unwrap();
        }
        
        BreakResult::Error(format!("[Interpreter Error: {}", msg))
    }
}
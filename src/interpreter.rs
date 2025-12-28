use crate::ast::Expression;
use crate::ast::Value;
use crate::ast::Statement; 
use crate::ast::BreakResult;
use crate::func;
use crate::token::TokenKind; 
use crate::token::Token; 
use crate::environment::Environment; 
use std::cell::Ref;
use std::clone;
use std::process; 
use std::rc::Rc; 
use std::cell::RefCell; 
use crate::func::*; 


//TODO FIX THE PRIVACY LEVELS OF THE INTERPRET (This removed but it'd be very helpful to properly learn how trait works in Rust )

pub struct Interpreter{
    pub global_environment: Rc<RefCell<Environment>>,
    pub is_prime: bool, 
}

impl Interpreter {

    pub fn new(is_prime: bool) -> Self {

        let mut environment = Rc::new(RefCell::new(Environment::new(None))); 

        let mut edittable_env = environment.borrow_mut();
        edittable_env.define_default("timeit", Value::Call(Rc::new(Timeit{}), Rc::clone(&environment))); //outputs current time in seconds 
        edittable_env.define_default("abs", Value::Call(Rc::new(Abs{}), Rc::clone(&environment))); //Absolute value
        Interpreter{
            global_environment: Rc::clone(&environment),
            is_prime: true
        }
    }
    
    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<Value, BreakResult>{

        for stmt in statements{
            let result = self.execute(stmt);

            //dbg!(&result);
            //dbg!(&self.is_prime);
            if self.is_prime{
                match result {
                    Err(BreakResult::Error(msg)) => {eprintln!("{}", msg); process::exit(1)},
                    Err(BreakResult::Return(t, val)) => 
                        {
                            eprintln!("[Line {}] Interpreter Error: {}", t.line, "Return is only allowed in functions"); process::exit(1)
                        },
                    Err(BreakResult::Break(t)) => {
                            eprintln!("[Line {}] Interpreter Error: {}", t.line, "Break is only allowed in loops"); process::exit(1)
                        },
                    _=> {}
                }
            } else {
                match result {
                    Err(a) => {return Err(a)},
                    _=> {}
                }
            }

        }
            
        return Ok(Value::None);
    }

    fn execute(&mut self, stmt:Statement) -> Result<Value, BreakResult>{
        match stmt {
            Statement::Expression(exp)=> {return self.evaluate(exp);}
            Statement::If(exp, then_s, else_s) => {return self.execute_if(exp, *then_s, *else_s);},
            Statement::Function(t,vt ,vs ) => {return self.execute_function(t, vt, *vs);},
            Statement::Print(exp) => {return self.execute_print(exp);},     
            Statement::Return(t, val) => {return self.execute_return(t, val);},
            Statement::Var(var, value) => {return self.execute_var(var, value);},
            Statement::Block(statements) => {return self.execute_block(*statements);},
            Statement::While(exp, s) => {return self.execute_while(exp, *s);},
            Statement::Break(t) => {return self.execute_break(t)},
            _ => {unreachable!()}
        };

        return Ok(Value::None); 
    }

    fn to_bool(&self, val: &Value) -> bool{
        match *val { 
            Value::Bool(t) => {t},
            Value::Int(n) => {
                if n == 0 {
                    false
                } else{
                    true
                }
            },
            Value::Float(n) => {
                if n == 0.0 {
                    false
                } else {
                    true
                }
            },
            Value::None => {false},
            _=> {true}
        }
    }

    fn execute_expression(&mut self, exp: Expression) -> Result<Value, BreakResult>{
        self.evaluate(exp)?;

        return Ok(Value::None);
    }

    fn execute_if(&mut self, exp: Expression, then_s: Statement, else_s: Statement) -> Result<Value, BreakResult>{

        let val = self.evaluate(exp)?;
        
        if self.to_bool(&val) {
            return self.execute(then_s);
        } else {
            return self.execute(else_s);
        }
    }

    fn execute_function(&mut self, t: Token, vt: Vec<Token>, vs: Vec<Statement>)-> Result<Value, BreakResult>{

        let t_clone = (&t).clone();

        let function_call = Function {
            name: t, args_list: vt, statement_list: vs
        };

        return self.global_environment.borrow_mut().define(t_clone, Value::Call(Rc::new(function_call), Rc::clone(&self.global_environment)));
    }

    fn execute_print(&mut self, expression:Expression)-> Result<Value, BreakResult>{
        let value = self.evaluate(expression)?; 

        match value { 
            Value::Int(m) => {println!("{}", m)},
            Value::Float(m) => {println!("{}", m)},
            Value::Bool(m) => {println!("{}", m)},
            Value::None => {println!("none")},
            Value::String(m) => {println!("{}", m)},
            Value::Call(callee, env) => {println!("<fn {}>", callee.toString())}
        }

        return Ok(Value::None);
    }

    fn execute_return(&mut self, t: Token, val: Expression)-> Result<Value, BreakResult>{
        let val_ev = self.evaluate(val)?;
        return Err(BreakResult::Return(t, val_ev));
    }

    fn execute_var(&mut self, var: Token, value: Expression)-> Result<Value, BreakResult>{ 

        let evaluated_var = self.evaluate(value)?; 
        return self.global_environment.borrow_mut().define(var, evaluated_var);
    }

    fn execute_block(&mut self, statements: Vec<Statement>)-> Result<Value, BreakResult>{
        //let mut scope_env = Environment::new(Some(Rc::new(RefCell::new)));
        let curr_env  = Rc::clone(&self.global_environment); 
        self.global_environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&self.global_environment)))));
        
        for statement in statements{
            self.execute(statement)?;
        }
        
        //let result = self.interpret(statements);

        self.global_environment = curr_env;

        return Ok(Value::None); 
    }

    fn execute_while(&mut self, exp: Expression, s: Statement)-> Result<Value, BreakResult>{

        let mut exp_ev = self.evaluate(exp.clone())?; 

        while self.to_bool(&exp_ev){

            let result = self.execute(s.clone());
            match result{
                Err(BreakResult::Break(t)) => {break;},
                Err(br) => {return Err(br);},
                _ => {},
            }
            
            exp_ev = self.evaluate(exp.clone())?;     
        }

        return Ok(Value::None); 
    }

    fn execute_break(&mut self, t: Token) -> Result<Value, BreakResult>{ 
        return Err(BreakResult::Break(t));
    }

    fn evaluate(&mut self, expression: Expression) -> Result<Value, BreakResult>{
        //dbg!(&expression);
        match expression{ 
            Expression::Assign(t, a) => self.evaluate_assign(t, a),
            Expression::Binary(l, o, r) => self.evaluate_binary(l, o, r), 
            Expression::Unary(o, r) => self.evaluate_unary(o, r),
            Expression::Call(callee, paren, args) => {self.evaluate_call(*callee, paren, *args)} 
            Expression::Logical(l,o ,r ) => self.evaluate_logical(*l, o, *r),
            Expression::Literal(v) => self.evaluate_literal(v),
            Expression::Grouping(exp) => self.evaluate_grouping(exp), 
            Expression::Variable(t) => self.evaluate_variable(t)
        }
    }
    
    fn evaluate_assign(&mut self, t: Token, a: Box<Expression>) -> Result<Value, BreakResult>{
        let a_ev = self.evaluate(*a)?; 

        self.global_environment.borrow_mut().assign(t, a_ev);
        return Ok(Value::None); 
    }

   fn evaluate_binary(&mut self, l: Box<Expression>, o: Token, r: Box<Expression>) -> Result<Value, BreakResult> {
        let l_ev = self.evaluate(*l)?;
        let r_ev = self.evaluate(*r)?;

        match o.kind {
            TokenKind::PLUS => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Ok(Value::Int(m + n)),
                (Value::Float(m), Value::Float(n)) => Ok(Value::Float(m + n)),
                (Value::Int(m), Value::Float(n)) => Ok(Value::Float((m as f64) + n)),
                (Value::Float(m), Value::Int(n)) => Ok(Value::Float(m + (n as f64))),
                (Value::String(m), Value::Int(n)) => Ok(Value::String(m.clone() + &n.to_string())),
                (Value::String(m), Value::Float(n)) => Ok(Value::String(m.clone() + &n.to_string())),
                (Value::String(m), Value::String(n)) => Ok(Value::String(m.clone() + &n)),
                _ => Err(self.handle_error("Invalid operation performed with '+'", o.line)),
            },

            TokenKind::MINUS => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Ok(Value::Int(m - n)),
                (Value::Float(m), Value::Float(n)) => Ok(Value::Float(m - n)),
                (Value::Int(m), Value::Float(n)) => Ok(Value::Float((m as f64) - n)),
                (Value::Float(m), Value::Int(n)) => Ok(Value::Float(m - (n as f64))),
                _ => Err(self.handle_error("Invalid operation performed with '-'", o.line)),
            },

            TokenKind::STAR => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Ok(Value::Int(m * n)),
                (Value::Float(m), Value::Float(n)) => Ok(Value::Float(m * n)),
                (Value::Int(m), Value::Float(n)) => Ok(Value::Float((m as f64) * n)),
                (Value::Float(m), Value::Int(n)) => Ok(Value::Float(m * (n as f64))),
                _ => Err(self.handle_error("Invalid operation performed with '*'", o.line)),
            },

            TokenKind::SLASH => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => {
                    if n == 0 { Err(self.handle_error("Division by 0", o.line)) } 
                    else { Ok(Value::Int(m / n)) }
                }
                (Value::Float(m), Value::Float(n)) => {
                    if n == 0.0 { Err(self.handle_error("Division by 0", o.line)) } 
                    else { Ok(Value::Float(m / n)) }
                }
                (Value::Int(m), Value::Float(n)) => {
                    if n == 0.0 { Err(self.handle_error("Division by 0", o.line)) } 
                    else { Ok(Value::Float((m as f64) / n)) }
                }
                (Value::Float(m), Value::Int(n)) => {
                    if n == 0 { Err(self.handle_error("Division by 0", o.line)) } 
                    else { Ok(Value::Float(m / (n as f64))) }
                }
                _ => Err(self.handle_error("Invalid operation performed with '/'", o.line)),
            },

            TokenKind::PERCENT => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Ok(Value::Int(m % n)),
                (Value::Float(m), Value::Float(n)) => Ok(Value::Float(m % n)),
                (Value::Int(m), Value::Float(n)) => Ok(Value::Float((m as f64) % n)),
                (Value::Float(m), Value::Int(n)) => Ok(Value::Float(m % (n as f64))),
                _ => Err(self.handle_error("Invalid operation performed with '%'", o.line)),
            },

            TokenKind::GREATER => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Ok(Value::Bool(m > n)),
                (Value::Float(m), Value::Float(n)) => Ok(Value::Bool(m > n)),
                (Value::Int(m), Value::Float(n)) => Ok(Value::Bool((m as f64) > n)),
                (Value::Float(m), Value::Int(n)) => Ok(Value::Bool(m > (n as f64))),
                _ => Err(self.handle_error("Invalid operation performed with '>'", o.line)),
            },

            TokenKind::LESS => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Ok(Value::Bool(m < n)),
                (Value::Float(m), Value::Float(n)) => Ok(Value::Bool(m < n)),
                (Value::Int(m), Value::Float(n)) => Ok(Value::Bool((m as f64) < n)),
                (Value::Float(m), Value::Int(n)) => Ok(Value::Bool(m < (n as f64))),
                _ => Err(self.handle_error("Invalid operation performed with '<'", o.line)),
            },

            TokenKind::GREATER_EQUAL => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Ok(Value::Bool(m >= n)),
                (Value::Float(m), Value::Float(n)) => Ok(Value::Bool(m >= n)),
                (Value::Int(m), Value::Float(n)) => Ok(Value::Bool((m as f64) >= n)),
                (Value::Float(m), Value::Int(n)) => Ok(Value::Bool(m >= (n as f64))),
                _ => Err(self.handle_error("Invalid operation performed with '>='", o.line)),
            },

            TokenKind::LESS_EQUAL => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Ok(Value::Bool(m <= n)),
                (Value::Float(m), Value::Float(n)) => Ok(Value::Bool(m <= n)),
                (Value::Int(m), Value::Float(n)) => Ok(Value::Bool((m as f64) <= n)),
                (Value::Float(m), Value::Int(n)) => Ok(Value::Bool(m <= (n as f64))),
                _ => Err(self.handle_error("Invalid operation performed with '<='", o.line)),
            },

            TokenKind::EQUAL_EQUAL => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Ok(Value::Bool(m == n)),
                (Value::Float(m), Value::Float(n)) => Ok(Value::Bool(m == n)),
                (Value::Int(m), Value::Float(n)) => Ok(Value::Bool((m as f64) == n)),
                (Value::Float(m), Value::Int(n)) => Ok(Value::Bool(m == (n as f64))),
                (Value::Bool(m), Value::Bool(n)) => Ok(Value::Bool(m == n)),
                (Value::String(m), Value::String(n)) => Ok(Value::Bool(m == n)),
                (Value::None, Value::None) => Ok(Value::Bool(true)),
                _ => Err(self.handle_error("Invalid operation performed with '=='", o.line)),
            },

            TokenKind::BANG_EQUAL => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Ok(Value::Bool(m != n)),
                (Value::Float(m), Value::Float(n)) => Ok(Value::Bool(m != n)),
                (Value::Int(m), Value::Float(n)) => Ok(Value::Bool((m as f64) != n)),
                (Value::Float(m), Value::Int(n)) => Ok(Value::Bool(m != (n as f64))),
                (Value::Bool(m), Value::Bool(n)) => Ok(Value::Bool(m != n)),
                _ => Err(self.handle_error("Invalid operation performed with '!='", o.line)),
            },
            _ => Err(self.handle_error("Unknown binary operator", o.line)),
        }
    }

    fn evaluate_unary(&mut self,  o: Token, r: Box<Expression>) -> Result<Value, BreakResult> {

        let r_ev: Value = self.evaluate(*r)?; 

        match o.kind{
            TokenKind::MINUS => {
                if let Value::Int(m) = r_ev { 
                    return Ok(Value::Int(- m));
                } else if let Value::Float(m) = r_ev {
                    return Ok(Value::Float(- m));
                }
                else {
                    return Err(self.handle_error(&format!("Invalid operation performed with '-'"), o.line)); 
                }
            }

            TokenKind::BANG => {
                if let Value::Bool(m) = r_ev { 
                    return Ok(Value::Bool(!m));
                } 
                else {
                    return Err(self.handle_error(&format!("Invalid operation performed with '!'"), o.line)); 
                }
            }
            _ => {}
        }

        return  Ok(Value::None);
    }

    fn evaluate_call(&mut self, callee:Expression, paren:Token, args: Vec<Expression>) -> Result<Value, BreakResult> {
        let callee_ev = self.evaluate(callee)?; 
        
        let mut processed_args: Vec<Value> = Vec::new();

        for arg in args {
            processed_args.push(self.evaluate(arg)?);
        }

        let mut func_env = Environment::new(Some(Rc::clone(&self.global_environment))); 

        match callee_ev {
            Value::Call(call, env) => {
                let value = call.call(Interpreter { global_environment: Rc::clone(&env), is_prime: false}, processed_args);
                
                match value {
                    Ok(v) => {return Ok(v)},
                    Err(BreakResult::Error(e)) => {return Err(self.handle_error( &format!("Error from function call \n \t ^ {}", &e), paren.line));}
                    Err(a) => {return Err(a);}
                }
            },
            _ => {return Err(self.handle_error("Only Calls can be called", paren.line));}
        }

        unreachable!();
    }
    
    fn evaluate_logical(&mut self, l:Expression, o: Token, r: Expression) -> Result<Value, BreakResult>{
        let l_ev = self.evaluate(l)?;

        if o.kind == TokenKind::OR {
            if self.to_bool(&l_ev) {
                return Ok(l_ev); 
            }
        } else {
            if !self.to_bool(&l_ev) {
                return Ok(l_ev); 
            }
        }

        return self.evaluate(r);
    }

    fn evaluate_literal(&mut self, v: Value) -> Result<Value, BreakResult>{
        return Ok(v) 
    }

    fn evaluate_grouping(&mut self, exp: Box<Expression>) -> Result<Value, BreakResult>{
        return self.evaluate(*exp);
    }

    fn evaluate_variable(&mut self, token: Token) -> Result<Value, BreakResult>{
        return self.global_environment.borrow_mut().get(token);

        //The none case should be impossible
    }

    // fn handle_error(&self, msg: &str, line: i32) {

    //     eprintln!("[Line {}] Interpreter Error: {}", line, msg);
    //     process::exit(1);
    // }

    fn handle_error(&self, msg: &str, line: i32) -> BreakResult{
        return BreakResult::Error(format!("[Line {}] Interpreter Error: {}", line, msg));
    }
}


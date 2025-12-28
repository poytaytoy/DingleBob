use crate::ast::Expression;
use crate::ast::Value;
use crate::ast::Statement; 
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
    loop_iteration: i32, 
}

impl Interpreter {

    pub fn new() -> Self {

        let mut environment = Environment::new(None); 
        environment.define_default("timeit", Value::Call(Rc::new(Timeit{}))); //outputs current time in seconds 
        environment.define_default("abs", Value::Call(Rc::new(Abs{}))); //Absolute value
        Interpreter{
            global_environment: Rc::new(RefCell::new(environment)),
            loop_iteration: 0
        }
    }
    
    pub fn interpret(&mut self, statements: Vec<Statement>){

        let curr_loop_iteration = self.loop_iteration; 

        for stmt in statements{
            if curr_loop_iteration == self.loop_iteration{
                self.execute(stmt); 
            } else {
                break;
            }

        }
    }

    fn execute(&mut self, stmt:Statement){
        match stmt {
            Statement::Expression(exp)=> {self.evaluate(exp);}
            Statement::If(exp, then_s, else_s) => {self.execute_if(exp, *then_s, *else_s)},
            Statement::Function(t,vt ,vs ) => {self.execute_function(t, vt, *vs);},
            Statement::Print(exp) => {self.execute_print(exp)},     
            Statement::Return(t, val) => {self.execute_return(t, val)},
            Statement::Var(var, value) => {self.execute_var(var, value)},
            Statement::Block(statements) => {self.execute_block(*statements)},
            Statement::While(exp, s) => {self.execute_while(exp, *s)},
            Statement::Break(t) => {
                if self.loop_iteration == 0{
                    self.handle_error("Break can only be used in a loop", t.line)
                } else {
                    self.loop_iteration -= 1;
                }
                ;
            },
            _ => {unreachable!()}
        };
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

    fn execute_if(&mut self, exp: Expression, then_s: Statement, else_s: Statement){

        let val = self.evaluate(exp);
        
        if self.to_bool(&val) {
            self.execute(then_s);
        } else {
            self.execute(else_s);
        }
    }

    fn execute_function(&mut self, t: Token, vt: Vec<Token>, vs: Vec<Statement>){

        let t_clone = (&t).clone();

        let function_call = Function {
            name: t, args_list: vt, statement_list: vs
        };

        self.global_environment.borrow_mut().define(t_clone, Value::Call(Rc::new(function_call)));
    }

    fn execute_print(&mut self, expression:Expression){
        let value = self.evaluate(expression); 

        match value { 
            Value::Int(m) => {println!("{}", m)},
            Value::Float(m) => {println!("{}", m)},
            Value::Bool(m) => {println!("{}", m)},
            Value::None => {println!("none")},
            Value::String(m) => {println!("{}", m)},
            Value::Call(callee) => {println!("<fn {}>", callee.toString())}
        }
    }

    fn execute_return(&mut self, t: Token, val: Expression){
    
    }

    fn execute_var(&mut self, var: Token, value: Expression){ 

        let evaluated_var = self.evaluate(value); 
        self.global_environment.borrow_mut().define(var, evaluated_var);
    }

    fn execute_block(&mut self, statements: Vec<Statement>){
        //let mut scope_env = Environment::new(Some(Rc::new(RefCell::new)));
        let curr_env  = Rc::clone(&self.global_environment); 
        self.global_environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&self.global_environment)))));
        
        self.interpret(statements);

        self.global_environment = curr_env;
    }

    fn execute_while(&mut self, exp: Expression, s: Statement){

        let mut exp_ev = self.evaluate(exp.clone()); 

        self.loop_iteration += 1; 
        //TODO ngl, this cloning thing feels really awkward, might be worth checking this up once all is done

        let curr_loop_iteration = self.loop_iteration; 

        while self.to_bool(&exp_ev) && curr_loop_iteration == self.loop_iteration{
            self.execute(s.clone());
            exp_ev = self.evaluate(exp.clone());    
        }

        if curr_loop_iteration == self.loop_iteration{
            self.loop_iteration -= 1; 
        }
    }

    fn evaluate(&mut self, expression: Expression) -> Value{
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
    
    fn evaluate_assign(&mut self, t: Token, a: Box<Expression>) -> Value{

        let a_ev = self.evaluate(*a); 

        self.global_environment.borrow_mut().assign(t, a_ev.clone());

        return a_ev; 
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
                    self.handle_error(&format!("Invalid operation performed with '+'"), o.line);
                    Value::None
                }
            },

            TokenKind::MINUS => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Value::Int(m - n),
                (Value::Float(m), Value::Float(n)) => Value::Float(m - n),
                (Value::Int(m), Value::Float(n)) => Value::Float((m as f64) - n),
                (Value::Float(m), Value::Int(n)) => Value::Float(m - (n as f64)),
                _ => {
                    self.handle_error(&format!("Invalid operation performed with '-'"), o.line);
                    Value::None
                }
            },

            TokenKind::STAR => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Value::Int(m * n),
                (Value::Float(m), Value::Float(n)) => Value::Float(m * n),
                (Value::Int(m), Value::Float(n)) => Value::Float((m as f64) * n),
                (Value::Float(m), Value::Int(n)) => Value::Float(m * (n as f64)),
                _ => {
                    self.handle_error(&format!("Invalid operation performed with '*'"), o.line);
                    Value::None
                }
            },

            TokenKind::SLASH => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => {
                    if n == 0 {
                        self.handle_error(&format!("Division by 0"), o.line);
                        Value::None
                    } else {
                        Value::Int(m / n)
                    }
                }
                (Value::Float(m), Value::Float(n)) => {
                    if n == 0.0 {
                        self.handle_error(&format!("Division by 0"), o.line);
                        Value::None
                    } else {
                        Value::Float(m / n)
                    }
                }
                (Value::Int(m), Value::Float(n)) => {
                    if n == 0.0 {
                        self.handle_error(&format!("Division by 0"), o.line);
                        Value::None
                    } else {
                        Value::Float((m as f64) / n)
                    }
                }
                (Value::Float(m), Value::Int(n)) => {
                    if n == 0 {
                        self.handle_error(&format!("Division by 0"), o.line);
                        Value::None
                    } else {
                        Value::Float(m / (n as f64))
                    }
                }
                _ => {
                    self.handle_error(&format!("Invalid operation performed with '/'"), o.line);
                    Value::None
                }
            },
             TokenKind::PERCENT => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Value::Int(m % n),
                (Value::Float(m), Value::Float(n)) => Value::Float(m % n),
                (Value::Int(m), Value::Float(n)) => Value::Float((m as f64) % n),
                (Value::Float(m), Value::Int(n)) => Value::Float(m % (n as f64)),
                _ => {
                    self.handle_error(&format!("Invalid operation performed with '%'"), o.line);
                    Value::None
                }
            },
            TokenKind::SLASH => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => {
                    if n == 0 {
                        self.handle_error(&format!("Division by 0"), o.line);
                        Value::None
                    } else {
                        Value::Int(m / n)
                    }
                }
                (Value::Float(m), Value::Float(n)) => {
                    if n == 0.0 {
                        self.handle_error(&format!("Division by 0"), o.line);
                        Value::None
                    } else {
                        Value::Float(m / n)
                    }
                }
                (Value::Int(m), Value::Float(n)) => {
                    if n == 0.0 {
                        self.handle_error(&format!("Division by 0"), o.line);
                        Value::None
                    } else {
                        Value::Float((m as f64) / n)
                    }
                }
                (Value::Float(m), Value::Int(n)) => {
                    if n == 0 {
                        self.handle_error(&format!("Division by 0"), o.line);
                        Value::None
                    } else {
                        Value::Float(m / (n as f64))
                    }
                }
                _ => {
                    self.handle_error(&format!("Invalid operation performed with '/'"), o.line);
                    Value::None
                }
            },

            TokenKind::GREATER => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Value::Bool(m > n),
                (Value::Float(m), Value::Float(n)) => Value::Bool(m > n),
                (Value::Int(m), Value::Float(n)) => Value::Bool((m as f64) > n),
                (Value::Float(m), Value::Int(n)) => Value::Bool(m > (n as f64)),
                _ => {
                    self.handle_error(&format!("Invalid operation performed with '>'"), o.line);
                    Value::None
                }
            },

            TokenKind::LESS => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Value::Bool(m < n),
                (Value::Float(m), Value::Float(n)) => Value::Bool(m < n),
                (Value::Int(m), Value::Float(n)) => Value::Bool((m as f64) < n),
                (Value::Float(m), Value::Int(n)) => Value::Bool(m < (n as f64)),
                _ => {
                    self.handle_error(&format!("Invalid operation performed with '<'"), o.line);
                    Value::None
                }
            },

            TokenKind::GREATER_EQUAL => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Value::Bool(m >= n),
                (Value::Float(m), Value::Float(n)) => Value::Bool(m >= n),
                (Value::Int(m), Value::Float(n)) => Value::Bool((m as f64) >= n),
                (Value::Float(m), Value::Int(n)) => Value::Bool(m >= (n as f64)),
                _ => {
                    self.handle_error(&format!("Invalid operation performed with '>='"), o.line);
                    Value::None
                }
            },

            TokenKind::LESS_EQUAL => match (l_ev, r_ev) {
                (Value::Int(m), Value::Int(n)) => Value::Bool(m <= n),
                (Value::Float(m), Value::Float(n)) => Value::Bool(m <= n),
                (Value::Int(m), Value::Float(n)) => Value::Bool((m as f64) <= n),
                (Value::Float(m), Value::Int(n)) => Value::Bool(m <= (n as f64)),
                _ => {
                    self.handle_error(&format!("Invalid operation performed with '<='"), o.line);
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
                    self.handle_error(&format!("Invalid operation performed with '=='"), o.line);
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
                    self.handle_error(&format!("Invalid operation performed with '!='"), o.line);
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
                    self.handle_error(&format!("Invalid operation performed with '-'"), o.line); 
                }
            }

            TokenKind::BANG => {
                if let Value::Bool(m) = r_ev { 
                    return Value::Bool(!m);
                } 
                else {
                    self.handle_error(&format!("Invalid operation performed with '!'"), o.line); 
                }
            }
            _ => {}
        }

        return Value::None; 
    }

    fn evaluate_call(&mut self, callee:Expression, paren:Token, args: Vec<Expression>) -> Value{
        let callee_ev = self.evaluate(callee); 
        
        let mut processed_args: Vec<Value> = Vec::new();

        for arg in args {
            processed_args.push(self.evaluate(arg));
        }

        let mut func_env = Environment::new(Some(Rc::clone(&self.global_environment))); 

        match callee_ev {
            Value::Call(call) => {
                let value = call.call(Interpreter { global_environment: Rc::new(RefCell::new(func_env)), loop_iteration: 0 }, processed_args);
                match value { 
                    Ok(v) => {return v},
                    Err(e) => {self.handle_error( &format!("Error from function call \n \t ^ {}", &e), paren.line);}
                }
            },
            _ => {self.handle_error("Only Calls can be called", paren.line);}
        }

        unreachable!();
    }
    
    fn evaluate_logical(&mut self, l:Expression, o: Token, r: Expression) -> Value{
        let l_ev = self.evaluate(l);

        if o.kind == TokenKind::OR {
            if self.to_bool(&l_ev) {
                return l_ev; 
            }
        } else {
            if !self.to_bool(&l_ev) {
                return l_ev; 
            }
        }

        return self.evaluate(r);
    }

    fn evaluate_literal(&mut self, v: Value) -> Value{
        return v; 
    }

    fn evaluate_grouping(&mut self, exp: Box<Expression>) -> Value{
        self.evaluate(*exp)
    }

    fn evaluate_variable(&mut self, token: Token) -> Value{
        self.global_environment.borrow_mut().get(token).unwrap()

        //The none case should be impossible
    }

    fn handle_error(&self, msg: &str, line: i32) {

        eprintln!("[Line {}] Interpreter Error: {}", line, msg);
        process::exit(1);
    }
}


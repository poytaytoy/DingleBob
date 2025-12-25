use crate::ast::Expression;
use crate::ast::Value;
use crate::ast::Statement; 
use crate::token::TokenKind; 
use crate::token::Token; 
use crate::environment::Environment; 
use std::cell::Ref;
use std::f32::consts::E;
use std::process; 
use std::rc::Rc; 
use std::cell::RefCell; 


//TODO FIX THE PRIVACY LEVELS OF THE INTERPRET (This removed but it'd be very helpful to properly learn how trait works in Rust )

pub struct Interpreter{
    global_environment: Rc<RefCell<Environment>>,
}


impl Interpreter {

    pub fn new() -> Self {
        Interpreter{
            global_environment: Rc::new(RefCell::new(Environment::new(None)))
        }
    }
    
    pub fn interpret(&mut self, statements: Vec<Statement>){
        for stmt in statements{
            //dbg!(&stmt);
            self.execute(stmt); 
        }
    }

    fn execute(&mut self, stmt:Statement){
        match stmt {
            Statement::Print(exp) => {self.execute_print(exp)},     
            Statement::Var(var, value) => {self.execute_var(var, value)},
            Statement::Block(statements) => {self.execute_block(*statements)}
            Statement::Expression(exp)=> {self.evaluate(exp);}
        };
    }

    fn execute_print(&mut self, expression:Expression){
        let value = self.evaluate(expression); 

        match value { 
            Value::Int(m) => {println!("{}", m)},
            Value::Float(m) => {println!("{}", m)},
            Value::Bool(m) => {println!("{}", m)},
            Value::None => {println!("None")},
            Value::String(m) => {println!("{}", m)}
        }
    }

    fn execute_var(&mut self, var: String, value: Expression){ 

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

    fn evaluate(&mut self, expression: Expression) -> Value{
        //dbg!(&expression);
        match expression{ 
            Expression::Assign(t, a) => self.evaluate_assign(t, a),
            Expression::Binary(l, o, r) => self.evaluate_binary(l, o, r), 
            Expression::Unary(o, r) => self.evaluate_unary(o, r), 
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


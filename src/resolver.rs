use crate::{ast::{Expression, Statement, Value}, interpreter::Interpreter, token::Token}; 
use std::{collections::{HashMap, VecDeque}, hash::Hash, process, rc::Rc}; 

pub struct Resolver {
    pub locals: HashMap<Token, i32>,
    stack: VecDeque<HashMap<String, bool>>,
}

impl Resolver{

    pub fn new() -> Self {
        let stack = VecDeque::new();
        Resolver { locals: HashMap::new(), stack: stack}
    }

    pub fn resolve(&mut self, statements: Vec<Statement>){
        for statement in statements{
            self.resolve_stmt(statement);
        }
    }

    fn resolve_stmt(&mut self, stmt: Statement) {
        match stmt {
            Statement::Expression(exp) => { self.resolve_exp(exp); }
            Statement::If(exp, then_s, else_s) => { self.resolve_if(exp, *then_s, *else_s); },
            Statement::Function(t, vt, vs) => { self.resolve_function(t, vt, *vs); },
            Statement::Print(exp) => { self.resolve_print(exp); },
            Statement::Return(t, val) => { self.resolve_return(t, val); },
            Statement::Var(var, value) => { self.resolve_var(var, value); },
            Statement::Block(statements) => { self.resolve_block(*statements); },
            Statement::While(exp, s) => { self.resolve_while(exp, *s); },
            Statement::Break(t) => { self.resolve_break(t) },
            _ => { unreachable!() }
        };
    }

    fn resolve_if(&mut self, exp: Expression, then_s: Statement, else_s: Statement) {
        self.resolve_exp(exp);
        self.resolve_stmt(then_s);
        self.resolve_stmt(else_s);
    }

    fn resolve_function(&mut self, name: Token, params: Vec<Token>, body: Vec<Statement>) {
        
        if !self.stack.is_empty(){
            self.stack[0].insert(name.lexeme, true);
            self.begin_scope();

            for args in params {
                self.stack[0].insert(args.lexeme, true);
            }
            self.resolve_block(body);
            
            self.end_scope();
        }
        
    }

    fn resolve_print(&mut self, exp: Expression) {
        self.resolve_exp(exp);
    }

    fn resolve_return(&mut self, token: Token, val: Expression) {
        self.resolve_exp(val);
    }

    fn resolve_var(&mut self, var: Token, value: Expression) {
        self.resolve_exp(value);

        if !self.stack.is_empty(){
            if self.stack[0].contains_key(&var.lexeme){
                self.handle_error(
                    &format!("Duplicate definition: '{}' is already defined in this scope.", &var.lexeme),
                    var.line
                );
            };
            self.stack[0].insert(var.lexeme.clone(), true);
        }
    }

    fn resolve_block(&mut self, statements: Vec<Statement>) {

        self.begin_scope();

        for statement in statements{
            self.resolve_stmt(statement);
        }

        self.end_scope();
    }

    fn resolve_while(&mut self, exp: Expression, body: Statement) {
        self.resolve_exp(exp);
        self.resolve_stmt(body);
    }

    fn resolve_break(&mut self, token: Token) {
    }

    fn resolve_exp(&mut self, exp: Expression) {
        match exp {
            Expression::Assign(t, a, v) => { self.resolve_assign(*t, a, *v); },
            Expression::Binary(l, o, r) => { self.resolve_binary(*l, o, *r); },
            Expression::Unary(o, r) => { self.resolve_unary(o, *r); },
            Expression::Call(callee, paren, args) => { self.resolve_call(*callee, paren, *args); },
            Expression::Logical(l, o, r) => { self.resolve_logical(*l, o, *r); },
            Expression::Literal(v) => { self.resolve_literal(v); },
            Expression::Grouping(exp) => { self.resolve_grouping(*exp); },
            Expression::Variable(t) => { self.resolve_variable(t); },
            Expression::Lambda(t, stmt) => {self.resolve_lambda(t, *stmt)},
            Expression::Index(l,t ,i ) => {self.resolve_index(*l, t, *i)},
            Expression::List(content, t) => self.resolve_list(*content, t)
        
        }
    }

    fn resolve_assign(&mut self, assignee: Expression, equal: Token, value: Expression) {
        self.resolve_exp(assignee);
        self.resolve_exp(value);
    }

    fn resolve_binary(&mut self, left: Expression, operator: Token, right: Expression) {
        self.resolve_exp(left);
        self.resolve_exp(right);
    }

    fn resolve_unary(&mut self, operator: Token, right: Expression) {
        self.resolve_exp(right);
    }

    fn resolve_call(&mut self, callee: Expression, paren: Token, args: Vec<Expression>) {
        self.resolve_exp(callee);
        for arg in args{
            self.resolve_exp(arg);
        }
    }

    fn resolve_logical(&mut self, left: Expression, operator: Token, right: Expression) {
        self.resolve_exp(left);
        self.resolve_exp(right);
    }

    fn resolve_literal(&mut self, value: Value) {
        // Nothing to resolve for literals (no variables inside)
    }

    fn resolve_grouping(&mut self, exp: Expression) {
        self.resolve_exp(exp);
    }

    fn resolve_variable(&mut self, name: Token) {
        if self.stack.len() > 0{
            let mut env_count = self.stack.len();
            for i in 0..self.stack.len(){
                if self.stack[i].contains_key(&name.lexeme){
                    env_count = i;
                    break;
                }
            }
            self.locals.insert(name, env_count as i32); 
        }

        //dbg!(&self.locals);
    }

    fn resolve_lambda(&mut self, t: Vec<Token>, stmts: Vec<Statement>){
        if !self.stack.is_empty(){
            self.begin_scope();

            for args in t {
                self.stack[0].insert(args.lexeme, true);
            }
            self.resolve_block(stmts);
            
            self.end_scope();
        }
    }

    fn resolve_index(&mut self, l: Expression, t: Token, i: Expression){
        self.resolve_exp(l);
        self.resolve_exp(i);
    }

    fn resolve_list(&mut self, content: Vec<Expression>, t: Token){

        for item in content{ 
            self.resolve_exp(item);
        }
    }

    fn begin_scope(&mut self) {
        self.stack.push_front(HashMap::new());
    }
    
    fn end_scope(&mut self) {
        self.stack.pop_front(); 
    }

    pub fn give_local(self) -> HashMap<Token, i32>{
        return self.locals;
    }

    fn handle_error(&self, msg: &str, line: i32) {

        eprintln!("[Line {}] Semantic Error: {}", line, msg);
        process::exit(1);
    }

}

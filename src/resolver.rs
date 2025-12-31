use crate::{ast::{Expression, Statement, Value}, interpreter::Interpreter, token::Token}; 
use std::{cell::RefCell, collections::{HashMap, VecDeque}, hash::Hash, rc::Rc}; 

type ResolveResult<T> = Result<T, String>;

pub struct Resolver {
    pub locals: Rc<RefCell<HashMap<Token, i32>>>,
    pub stack: VecDeque<HashMap<String, bool>>,
}

impl Resolver{

    pub fn clone(&mut self) -> Self{
        Resolver{ 
            locals:  Rc::new(RefCell::new(self.locals.borrow().clone())), 
            stack: self.stack.clone()
        }
    }

    pub fn new() -> Self {
        Resolver {
            locals: Rc::new(RefCell::new(HashMap::new())),
            stack: VecDeque::new(),
        }
    }

    pub fn resolve(&mut self, statements: Vec<Statement>) -> ResolveResult<()> {
        for statement in statements {
            self.resolve_stmt(statement)?;
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: Statement) -> ResolveResult<()> {
        match stmt {
            Statement::Expression(exp) => self.resolve_exp(exp),
            Statement::If(exp, then_s, else_s) => self.resolve_if(exp, *then_s, *else_s),
            Statement::Function(t, vt, vs) => self.resolve_function(t, vt, *vs),
            Statement::Print(exp) => self.resolve_print(exp),
            Statement::Return(t, val) => self.resolve_return(t, val),
            Statement::Var(var, value) => self.resolve_var(var, value),
            Statement::Block(statements) => self.resolve_block(*statements),
            Statement::While(exp, s) => self.resolve_while(exp, *s),
            Statement::Break(t) => self.resolve_break(t),
            _ => unreachable!(),
        }
    }

    fn resolve_if(&mut self, exp: Expression, then_s: Statement, else_s: Statement) -> ResolveResult<()> {
        self.resolve_exp(exp)?;
        self.resolve_stmt(then_s)?;
        self.resolve_stmt(else_s)?;
        Ok(())
    }

    fn resolve_function(&mut self, name: Token, params: Vec<Token>, body: Vec<Statement>) -> ResolveResult<()> {
        if !self.stack.is_empty() {
            self.stack[0].insert(name.lexeme, true);

            self.begin_scope();

            for args in params {
                self.stack[0].insert(args.lexeme, true);
            }

            self.resolve_block(body)?;
            self.end_scope();
        }
        Ok(())
    }

    fn resolve_print(&mut self, exp: Expression) -> ResolveResult<()> {
        self.resolve_exp(exp)
    }

    fn resolve_return(&mut self, _token: Token, val: Expression) -> ResolveResult<()> {
        self.resolve_exp(val)
    }

    fn resolve_var(&mut self, var: Token, value: Expression) -> ResolveResult<()> {
        self.resolve_exp(value)?;

        if !self.stack.is_empty() {
            if self.stack[0].contains_key(&var.lexeme) {
                // Pass the whole 'var' token instead of just var.line
                return self.handle_error(
                    &format!("Duplicate definition: '{}' is already defined in this scope.", &var.lexeme),
                    &var, 
                );
            }
            self.stack[0].insert(var.lexeme.clone(), true);
        }

        Ok(())
    }

    fn resolve_block(&mut self, statements: Vec<Statement>) -> ResolveResult<()> {
        self.begin_scope();

        for statement in statements {
            self.resolve_stmt(statement)?;
        }

        self.end_scope();
        Ok(())
    }

    fn resolve_while(&mut self, exp: Expression, body: Statement) -> ResolveResult<()> {
        self.resolve_exp(exp)?;
        self.resolve_stmt(body)?;
        Ok(())
    }

    fn resolve_break(&mut self, _token: Token) -> ResolveResult<()> {
        Ok(())
    }

    fn resolve_exp(&mut self, exp: Expression) -> ResolveResult<()> {
        match exp {
            Expression::Assign(t, a, v) => self.resolve_assign(*t, a, *v),
            Expression::Binary(l, o, r) => self.resolve_binary(*l, o, *r),
            Expression::Unary(o, r) => self.resolve_unary(o, *r),
            Expression::Call(callee, paren, args) => self.resolve_call(*callee, paren, *args),
            Expression::Logical(l, o, r) => self.resolve_logical(*l, o, *r),
            Expression::Literal(v) => self.resolve_literal(v),
            Expression::Grouping(exp) => self.resolve_grouping(*exp),
            Expression::Variable(t) => self.resolve_variable(t),
            Expression::Lambda(t, stmt) => self.resolve_lambda(t, *stmt),
            Expression::Index(l, t, i) => self.resolve_index(*l, t, *i),
            Expression::List(content, t) => self.resolve_list(*content, t),
        }
    }

    fn resolve_assign(&mut self, assignee: Expression, _equal: Token, value: Expression) -> ResolveResult<()> {
        self.resolve_exp(assignee)?;
        self.resolve_exp(value)?;
        Ok(())
    }

    fn resolve_binary(&mut self, left: Expression, _operator: Token, right: Expression) -> ResolveResult<()> {
        self.resolve_exp(left)?;
        self.resolve_exp(right)?;
        Ok(())
    }

    fn resolve_unary(&mut self, _operator: Token, right: Expression) -> ResolveResult<()> {
        self.resolve_exp(right)
    }

    fn resolve_call(&mut self, callee: Expression, _paren: Token, args: Vec<Expression>) -> ResolveResult<()> {
        self.resolve_exp(callee)?;
        for arg in args {
            self.resolve_exp(arg)?;
        }
        Ok(())
    }

    fn resolve_logical(&mut self, left: Expression, _operator: Token, right: Expression) -> ResolveResult<()> {
        self.resolve_exp(left)?;
        self.resolve_exp(right)?;
        Ok(())
    }

    fn resolve_literal(&mut self, _value: Value) -> ResolveResult<()> {
        Ok(())
    }

    fn resolve_grouping(&mut self, exp: Expression) -> ResolveResult<()> {
        self.resolve_exp(exp)
    }

    fn resolve_variable(&mut self, name: Token) -> ResolveResult<()> {
        if !self.stack.is_empty() {
            let mut env_count = self.stack.len();

            for i in 0..self.stack.len() {
                if self.stack[i].contains_key(&name.lexeme) {
                    env_count = i;
                    break;
                }
            }

            self.locals.borrow_mut().insert(name, env_count as i32);
        }

        Ok(())
    }

    fn resolve_lambda(&mut self, t: Vec<Token>, stmts: Vec<Statement>) -> ResolveResult<()> {
        if !self.stack.is_empty() {
            self.begin_scope();

            for args in t {
                self.stack[0].insert(args.lexeme, true);
            }

            self.resolve_block(stmts)?;
            self.end_scope();
        }
        Ok(())
    }

    fn resolve_index(&mut self, l: Expression, _t: Token, i: Expression) -> ResolveResult<()> {
        self.resolve_exp(l)?;
        self.resolve_exp(i)?;
        Ok(())
    }

    fn resolve_list(&mut self, content: Vec<Expression>, _t: Token) -> ResolveResult<()> {
        for item in content {
            self.resolve_exp(item)?;
        }
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.stack.push_front(HashMap::new());
    }
    
    fn end_scope(&mut self) {
        self.stack.pop_front(); 
    }

    pub fn give_local(&self) -> Rc<RefCell<HashMap<Token, i32>>>{
        Rc::clone(&self.locals)
    }

    fn handle_error<T>(&self, msg: &str, token: &Token) -> ResolveResult<T> {
        // Now using the token's metadata directly
        Err(format!("[{}: Line {}] Semantic Error at '{}': {}", token.file, token.line, token.lexeme, msg))
    }
}

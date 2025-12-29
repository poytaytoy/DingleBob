use std::collections::HashMap;
use crate::ast::Value;
use crate::ast::BreakResult;
use crate::token::Token;
use std::rc::Rc; 
use std::cell::RefCell; 

pub struct Environment { 
    env_superior: Option<Rc<RefCell<Environment>>>, 
    hashMap: HashMap<String, Value>
}

impl Environment { 
    pub fn new(env_superior: Option<Rc<RefCell<Environment>>>) -> Self {
        Environment {
            env_superior, 
            hashMap: HashMap::new()
        }
    }

    pub fn clone(&self) -> Self {

        let mut dummy_env_superior = None; 
        
        if !self.env_superior.is_none(){
            dummy_env_superior = Some(Rc::new(RefCell::new(self.env_superior.clone().unwrap().borrow().clone())));
        }

        Environment{
            env_superior: dummy_env_superior, 
            hashMap: self.hashMap.clone()
        }
    }

    pub fn define(&mut self, var: Token, value: Value) -> Result<Value, BreakResult> {
        if self.hashMap.contains_key(&var.lexeme) {
            return Err(BreakResult::Error(self.handle_error(&format!("Variable '{}' has already been declared", &var.lexeme), var.line)));
        }
        self.hashMap.insert(var.lexeme.clone(), value); 

        Ok(Value::None)
    }

    pub fn define_default(&mut self, name: &str, value: Value) {
        self.hashMap.insert(String::from(name), value); 
    }

    fn retrieve(&self, token: &Token) -> Option<Value> {
        if let Some(v) = self.hashMap.get(&token.lexeme) {
            return Some(v.clone());
        }

        match &self.env_superior { 
            // Note: use .borrow() for reading to avoid runtime panics
            Some(env) => env.borrow().retrieve(token),
            None => None
        }
    }

    pub fn get(&self, token: Token) -> Result<Value, BreakResult> {
        let cloned_token = token.clone();
        match self.retrieve(&token) {
            Some(v) => Ok(v), 
            None => Err(BreakResult::Error(self.handle_error(&format!("Undefined variable '{}' not found within current scope", cloned_token.lexeme), cloned_token.line)))
        }
    }

    pub fn get_at(&self, token: Token, steps: i32) -> Result<Value, BreakResult> {
        if steps == 0 {
            return match self.hashMap.get(&token.lexeme) {
                Some(v) => Ok(v.clone()),
                None => Err(BreakResult::Error(self.handle_error(&format!("Undefined variable '{}' at resolved depth", &token.lexeme), token.line)))
            };
        }

        match &self.env_superior {
            Some(env) => env.borrow().get_at(token, steps - 1),
            None => Err(BreakResult::Error(self.handle_error("Resolver depth exceeded environment chain", token.line)))
        }
    }
    
    pub fn assign(&mut self, token: Token, value: Value) -> Result<Value, BreakResult> {
        if self.hashMap.contains_key(&token.lexeme) {
            self.hashMap.insert(token.lexeme.clone(), value);
            Ok(Value::None)
        } else {
            match &mut self.env_superior {
                Some(env) => env.borrow_mut().assign(token, value),
                None => Err(BreakResult::Error(self.handle_error(&format!("Undefined symbol assignment not found within current scope {}", &token.lexeme), token.line)))
            }
        }
    }

    fn handle_error(&self, msg: &str, line: i32) -> String {
        format!("[Line {}] Interpreter Error: {}", line, msg)
    }
}
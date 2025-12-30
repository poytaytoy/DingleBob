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
            return Err(BreakResult::Error(self.handle_error(
                &format!("Name '{}' is already defined in this scope.", &var.lexeme),
                var.line
            )));
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
            Some(env) => env.borrow().retrieve(token),
            None => None
        }
    }

    pub fn get(&self, token: Token) -> Result<Value, BreakResult> {
        let cloned_token = token.clone();
        match self.retrieve(&token) {
            Some(v) => Ok(v), 
            None => Err(BreakResult::Error(self.handle_error(
                &format!("Undefined variable '{}': no binding found in this scope (or any enclosing scope).", cloned_token.lexeme),
                cloned_token.line
            )))
        }
    }

    pub fn get_at(&self, token: Token, steps: i32) -> Result<Value, BreakResult> {
        if steps == 0 {
            return match self.hashMap.get(&token.lexeme) {
                Some(v) => Ok(v.clone()),
                None => Err(BreakResult::Error(self.handle_error(
                    &format!("Resolved variable '{}' not found at the expected scope depth (resolver bug or stale locals map).", &token.lexeme),
                    token.line
                )))
            };
        }

        match &self.env_superior {
            Some(env) => env.borrow().get_at(token, steps - 1),
            None => Err(BreakResult::Error(self.handle_error(
                &format!("Resolver depth {} exceeds the available environment chain.", steps),
                token.line
            )))
        }
    }
    
    pub fn assign(&mut self, token: Token, value: Value) -> Result<Value, BreakResult> {
        if self.hashMap.contains_key(&token.lexeme) {
            self.hashMap.insert(token.lexeme.clone(), value);
            Ok(Value::None)
        } else {
            match &mut self.env_superior {
                Some(env) => env.borrow_mut().assign(token, value),
                None => Err(BreakResult::Error(self.handle_error(
                    &format!("Assignment to undefined variable '{}'. Declare it before assigning.", &token.lexeme),
                    token.line
                )))
            }
        }
    }

    fn handle_error(&self, msg: &str, line: i32) -> String {
        format!("[Line {}] Interpreter Error: {}", line, msg)
    }
}

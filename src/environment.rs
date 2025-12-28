use std::collections::HashMap;
use crate::ast::Value;
use crate::token::Token; 
use std::process; 
use std::rc::Rc; 
use std::cell::RefCell; 

//TODO get a deeper understanding of what rc and ref cells are doing 
pub struct Environment { 
    env_superior: Option<Rc<RefCell<Environment>>>, 
    hashMap: HashMap<String, Value>
}

impl Environment { 
    pub fn new(env_superior: Option<Rc<RefCell<Environment>>>) -> Self{
        Environment{
            env_superior: env_superior, 
            hashMap: HashMap::new()
        }
    }

    pub fn define(&mut self, var: Token, value:Value){
        if self.hashMap.contains_key(&var.lexeme){
            self.handle_error(&format!("Variable '{}' has already been declared", &var.lexeme), var.line);
        }
        self.hashMap.insert(var.lexeme.clone(), value); 
    }

    pub fn define_default(&mut self, name: &str, value:Value){
        //for default functions
        self.hashMap.insert(String::from(name), value); 
    }

    pub fn get(&self, token:Token) -> Option<Value>{
        let result = self.hashMap.get(&token.lexeme); 

        match result {
            Some(v) => {return Some(v.clone())} 
            _ => {}
        }

        match &self.env_superior { 
            Some(env) => {return env.borrow_mut().get(token)},
            None => {self.handle_error(&format!("Undenfined variable '{}' not found within any scope", token.lexeme), token.line);
                     return None}
        }
    }
    
    pub fn assign(&mut self, token: Token, value: Value){
        //dbg!(self.hashMap.contains_key(&token.lexeme));
        if self.hashMap.contains_key(&token.lexeme){
            
            let entry_value = self.hashMap.entry((&token.lexeme).to_string()).or_insert(Value::None);
            *entry_value = value; 

        } else {
            match &mut self.env_superior{
                Some(env) => {env.borrow_mut().assign(token, value);},
                None => {self.handle_error(&format!("Undenfined symbol assignment not found within any scope {}", &token.lexeme), token.line);}
            }
        }
    }

    fn handle_error(&self, msg: &str, line: i32) {

        eprintln!("[Line {}] Environment Error: {}", line, msg);
        process::exit(1);
    }
}

use std::collections::HashMap;
use crate::ast::Value;
use crate::ast::BreakResult;
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

    pub fn define(&mut self, var: Token, value:Value) -> Result<Value, BreakResult>{
        if self.hashMap.contains_key(&var.lexeme){
            return Err(BreakResult::Error(self.handle_error(&format!("Variable '{}' has already been declared", &var.lexeme), var.line)));
        }
        self.hashMap.insert(var.lexeme.clone(), value); 

        return Ok(Value::None);
    }

    pub fn define_default(&mut self, name: &str, value:Value){
        //for default functions
        self.hashMap.insert(String::from(name), value); 
    }

    fn retrieve(&self, token: Token) -> Option<Value>{
        let result = self.hashMap.get(&token.lexeme); 

        match result {
            Some(v) => {return Some(v.clone())} 
            _ => {}
        }

        match &self.env_superior { 
            Some(env) => {return env.borrow_mut().retrieve(token)},
            None => {return None}
        }
    }

    pub fn get(&self, token:Token) -> Result<Value, BreakResult>{

        let cloned_token = token.clone();

        match self.retrieve(token){
            Some(v) => {return Ok(v)}, 
            None => {return Err(BreakResult::Error(self.handle_error(&format!("Undenfined variable '{}' not found within current scope", cloned_token.lexeme), cloned_token.line)))}
        }
    }
    
    pub fn assign(&mut self, token: Token, value: Value)-> Result<Value, BreakResult>{
        //dbg!(self.hashMap.contains_key(&token.lexeme));
        if self.hashMap.contains_key(&token.lexeme){
            
            let entry_value = self.hashMap.entry((&token.lexeme).to_string()).or_insert(Value::None);
            *entry_value = value; 

        } else {
            match &mut self.env_superior{
                Some(env) => {env.borrow_mut().assign(token, value);},
                None => {return Err(BreakResult::Error(self.handle_error(&format!("Undenfined symbol assignment not found within current scope {}", &token.lexeme), token.line)));}
            }
        }

        return Ok(Value::None); 
    }

    // fn handle_error(&self, msg: &str, line: i32) {

    //     eprintln!("[Line {}] Environment Error: {}", line, msg);
    //     process::exit(1);
    // }

    fn handle_error(&self, msg: &str, line: i32) -> String{
        return format!("[Line {}] Interpreter Error: {}", line, msg);
    }
}

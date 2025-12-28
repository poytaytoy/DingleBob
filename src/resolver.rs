use crate::interpreter::Interpreter; 

pub struct Resolver {
    interpreter: Interpreter
}

impl Resolver {

    pub fn new(interpreter: Interpreter) -> Self {
        Resolver { interpreter }
    }
    
    
}
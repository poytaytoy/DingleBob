const ADD:i32 = 1;
const MINUS: i32 = 2; 
const EQUAL: i32 = 3; 
const NOT_EQUAL: i32 = 4;
const IDENTIFIER: i32 = 5; 
const NUMBER: i32 = 6;   

pub struct Token {
    key: i32, 
    text: String, 
}

pub struct Scanner {
    curr_input: String,
    curr_index: i32,
    token_list: Vec<Token>,
}

impl Scanner {
    pub fn new () -> Self {
        Scanner {
            curr_input: String::new(), 
            curr_index: 0, 
            token_list: Vec::new() 
        }
    }
    
    pub fn insert (&mut self, input: &str) {
        self.curr_input = String::from(input); 
        self.curr_index = 0; 
    }

    fn peak() -> u8{
        
    }

    fn convert(&mut self){

    }

}
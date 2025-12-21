use std::str::Chars;

#[derive(Debug)]
enum TokenKind {
    EQUAL, 
    PLUS, 
    MULTIPLY, 
    MINUS, 
    DIVIDE, 
    FLOAT(f32), 
    INT(i32), 
    IDENTIFIER(String),
    STRING(String),  
    UNKNOWN(String)
}

pub struct Scanner<'a> {
    curr_input: Chars<'a>,
    token_list: Vec<TokenKind>,
}

impl <'a> Scanner<'a> {
    pub fn new (input: &'a str) -> Self {
        Scanner {
            curr_input: input.chars(),
            token_list: Vec::new(),
        }
    }   

    fn peak(&self) -> Option<char>{
        self.curr_input.clone().next()    
    }

    fn peakpeak(&self) -> Option<char>{
        let mut clone = self.curr_input.clone();
        clone.next();
        return clone.next();    
    }

    fn handle_digits(&mut self, initial_digit_char:char){
        let mut number_store = String::new(); 

        number_store.push(initial_digit_char); 

        loop{
            if let Some(c) = self.peak() {
            
                if c.is_numeric() || c == '.'{
                    number_store.push(c);
                } else {
                    break; 
                }
                self.curr_input.next(); 
            }
            else{
                break; 
            }            
        }

        let mut resulti = number_store.parse::<i32>(); 
        let mut resultf = number_store.parse::<f32>(); 

        if let Ok(num) = resulti {
            self.token_list.push(TokenKind::INT(num));
        } else if let Ok(num) = resultf{
            self.token_list.push(TokenKind::FLOAT(num));
        } else{
            self.token_list.push(TokenKind::UNKNOWN(number_store));
        }

    }

    fn handle_strings(&mut self){
        let mut string_store = String::new(); 
        let mut met_end = false; 

        loop{
            if let Some(c) = self.peak() {
            
                if c != '"'{
                    string_store.push(c);
                    self.curr_input.next();
                } else {
                    met_end = true; 
                    self.curr_input.next();
                    break; 
                }
            }
            else{
                break; 
            }            
        }

        if met_end {
            self.token_list.push(TokenKind::STRING(string_store))
        } else {
            self.token_list.push(TokenKind::UNKNOWN(string_store))
        }
        
        
    }

    fn handle_identifier(&mut self, initial_alpha_char: char){
        let mut identifier_store = String::new(); 
        identifier_store.push(initial_alpha_char); 

        loop{
            if let Some(c) = self.peak() {
                if c.is_alphabetic() || c.is_numeric() || c == '_'{
                    identifier_store.push(c);
                    
                } else {
                    break; 
                }
                self.curr_input.next();
            }
            else{
                break; 
            }            
        }        

        self.token_list.push(TokenKind::IDENTIFIER(identifier_store));
    }

    fn convert(&mut self){

        loop{
            let mut curr_char: char; 

            if let Some(c) = self.curr_input.next() {
                curr_char = c; 
            } else {
                break; 
            } 

            match curr_char {
                '+' => self.token_list.push(TokenKind::PLUS),
                '-' => self.token_list.push(TokenKind::MINUS),
                '=' => self.token_list.push(TokenKind::EQUAL),
                '*' => self.token_list.push(TokenKind::MULTIPLY),
                '/' => self.token_list.push(TokenKind::DIVIDE),
                ' ' | '\n' => continue,
                '"' => self.handle_strings(),
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'  => self.handle_digits(curr_char),
                _ => {
                    if curr_char.is_alphabetic(){
                        self.handle_identifier(curr_char);
                    } else {
                        self.token_list.push(TokenKind::UNKNOWN(String::from(curr_char)))
                    }
                },
            }

            
        }
    }

    pub fn output(self) -> Vec<TokenKind>{
        
        self.token_list
    }

    pub fn debug(&mut self){
        self.convert(); 

        for item in &self.token_list{
            println!("{:?}", item); 
        }

    }

}
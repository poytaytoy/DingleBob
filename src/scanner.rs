use std::str::Chars;


use crate::token; 
use token::TokenKind;

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

    // fn peakpeak(&self) -> Option<char>{
    //     let mut clone = self.curr_input.clone();
    //     clone.next();
    //     return clone.next();    
    // }

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

        let resulti = number_store.parse::<i32>(); 
        let resultf = number_store.parse::<f32>(); 

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

        self.token_list.push(
            match identifier_store.as_str() {
                "and" => TokenKind::AND,
                "class" => TokenKind::CLASS,
                "else" => TokenKind::ELSE,
                "false" => TokenKind::FALSE,
                "for" => TokenKind::FOR,
                "if" => TokenKind::IF, 
                "none" => TokenKind::NONE,
                "or" => TokenKind::OR,
                "print" => TokenKind::PRINT,
                "return" => TokenKind::RETURN,
                "super" => TokenKind::SUPER,
                "this" => TokenKind::THIS, 
                "true" => TokenKind::TRUE,  
                "var" => TokenKind::VAR, 
                "while" => TokenKind::WHILE, 
                _ => TokenKind::IDENTIFIER(identifier_store)
            }
        );
    }

    fn handle_comment(&mut self){
        loop{
            if let Some(c) = self.peak() {
                self.curr_input.next();
                if c == '\n' {
                    break;
                }
            }
            else{
                break; 
            }

        }
    }

    fn handle_equal(&mut self, nonequal: TokenKind, equal: TokenKind){

        if let Some(c) = self.peak(){
            if c == '='{
                self.token_list.push(equal);
                self.curr_input.next(); 
            }
            else {
                self.token_list.push(nonequal);
            }
        } else{
            self.token_list.push(nonequal)
        }
    }

    pub fn convert(&mut self){

        loop{
            let curr_char: char; 

            if let Some(c) = self.curr_input.next() {
                curr_char = c; 
            } else {
                self.token_list.push(TokenKind::EOF);
                break; 
            } 

            match curr_char {
                '+' => self.token_list.push(TokenKind::PLUS),
                '-' => self.token_list.push(TokenKind::MINUS),
                '*' => self.token_list.push(TokenKind::STAR),
                '/' => self.token_list.push(TokenKind::SLASH),
                ';' => self.token_list.push(TokenKind::SEMICOLON),
                ',' => self.token_list.push(TokenKind::COMMA),
                '.' => self.token_list.push(TokenKind::DOT),
                '{' => self.token_list.push(TokenKind::LEFT_BRACE),
                '}' => self.token_list.push(TokenKind::RIGHT_BRACE),
                '(' => self.token_list.push(TokenKind::LEFT_PAREN),
                ')' => self.token_list.push(TokenKind::RIGHT_PAREN),
                '#' => self.handle_comment(),
                ' ' | '\n' | '\t' => continue,
                '=' => self.handle_equal(TokenKind::EQUAL, TokenKind::EQUAL_EQUAL),
                '!' => self.handle_equal(TokenKind::BANG, TokenKind::BANG_EQUAL),
                '>' => self.handle_equal(TokenKind::GREATER, TokenKind::GREATER_EQUAL),
                '<' => self.handle_equal(TokenKind::LESS, TokenKind::LESS_EQUAL),
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

        for item in &self.token_list{
            println!("{:?}", item); 
        }

    }

}
use crate::token::Token;
use crate::token::TokenKind;
use std::str::Chars;
use std::process; 

struct Scanner<'a> {
    curr_input: Chars<'a>,
    token_list: Vec<Token>,
    line: i32, 
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Scanner {
            curr_input: input.chars(),
            token_list: Vec::new(),
            line: 1,
        }
    }
    fn handle_error(&self, msg: &str) {
        eprintln!("[Line {}] Scanner Error: {}", self.line, msg);
        process::exit(1);
    }

    fn add_token(&mut self, kind: TokenKind, lexeme: String) {
        self.token_list.push(Token {
            kind,
            lexeme,
            line: self.line,
        });
    }

    fn peak(&self) -> Option<char> {
        self.curr_input.clone().next()
    }

    fn handle_digits(&mut self, initial_digit_char: char) {
        let mut lexeme = String::from(initial_digit_char);

        while let Some(c) = self.peak() {
            if c.is_numeric() || c == '.' {
                lexeme.push(c);
                self.curr_input.next();
            } else {
                break;
            }
        }

        self.add_token(TokenKind::NUMBER, lexeme);

        // if lexeme.parse::<f32>().is_ok() {
            
        // } else {
        //     self.handle_error(&format!("Invalid numeric literal: {}", lexeme));
        // }
    }

    fn handle_strings(&mut self) {
        let mut string_content = String::new();
        let mut met_end = false;

        while let Some(c) = self.peak() {
            if c != '"' {
                if c == '\n' { self.line += 1; }
                string_content.push(c);
                self.curr_input.next();
            } else {
                met_end = true;
                self.curr_input.next(); 
                break;
            }
        }

        if met_end {
            let lexeme = string_content;
            self.add_token(TokenKind::STRING, lexeme);
        } else {
            self.handle_error("Unterminated string.");
        }
    }

    fn handle_identifier(&mut self, initial_alpha_char: char) {
        let mut lexeme = String::from(initial_alpha_char);

        while let Some(c) = self.peak() {
            if c.is_alphabetic() || c.is_numeric() || c == '_' {
                lexeme.push(c);
                self.curr_input.next();
            } else {
                break;
            }
        }

        let kind = match lexeme.as_str() {
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
            _ => TokenKind::IDENTIFIER,
        };
        
        self.add_token(kind, lexeme);
    }

    fn handle_comment(&mut self) {
        while let Some(c) = self.peak() {
            self.curr_input.next();
            if c == '\n' {
                self.line += 1;
                break;
            }
        }
    }

    fn handle_equal(&mut self, nonequal: TokenKind, equal: TokenKind, base_char: char) {
        if let Some(c) = self.peak() {
            if c == '=' {
                self.curr_input.next();
                self.add_token(equal, format!("{}=", base_char));
            } else {
                self.add_token(nonequal, base_char.to_string());
            }
        } else {
            self.add_token(nonequal, base_char.to_string());
        }
    }

    pub fn convert(&mut self) {
        loop {
            let curr_char = match self.curr_input.next() {
                Some(c) => c,
                None => {
                    self.add_token(TokenKind::EOF, String::new());
                    break;
                }
            };

            match curr_char {
                '+' => self.add_token(TokenKind::PLUS, String::from("+")),
                '-' => self.add_token(TokenKind::MINUS, String::from("-")),
                '*' => self.add_token(TokenKind::STAR, String::from("*")),
                '/' => self.add_token(TokenKind::SLASH, String::from("/")),
                ';' => self.add_token(TokenKind::SEMICOLON, String::from(";")),
                ',' => self.add_token(TokenKind::COMMA, String::from(",")),
                '.' => self.add_token(TokenKind::DOT, String::from(".")),
                '{' => self.add_token(TokenKind::LEFT_BRACE, String::from("{")),
                '}' => self.add_token(TokenKind::RIGHT_BRACE, String::from("}")),
                '(' => self.add_token(TokenKind::LEFT_PAREN, String::from("(")),
                ')' => self.add_token(TokenKind::RIGHT_PAREN, String::from(")")),
                
                '#' => self.handle_comment(),
                ' ' | '\r' | '\t' => continue,
                '\n' => self.line += 1,

                '=' => self.handle_equal(TokenKind::EQUAL, TokenKind::EQUAL_EQUAL, '='),
                '!' => self.handle_equal(TokenKind::BANG, TokenKind::BANG_EQUAL, '!'),
                '>' => self.handle_equal(TokenKind::GREATER, TokenKind::GREATER_EQUAL, '>'),
                '<' => self.handle_equal(TokenKind::LESS, TokenKind::LESS_EQUAL, '<'),

                '"' => self.handle_strings(),
                '0'..='9' => self.handle_digits(curr_char),
                _ => {
                    if curr_char.is_alphabetic() || curr_char == '_' {
                        self.handle_identifier(curr_char);
                    } else {
                        self.handle_error(&format!("Unexpected character '{}'", curr_char));
                    }
                }
            }
        }
    }

    pub fn debug(&mut self) {
        for item in &self.token_list{
            println!("{:?}", item)
        }
    }

    pub fn output(self) -> Vec<Token> {
        self.token_list
    }
}

pub fn scan(contents: &str, debug: bool) -> Vec<Token>{
    let mut scan = Scanner::new(&contents); 
    scan.convert(); 

    if debug{
        scan.debug();
    }
     
    scan.output()
}

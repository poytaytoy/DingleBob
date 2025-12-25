use crate::token::TokenKind;
use crate::token::Token; 
use crate::ast::Expression; 
use crate::ast::Value;
use crate::ast::Statement; 
use std::process; 

pub struct Parser {
    tokens_list: Vec<Token>,
    curr_index: usize, 
}

impl Parser{

    pub fn new(tokens_list: Vec<Token>) -> Self{
        Parser{
            tokens_list: tokens_list, 
            curr_index: 0,
        }
    }

    fn atEnd(&self) -> bool{ 
        (&self.tokens_list[self.curr_index]).kind == TokenKind::EOF
    }

    pub fn parse(&mut self) -> Vec<Statement> {

        let mut statement_list = Vec::new(); 
        
        while !(self.atEnd()) {
            statement_list.push(self.statement());
        }

        return statement_list; 
    }

    fn declaration(&mut self) -> Statement{ 

        if self.match_token(&[TokenKind::LET]){
            return self.varDeclaration(); 
        }

        return self.statement();
    }

    fn varDeclaration(&mut self) -> Statement{
        
        if self.check(TokenKind::IDENTIFIER){
            let token: Token = &self.tokens_list[self.curr_index];
            self.curr_index += 1; 

            if self.match_token(&[TokenKind::EQUAL]){
                let expr = self.expression(); 

                if self.check(TokenKind::SEMICOLON){
                    self.curr_index += 1; 

                    return Statement::Declaration(token.lexeme.clone(), expr)
                }

                self.handle_error("Expect ';' after variable declaration");
            }

            self.handle_error("Expect '=' after variable declaration");
        }

        self.handle_error("Expect variable name after declaration"); 

        Statement::Declaration(String::from("_"), Expression::Literal(Value::None)) 
    }

    fn statement(&mut self) -> Statement{

        if (self.match_token(&[TokenKind::PRINT])){
            return self.printStatement(); 
        }

        return self.printStatement(); //return self.expressionStatement();
    }

    fn printStatement(&mut self) -> Statement {

        let expr = self.expression(); 

        if self.check(TokenKind::SEMICOLON) {
            self.curr_index += 1; 
        } else {
            self.handle_error("Expect ';' after end of expression");
        }

        return Statement::Print(expr); 
    }


    fn expression(&mut self) -> Expression{
        return self.equality(); 
    }

    fn equality(&mut self) -> Expression{

        let mut expr: Expression  = self.comparison();

        let type_list = [TokenKind::BANG_EQUAL, TokenKind::EQUAL_EQUAL];

        while (self.match_token(&type_list)){
            let operator: Token  = (&self.tokens_list[self.curr_index - 1]).clone();
            let right: Expression  = self.comparison(); 

            expr = Expression::Binary(Box::new(expr), operator, Box::new(right)); 
        }

        return expr; 
    }

    fn comparison(&mut self) -> Expression{

        let mut expr: Expression  = self.term();

        let type_list = [TokenKind::GREATER, TokenKind::GREATER_EQUAL, TokenKind::LESS, TokenKind::LESS_EQUAL];

        while (self.match_token(&type_list)){
            let operator: Token  = (&self.tokens_list[self.curr_index - 1]).clone();
            let right: Expression  = self.term(); 

            expr = Expression::Binary(Box::new(expr), operator, Box::new(right)); 
        }

        return expr; 
    }

    fn term(&mut self) -> Expression{

        let mut expr: Expression  = self.factor();

        let type_list = [TokenKind::MINUS, TokenKind::PLUS];

        while (self.match_token(&type_list)){
            let operator: Token  = (&self.tokens_list[self.curr_index - 1]).clone();
            let right: Expression  = self.factor(); 

            expr = Expression::Binary(Box::new(expr), operator, Box::new(right)); 
        }

        return expr; 
    }

    fn factor(&mut self) -> Expression{

        let mut expr: Expression  = self.unary();

        let type_list = [TokenKind::SLASH, TokenKind::STAR];

        while (self.match_token(&type_list)){
            let operator: Token  = (&self.tokens_list[self.curr_index - 1]).clone();
            let right: Expression  = self.unary(); 

            expr = Expression::Binary(Box::new(expr), operator, Box::new(right)); 
        }

        return expr; 
    }

    fn unary(&mut self) -> Expression{

        let type_list = [TokenKind::BANG, TokenKind::MINUS];

        while (self.match_token(&type_list)){
            let operator: Token  = (&self.tokens_list[self.curr_index - 1]).clone();
            let right: Expression  = self.unary(); 

            return Expression::Unary(operator, Box::new(right)); 
        }

        return self.primary(); 
    }

    fn primary(&mut self)->Expression{

        if (self.atEnd()){
            self.handle_error("Invalid syntax at end of file");
        }

        let literal: &Token = &self.tokens_list[self.curr_index]; 

        self.curr_index += 1;

        match &literal.kind {
            TokenKind::FALSE => return Expression::Literal(Value::Bool(false)),
            TokenKind::TRUE => return Expression::Literal(Value::Bool(true)),
            TokenKind::NONE => return Expression::Literal(Value::None),
            TokenKind::STRING => return Expression::Literal(Value::String(literal.lexeme.clone())),
            TokenKind::NUMBER => {
                let mut resulti = literal.lexeme.parse::<i128>(); 
                let mut resultf = literal.lexeme.parse::<f64>(); 

                if resulti.is_ok() {
                    return Expression::Literal(Value::Int(resulti.unwrap()));
                } else if resultf.is_ok() { 
                    return Expression::Literal(Value::Float(resultf.unwrap()));
                } else {
                    self.handle_error(&format!("Invalid numeral with {}", &self.tokens_list[self.curr_index].lexeme));
                }
            }
            TokenKind::IDENTIFIER => return Expression::Variable(literal.clone()),
            _=> {self.curr_index -= 1}
        }

        if self.match_token(&[TokenKind::LEFT_PAREN]){
            let expression:Expression = self.expression();

            if self.match_token(&[TokenKind::RIGHT_PAREN]){
                return Expression::Grouping(Box::new(expression)); 
            }
            
            self.handle_error("Expected ')' after expression.");

        }

        self.handle_error(&format!("Invalid syntax with {}", &self.tokens_list[self.curr_index].lexeme));

        return Expression::Literal(Value::None);
    }



    fn advance(&mut self){
        if !(self.atEnd()) {self.curr_index += 1}
    }


    fn check(&mut self, kind:TokenKind) -> bool{
        if (self.atEnd()) {return false;} 

        (&self.tokens_list[self.curr_index]).kind == kind
    }

    fn match_token(&mut self, args: &[TokenKind]) -> bool{

        if (self.atEnd()){
            return false; 
        }

        for tokenKind in args{
            if self.check(*tokenKind){
                self.curr_index += 1; 
                return true; 
            }
        }

        false
    }

    fn handle_error(&self, msg: &str) {

        let line = &self.tokens_list[self.curr_index].line;

        eprintln!("[Line {}] Parser Error: {}", line, msg);
        process::exit(1);
    }
}
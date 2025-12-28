use crate::token::TokenKind;
use crate::token::Token; 
use crate::ast::Expression; 
use crate::ast::Value;
use crate::ast::Statement; 
use std::process; 
use std::mem;
use std::thread::current;
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
            let dec = self.declaration();
            //dbg!(&dec);
            statement_list.push(dec);
        }

        return statement_list; 
    }

    fn declaration(&mut self) -> Statement{ 

        let currentToken = &self.tokens_list[self.curr_index]; 
        self.curr_index += 1; 

        match currentToken.kind{
            TokenKind::LET => return self.varDeclaration(), 
            _ => {self.curr_index -= 1}   
        }

        return self.statement();
    }

    fn varDeclaration(&mut self) -> Statement{
        
        if self.check(TokenKind::IDENTIFIER){
            let token: Token = (&self.tokens_list[self.curr_index]).clone();
            self.curr_index += 1; 

            if self.match_token(&[TokenKind::EQUAL]){
                let expr = self.expression(); 

                if self.check(TokenKind::SEMICOLON){
                    self.curr_index += 1; 

                    return Statement::Var(token, expr)
                }

                self.handle_error("Expect ';' after variable declaration");
            }

            if self.check(TokenKind::SEMICOLON){
                self.curr_index += 1; 
                return Statement::Var(token, Expression::Literal(Value::None));
            }

            self.handle_error("Expect '=' after variable declaration");
        }

        self.handle_error("Expect variable name after declaration"); 

        unreachable!();
    }

    fn statement(&mut self) -> Statement{

        let currentToken = &self.tokens_list[self.curr_index]; 
        self.curr_index += 1; 

        match currentToken.kind{
            TokenKind::FOR => return self.forStatement(), 
            TokenKind::IF => return self.ifStatement(), 
            TokenKind::PRINT => return self.printStatement(), 
            TokenKind::WHILE => return self.whileStatement(), 
            TokenKind::BREAK => {
                let statement = Statement::Break(currentToken.clone()); 
                if !self.check(TokenKind::SEMICOLON){
                    self.handle_error("';' is expected after break");
                }
                self.curr_index += 1; 
                return statement; 
            }, 
            TokenKind::LEFT_BRACE => return self.block(),
            _ => {self.curr_index -= 1}   
        }

       return self.expressionStatement();
    }

    fn forStatement(&mut self) -> Statement{
        if !self.check(TokenKind::LEFT_PAREN) {
            self.handle_error("Expect '(' after 'for' ");
        }

        self.curr_index += 1; 

        let mut intializer: Option<Statement>;
        if self.check(TokenKind::SEMICOLON){
            self.curr_index += 1; 
            //idk what to do for the none case
            intializer = None; 
        } else if self.check(TokenKind::LET)  {
            self.curr_index += 1; 
            intializer = Some(self.varDeclaration());
        } else {
            intializer = Some(self.expressionStatement());
        }

        let mut condition = None; 
        if !(self.check(TokenKind::SEMICOLON)){
            condition = Some(self.expression()); 
        }
        if !(self.check(TokenKind::SEMICOLON)){
           self.handle_error("Expect ';' after loop condition");
        }

        self.curr_index += 1; 

        let mut increment = None; 
        if !(self.check(TokenKind::RIGHT_PAREN)){
            increment = Some(self.expression()); 
        }

        if !(self.check(TokenKind::RIGHT_PAREN)){
            self.handle_error("Expect ')' after for clauses");
        }

        self.curr_index += 1; 

        let mut body = self.statement(); 

        if !matches!(body, Statement::Block(_)){
            self.handle_error("Expect a scope '{ }' after 'for(....)'");
        }

        if !(increment.is_none()) {

            let mut s_array = Vec::new(); 
            s_array.push(body); 
            s_array.push(Statement::Expression(increment.unwrap()));

            body = Statement::Block(Box::new(s_array));
        }

        if condition.is_none(){
            condition = Some(Expression::Literal(Value::Bool(true)));
        }

        body = Statement::While(condition.unwrap(), Box::new(body));

        if !(intializer.is_none()){

            let mut s_array = Vec::new(); 
            s_array.push(intializer.unwrap()); 
            s_array.push(body); 

            body = Statement::Block(Box::new(s_array));
        }

        return body; 
    }

    fn ifStatement(&mut self) -> Statement{ 
        let expr = self.expression(); 
        let thenStatement = self.statement();

        if !(matches!(thenStatement, Statement::Block(_))){
            self.handle_error("Expected a scope following 'if'");
        }

        let mut elseStatement = Statement::Expression(Expression::Literal(Value::None));

        if self.check(TokenKind::ELSE){
            self.curr_index += 1; 
            elseStatement = self.statement();

            if !(matches!(elseStatement, Statement::Block(_))){
                self.handle_error("Expected a scope following 'else'");
            }
        }

        return Statement::If(expr, Box::new(thenStatement), Box::new(elseStatement));   

    }

    fn printStatement(&mut self) -> Statement {

        let expr = self.expression(); 

        if self.check(TokenKind::SEMICOLON) {
            self.curr_index += 1; 
        } else {
            self.handle_error("Expect ';' after end of expression in a print statement");
        }

        return Statement::Print(expr); 
    }

    fn whileStatement(&mut self) -> Statement {

        let expr = self.expression(); 
        let statement = self.statement(); 

        if !matches!(statement, Statement::Block(_)){
            self.handle_error("Expect a scope after declaring 'while'");
        }

        Statement::While(expr, Box::new(statement))
    }

    fn expressionStatement(&mut self) -> Statement {

        let expr = self.expression(); 

        if self.check(TokenKind::SEMICOLON) {
            self.curr_index += 1; 
        } else {
            self.handle_error("Expect ';' after end of expression");
        }

        return Statement::Expression(expr); 
    }

    fn block(&mut self) -> Statement{

        let mut statement = Vec::new(); 

        while !(self.check(TokenKind::RIGHT_BRACE)) && !self.atEnd(){
            statement.push(self.declaration()); 
        }

        if (self.check(TokenKind::RIGHT_BRACE)){
            self.curr_index += 1; 
            return Statement::Block(Box::new(statement)); 
        }
        
        self.handle_error("Expected '}' after end of block");
        Statement::Block(Box::new(statement))
    }

    fn expression(&mut self) -> Expression{
        return self.assignment(); 
    }

    fn assignment(&mut self) -> Expression{ 
        let expr = self.or(); 

        if self.check(TokenKind::EQUAL){
            self.curr_index += 1;
            let mut value = self.expression(); 

            if matches!(expr, Expression::Variable(_)) {
                let Expression::Variable(token)= expr else {
                    unreachable!(); 
                }; 

                return Expression::Assign(token, Box::new(value)); 
            }

            self.handle_error("Invalid assignment target");
        }

        return expr; 
    }

    fn or(&mut self) -> Expression{
        let mut expr = self.and(); 
        
        while self.check(TokenKind::OR){
            let op = (&self.tokens_list[self.curr_index]).clone(); 
            self.curr_index += 1; 
            let right = self.equality(); 

            expr = Expression::Logical(Box::new(expr), op, Box::new(right)); 
        }

        return expr; 
    }

    fn and(&mut self) -> Expression{
        let mut expr = self.equality(); 

        while self.check(TokenKind::AND){
            let op = (&self.tokens_list[self.curr_index]).clone(); 
            self.curr_index += 1; 
            let right = self.equality(); 

            expr = Expression::Logical(Box::new(expr), op, Box::new(right)); 
        }

        return expr; 
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

        let type_list = [TokenKind::SLASH, TokenKind::STAR, TokenKind::PERCENT];

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

        return self.call(); 
    }

    fn call(&mut self) -> Expression {
        let mut expr = self.primary(); 

        loop{
            if !self.check(TokenKind::LEFT_PAREN){
                break;
            }

            let mut args_list: Vec<Expression> = Vec::new(); 

            self.curr_index += 1;

            if !self.check(TokenKind::RIGHT_PAREN){
                args_list.push(self.expression()); 

                while self.check(TokenKind::COMMA){
                    self.curr_index += 1; 
                    args_list.push(self.expression());
                }
            }

            if self.check(TokenKind::RIGHT_PAREN){
                if args_list.len() > 255 {self.handle_error("You can have no more than 255 arguments")};
                expr = Expression::Call(Box::new(expr), (&self.tokens_list[self.curr_index]).clone(), Box::new(args_list));
                self.curr_index += 1; 
            } else {
                self.handle_error("Expected ')' after arguments");
            }

        }

        return expr; 
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
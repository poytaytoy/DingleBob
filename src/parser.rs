use crate::func::Func;
use crate::token::TokenKind;
use crate::token::Token; 
use crate::ast::Expression; 
use crate::ast::Value;
use crate::ast::Statement; 
use std::env::args;
use std::ops::Index;
use std::mem;

type ParseResult<T> = Result<T, String>;

pub struct Parser {
    tokens_list: Vec<Token>,
    curr_index: usize, 
    var_id: i32,
}

impl Parser{

    pub fn new(tokens_list: Vec<Token>) -> Self{
        Parser{
            tokens_list: tokens_list, 
            curr_index: 0,
            var_id: 0,
        }
    }

    fn atEnd(&self) -> bool{ 
        (&self.tokens_list[self.curr_index]).kind == TokenKind::EOF
    }

    pub fn parse(&mut self) -> ParseResult<Vec<Statement>> {

        let mut statement_list = Vec::new(); 
        
        while !self.atEnd() {
            let dec = self.declaration()?;
            statement_list.push(dec);
        }

        Ok(statement_list)
    }

    fn declaration(&mut self) -> ParseResult<Statement>{ 

        let currentToken = &self.tokens_list[self.curr_index]; 
        self.curr_index += 1; 

        match currentToken.kind{
            TokenKind::LET => return self.varDeclaration(), 
            TokenKind::DEFINE => return self.function(), 
            _ => {self.curr_index -= 1}   
        }

        self.statement()
    }

    fn varDeclaration(&mut self) -> ParseResult<Statement>{
        
        if self.check(TokenKind::IDENTIFIER){
            let token: Token = (&self.tokens_list[self.curr_index]).clone();
            self.curr_index += 1; 

            if self.match_token(&[TokenKind::EQUAL]){
                let expr = self.expression()?; 

                if self.check(TokenKind::SEMICOLON){
                    self.curr_index += 1; 
                    return Ok(Statement::Var(token, expr));
                }

                return self.handle_error("Expected ';' after variable declaration.");
            }

            if self.check(TokenKind::SEMICOLON){
                self.curr_index += 1; 
                return Ok(Statement::Var(token, Expression::Literal(Value::None)));
            }

            return self.handle_error("Expected '=' or ';' after variable name in variable declaration.");
        }

        self.handle_error("Expected an identifier after 'let' (variable name).")
    }

    fn function(&mut self) -> ParseResult<Statement> {
        
        if !self.check(TokenKind::IDENTIFIER){
            return self.handle_error("Expected an identifier after 'define' (function name).");
        }

        let name = (&self.tokens_list[self.curr_index]).clone(); 
        self.curr_index += 1; 

        if !self.check(TokenKind::LEFT_PAREN){
            return self.handle_error("Expected '(' after function name in function declaration.");
        }

        let mut args_list: Vec<Token> = Vec::new();

        self.curr_index += 1;

        if !self.check(TokenKind::RIGHT_PAREN){
            loop {
                if !self.check(TokenKind::IDENTIFIER){
                    return self.handle_error("Expected an identifier as a parameter name in function declaration.");
                }

                args_list.push((&self.tokens_list[self.curr_index]).clone()); 

                self.curr_index += 1; 

                if !self.check(TokenKind::COMMA){
                    break;
                }

                self.curr_index += 1; 
            }
        }

        if self.check(TokenKind::RIGHT_PAREN){
            if args_list.len() > 255 {
                return self.handle_error("Too many parameters: functions can have at most 255 parameters.");
            }
            self.curr_index += 1; 
        } else {
            return self.handle_error("Expected ')' after parameter list.");
        } 

        if !self.check(TokenKind::LEFT_BRACE){
            return self.handle_error("Expected '{' to start function body.");
        }

        self.curr_index += 1;

        let Statement::Block(statements) = self.block()? else {
            unreachable!()
        };

        Ok(Statement::Function(name, args_list, statements))
    }

    fn statement(&mut self) -> ParseResult<Statement>{

        let currentToken = &self.tokens_list[self.curr_index]; 
        self.curr_index += 1; 

        match currentToken.kind{
            TokenKind::FOR => return self.forStatement(), 
            TokenKind::IF => return self.ifStatement(), 
            TokenKind::PRINT => return self.printStatement(), 
            TokenKind::RETURN => return self.returnStatement(),
            TokenKind::WHILE => return self.whileStatement(), 
            TokenKind::BREAK => {
                let statement = Statement::Break(currentToken.clone()); 
                if !self.check(TokenKind::SEMICOLON){
                    return self.handle_error("Expected ';' after 'break'.");
                }
                self.curr_index += 1; 
                return Ok(statement);
            }, 
            TokenKind::LEFT_BRACE => return self.block(),
            _ => {self.curr_index -= 1}   
        }

        self.expressionStatement()
    }

    fn forStatement(&mut self) -> ParseResult<Statement>{
        if !self.check(TokenKind::LEFT_PAREN) {
            return self.handle_error("Expected '(' after 'for'.");
        }

        self.curr_index += 1; 

        let intializer: Option<Statement>;
        if self.check(TokenKind::SEMICOLON){
            self.curr_index += 1; 
            intializer = None; 
        } else if self.check(TokenKind::LET)  {
            self.curr_index += 1; 
            intializer = Some(self.varDeclaration()?);
        } else {
            intializer = Some(self.expressionStatement()?);
        }

        let mut condition = None; 
        if !self.check(TokenKind::SEMICOLON){
            condition = Some(self.expression()?); 
        }
        if !self.check(TokenKind::SEMICOLON){
            return self.handle_error("Expected ';' after loop condition in 'for' statement.");
        }

        self.curr_index += 1; 

        let mut increment = None; 
        if !self.check(TokenKind::RIGHT_PAREN){
            increment = Some(self.expression()?); 
        }

        if !self.check(TokenKind::RIGHT_PAREN){
            return self.handle_error("Expected ')' after for-clause list.");
        }

        self.curr_index += 1; 

        let mut body = self.statement()?; 

        if !matches!(body, Statement::Block(_)){
            return self.handle_error("Expected a block '{ ... }' after 'for (...)'.");
        }

        if increment.is_some() {
            let mut s_array = Vec::new(); 
            s_array.push(body); 
            s_array.push(Statement::Expression(increment.unwrap()));
            body = Statement::Block(Box::new(s_array));
        }

        let cond = condition.unwrap_or(Expression::Literal(Value::Bool(true)));
        body = Statement::While(cond, Box::new(body));

        if intializer.is_some() {
            let mut s_array = Vec::new(); 
            s_array.push(intializer.unwrap()); 
            s_array.push(body); 
            body = Statement::Block(Box::new(s_array));
        }

        Ok(body)
    }

    fn ifStatement(&mut self) -> ParseResult<Statement>{ 
        let expr = self.expression()?; 
        let thenStatement = self.statement()?;

        if !matches!(thenStatement, Statement::Block(_)){
            return self.handle_error("Expected a block '{ ... }' after 'if' condition.");
        }

        let mut elseStatement = Statement::Expression(Expression::Literal(Value::None));

        if self.check(TokenKind::ELSE){
            self.curr_index += 1; 
            elseStatement = self.statement()?;

            if !matches!(elseStatement, Statement::Block(_)){
                return self.handle_error("Expected a block '{ ... }' after 'else'.");
            }
        }

        Ok(Statement::If(expr, Box::new(thenStatement), Box::new(elseStatement)))
    }

    fn printStatement(&mut self) -> ParseResult<Statement> {

        let expr = self.expression()?; 

        if self.check(TokenKind::SEMICOLON) {
            self.curr_index += 1; 
        } else {
            return self.handle_error("Expected ';' after expression in 'print' statement.");
        }

        Ok(Statement::Print(expr))
    }

    fn returnStatement(&mut self) -> ParseResult<Statement> {
        let return_token = (&self.tokens_list[self.curr_index]).clone(); 
        let mut value = Expression::Literal(Value::None);

        if !self.check(TokenKind::SEMICOLON){
            value = self.expression()?; 
        }

        if !self.check(TokenKind::SEMICOLON){
            return self.handle_error("Expected ';' after return statement.");
        }

        self.curr_index += 1; 

        Ok(Statement::Return(return_token, value))
    }   

    fn whileStatement(&mut self) -> ParseResult<Statement> {

        let expr = self.expression()?; 
        let statement = self.statement()?; 

        if !matches!(statement, Statement::Block(_)){
            return self.handle_error("Expected a block '{ ... }' after 'while' condition.");
        }

        Ok(Statement::While(expr, Box::new(statement)))
    }

    fn expressionStatement(&mut self) -> ParseResult<Statement> {

        let expr = self.expression()?; 

        if self.check(TokenKind::SEMICOLON) {
            self.curr_index += 1; 
        } else {
            return self.handle_error("Expected ';' after expression.");
        }

        Ok(Statement::Expression(expr))
    }

    fn block(&mut self) -> ParseResult<Statement>{

        let mut statement = Vec::new(); 

        while !self.check(TokenKind::RIGHT_BRACE) && !self.atEnd(){
            statement.push(self.declaration()?); 
        }

        if self.check(TokenKind::RIGHT_BRACE){
            self.curr_index += 1; 
            return Ok(Statement::Block(Box::new(statement))); 
        }
        
        self.handle_error("Expected '}' to close block.")
    }

    fn expression(&mut self) -> ParseResult<Expression>{
        self.assignment()
    }

    fn assignment(&mut self) -> ParseResult<Expression>{ 
        let expr = self.or()?; 

        if self.check(TokenKind::EQUAL){
            let equal_store = (&self.tokens_list[self.curr_index]).clone();
            self.curr_index += 1;
            let value = self.expression()?; 
            return Ok(Expression::Assign(Box::new(expr), equal_store, Box::new(value)));
        }

        Ok(expr)
    }

    fn or(&mut self) -> ParseResult<Expression>{
        let mut expr = self.and()?; 
        
        while self.check(TokenKind::OR){
            let op = (&self.tokens_list[self.curr_index]).clone(); 
            self.curr_index += 1; 
            let right = self.equality()?; 
            expr = Expression::Logical(Box::new(expr), op, Box::new(right)); 
        }

        Ok(expr)
    }

    fn and(&mut self) -> ParseResult<Expression>{
        let mut expr = self.equality()?; 

        while self.check(TokenKind::AND){
            let op = (&self.tokens_list[self.curr_index]).clone(); 
            self.curr_index += 1; 
            let right = self.equality()?; 
            expr = Expression::Logical(Box::new(expr), op, Box::new(right)); 
        }

        Ok(expr)
    }

    fn equality(&mut self) -> ParseResult<Expression>{
        let mut expr: Expression  = self.comparison()?;
        let type_list = [TokenKind::BANG_EQUAL, TokenKind::EQUAL_EQUAL];

        while self.match_token(&type_list){
            let operator: Token  = (&self.tokens_list[self.curr_index - 1]).clone();
            let right: Expression  = self.comparison()?; 
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right)); 
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult<Expression>{
        let mut expr: Expression  = self.term()?;
        let type_list = [TokenKind::GREATER, TokenKind::GREATER_EQUAL, TokenKind::LESS, TokenKind::LESS_EQUAL];

        while self.match_token(&type_list){
            let operator: Token  = (&self.tokens_list[self.curr_index - 1]).clone();
            let right: Expression  = self.term()?; 
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right)); 
        }

        Ok(expr)
    }

    fn term(&mut self) -> ParseResult<Expression>{
        let mut expr: Expression  = self.factor()?;
        let type_list = [TokenKind::MINUS, TokenKind::PLUS];

        while self.match_token(&type_list){
            let operator: Token  = (&self.tokens_list[self.curr_index - 1]).clone();
            let right: Expression  = self.factor()?; 
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right)); 
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult<Expression>{
        let mut expr: Expression  = self.unary()?;
        let type_list = [TokenKind::SLASH, TokenKind::STAR, TokenKind::PERCENT];

        while self.match_token(&type_list){
            let operator: Token  = (&self.tokens_list[self.curr_index - 1]).clone();
            let right: Expression  = self.unary()?; 
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right)); 
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult<Expression>{
        let type_list = [TokenKind::BANG, TokenKind::MINUS];

        while self.match_token(&type_list){
            let operator: Token  = (&self.tokens_list[self.curr_index - 1]).clone();
            let right: Expression  = self.unary()?; 
            return Ok(Expression::Unary(operator, Box::new(right))); 
        }

        self.call()
    }

    fn call(&mut self) -> ParseResult<Expression> {
        let mut expr = self.index()?; 

        loop{
            if !self.check(TokenKind::LEFT_PAREN){
                break;
            }

            let mut args_list: Vec<Expression> = Vec::new(); 
            self.curr_index += 1;

            if !self.check(TokenKind::RIGHT_PAREN){
                args_list.push(self.expression()?); 

                while self.check(TokenKind::COMMA){
                    self.curr_index += 1; 
                    args_list.push(self.expression()?);
                }
            }

            if self.check(TokenKind::RIGHT_PAREN){
                if args_list.len() > 255 {
                    return self.handle_error("Too many arguments: function calls can have at most 255 arguments.");
                }
                expr = Expression::Call(Box::new(expr), (&self.tokens_list[self.curr_index]).clone(), Box::new(args_list));
                self.curr_index += 1; 
            } else {
                return self.handle_error("Expected ')' after argument list.");
            }
        }

        Ok(expr)
    }

    fn index(&mut self) -> ParseResult<Expression>{
        let left_expr = self.primary()?; 

        if self.check(TokenKind::LEFT_SQUARE){

            self.curr_index += 1; 
            let right_expr = self.expression()?; 
            
            if !self.check(TokenKind::RIGHT_SQUARE){
                return self.handle_error("Expected ']' to close index expression.");
            }

            let right_brace_store = (&self.tokens_list[self.curr_index]).clone();
            self.curr_index += 1; 

            return Ok(Expression::Index(Box::new(left_expr), right_brace_store, Box::new(right_expr)));
        }

        Ok(left_expr)
    }

    fn primary(&mut self) -> ParseResult<Expression>{

        if self.atEnd(){
            return self.handle_error("Unexpected end of input.");
        }

        let literal: &Token = &self.tokens_list[self.curr_index]; 
        self.curr_index += 1;

        match &literal.kind {
            TokenKind::FALSE => return Ok(Expression::Literal(Value::Bool(false))),
            TokenKind::TRUE => return Ok(Expression::Literal(Value::Bool(true))),
            TokenKind::NONE => return Ok(Expression::Literal(Value::None)),
            TokenKind::STRING => return Ok(Expression::Literal(Value::String(literal.lexeme.clone()))),
            TokenKind::NUMBER => {
                let resulti = literal.lexeme.parse::<i128>(); 
                let resultf = literal.lexeme.parse::<f64>(); 

                if let Ok(i) = resulti {
                    return Ok(Expression::Literal(Value::Int(i)));
                } else if let Ok(f) = resultf {
                    return Ok(Expression::Literal(Value::Float(f)));
                } else {
                    return self.handle_error(&format!("Invalid number literal near '{}'.", literal.lexeme));
                }
            }
            TokenKind::IDENTIFIER => return Ok(Expression::Variable(literal.clone())),
            TokenKind::LAMBDA => return Ok(self.lambda()?),
            TokenKind::LEFT_SQUARE => return Ok(self.list()?),
            _ => { self.curr_index -= 1; }
        }

        if self.match_token(&[TokenKind::LEFT_PAREN]){
            let expression:Expression = self.expression()?;

            if self.match_token(&[TokenKind::RIGHT_PAREN]){
                return Ok(Expression::Grouping(Box::new(expression))); 
            }
            
            return self.handle_error("Expected ')' to close parenthesized expression.");
        }

        self.handle_error(&format!("Unexpected token '{}'.", &self.tokens_list[self.curr_index].lexeme))
    }

    fn lambda(&mut self) -> ParseResult<Expression>{

        if !self.check(TokenKind::LEFT_PAREN){
            return self.handle_error("Expected '(' after 'lambda'.");
        }
        
        self.curr_index += 1;
       
        let mut args_list: Vec<Token> = Vec::new();

        loop{
            if !self.check(TokenKind::RIGHT_PAREN){

                if !self.check(TokenKind::IDENTIFIER){
                    return self.handle_error("Expected an identifier as a parameter name in lambda expression.");
                }
                args_list.push((&self.tokens_list[self.curr_index]).clone()); 

                self.curr_index += 1;

                if !self.check(TokenKind::COMMA){
                    break;
                }

                self.curr_index += 1; 
            } else {
                break;
            }
        }

        if !self.check(TokenKind::RIGHT_PAREN){
            return self.handle_error("Expected ')' after lambda parameter list.");
        }

        self.curr_index += 1; 

        let statement = self.statement()?; 

        if !matches!(statement, Statement::Block(_)){
            return self.handle_error("Expected a block '{ ... }' for lambda body.");
        }

        let Statement::Block(statements) = statement else {
            unreachable!()
        };
        
        Ok(Expression::Lambda(args_list, statements))
    }

    fn list(&mut self) -> ParseResult<Expression>{

        let mut content: Vec<Expression> = Vec::new();
        loop{
            if !self.check(TokenKind::RIGHT_SQUARE){
                content.push(self.or()?); 

                if !self.check(TokenKind::COMMA){
                    break;
                }

                self.curr_index += 1; 
            } else {
                break;
            }
        }

        if !self.check(TokenKind::RIGHT_SQUARE){
            return self.handle_error("Expected ']' to close list literal.");
        }

        let right_brace_store = self.tokens_list[self.curr_index].clone();
        self.curr_index += 1; 
        
        Ok(Expression::List(Box::new(content), right_brace_store))
    }

    fn check(&mut self, kind:TokenKind) -> bool{
        if self.atEnd() { return false; }
        (&self.tokens_list[self.curr_index]).kind == kind
    }

    fn match_token(&mut self, args: &[TokenKind]) -> bool{
        if self.atEnd(){ return false; }

        for tokenKind in args{
            if self.check(*tokenKind){
                self.curr_index += 1; 
                return true; 
            }
        }

        false
    }

    fn handle_error<T>(&self, msg: &str) -> ParseResult<T> {
        let line = self.tokens_list[self.curr_index].line;
        let mut file = self.tokens_list[self.curr_index].file.clone();

        Err(format!("[{}: Line {}] Parser Error: {}", file, line, msg))
    }
}

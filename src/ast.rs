use crate::token::Token;
use crate::token::TokenKind;

#[derive(Debug, Clone)]

pub enum Expression {
    Binary(Box<Expression>, Token , Box<Expression>),
    Unary(Token, Box<Expression>), 
    Literal(Token), 
    Grouping(Box<Expression>)
}


trait ExpressionEvaluator {

    fn evaluate_binary(&mut self, l: Box<Expression>, o:Token, r:Box<Expression>) -> Expression;
    fn evaluate_unary(&mut self,  o: Token, r: Box<Expression>) -> Expression; 
    fn evaluate_literal(&mut self, t: Token) -> Expression; 
    fn evaluate_grouping(&mut self, exp: Box<Expression>) -> Expression; 

    fn evaluate(&mut self, expression: Expression) -> Expression{
        match expression{ 
            Expression::Binary(l, o, r) => self.evaluate_binary(l, o, r), 
            Expression::Unary(o, r) => self.evaluate_unary(o, r), 
            Expression::Literal(t) => self.evaluate_literal(t),
            Expression::Grouping(exp) => self.evaluate_grouping(exp), 
        }
    }
}



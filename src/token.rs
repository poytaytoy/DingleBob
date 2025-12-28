#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TokenKind {
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, PERCENT, SEMICOLON, SLASH, STAR, DEFINE, 

    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    AND, CLASS, ELSE, FALSE, FOR, IF, NONE, OR, BREAK, 
    PRINT, RETURN, SUPER, THIS, TRUE, LET, WHILE,

    NUMBER, 
    IDENTIFIER,
    STRING,  

    EOF 
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub kind: TokenKind, 
    pub lexeme: String, 
    pub line: i32 
}


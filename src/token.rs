#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum TokenKind {
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE, LEFT_SQUARE, RIGHT_SQUARE, 
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

    LAMBDA,

    EOF 
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Token {
    pub kind: TokenKind, 
    pub lexeme: String, 
    pub line: i32,
    pub id: i32,  
}


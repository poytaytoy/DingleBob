    use std::fs;
    use std::io;
    use std::env;
    use std::process; 

    const LEFT_PAREN: i8 = 1; 
    const RIGHT_PAREN: i8 = 2 ; 
    const LEFT_BRACE: i8 = 3 ; 
    const RIGHT_BRACE: i8 = 4;
    const COMMA: i8 = 5 ; 
    const DOT: i8 = 6 ; 
    const MINUS: i8 = 7; 
    const PLUS: i8 = 8; 
    const SEMICOLON: i8 = 9; 
    const SLASH: i8 = 10; 
    const STAR: i8 = 11;
    const EOF: i8 = 100;

    const BANG: i8 = 12;
    const BANG_EQUAL: i8 = 13;
    const EQUAL: i8 = 14;
    const EQUAL_EQUAL: i8 = 15;
    const GREATER: i8 = 16;
    const GREATER_EQUAL: i8 = 17;
    const LESS: i8 = 18;
    const LESS_EQUAL: i8 = 19;

    const IDENTIFIER: i8 = 20;
    const STRING: i8 = 21;
    const NUMBER: i8 = 22;

    const AND: i8 = 23;
    const CLASS: i8 = 24;
    const ELSE: i8 = 25;
    const FALSE: i8 = 26;
    const FUN: i8 = 27;
    const FOR: i8 = 28;
    const IF: i8 = 29;
    const NIL: i8 = 30;
    const OR: i8 = 31;
    const PRINT: i8 = 32;
    const RETURN: i8 = 33;
    const SUPER: i8 = 34;
    const THIS: i8 = 35;
    const TRUE: i8 = 36;
    const VAR: i8 = 37;
    const WHILE: i8 = 38;

    struct Token {
        source: String, 
        tokenType: i8,
        line: i32
    }

    struct Scanner{
        tokenList: Vec<Token>,
        source: String, 
        currIndex: usize, 
        line: i32, 

    }

    fn handleError(message: &str){
        println!("Error: {}", message); 
        process::exit(0); 
    }

    impl Scanner { 
        fn new(source: String) -> Scanner{
            Scanner { tokenList: Vec::new(), source: source, currIndex: 0, line: 0}
        }


        fn addToken(&mut self, source:String, tokenType: i8){
            self.tokenList.push(Token {
                source: source,
                tokenType: tokenType,
                line: self.line
            });
        }

        fn advance(&mut self) -> u8 {
            let s = self.source.as_bytes();

            let charByte = s[(self.currIndex as usize)]; 

            self.currIndex += 1; 

            charByte 
        } 

        fn peek(&self) -> u8 {

            if self.currIndex + 1 >= self.source.len() {
                return ('\0' as u8); 
            }

            let s = self.source.as_bytes();

            let charByte = s[(self.currIndex as usize) + 1];  

            charByte 
        }    

        fn peekPeek(&self) -> u8 {

            if self.currIndex + 2 >= self.source.len() {
                return ('\0' as u8); 
            }

            let s = self.source.as_bytes();

            let charByte = s[(self.currIndex as usize) + 2];  

            charByte 
        }    

        fn atEndOfSource (&self) -> bool {
            self.currIndex >= self.source.len()
        }

        fn matchDual(&mut self, check: char, IsCheck: i8, NotCheck: i8){

            let currentChar = self.source.as_bytes()[self.currIndex] as char; 

            if (self.peek() as char) == check {
                self.addToken(format!("{}{}", currentChar, check), IsCheck);
            } else{
                self.addToken(currentChar.to_string(), NotCheck);
            }

            self.currIndex += 1; 
        }

        fn handleString(&mut self, stringType: char) {

            let mut stringStore = String::new(); 
            let mut peek = self.peek() as char;

            while (peek != stringType){  

                if peek == '\0'{
                    handleError("Unterminated string found");
                } else {
                    stringStore.push((self.advance() as char));
                }    

                peek = self.peek() as char; 
            }

            self.advance();
            self.addToken(stringStore, STRING);
            }
        
        fn handleZero(&mut self) {
            
            let peek = self.peek() as char; 

            if peek >= '0' && peek <= '9'{
                handleError("Invalid Number");
            } else if peek == '.' {
                self.advance();

                let mut storedEnumerate = String::from("0.");
                let mut peek2 = self.peek() as char; 

                while  peek2 >= '0' && peek2 <= '9' {
                    storedEnumerate.push(peek2);
                    self.advance();
                }

                self.addToken(storedEnumerate, NUMBER);
            } else {
                self.addToken(String::from("0"), NUMBER);
            }
        }

        fn handleDigit(&mut self, currDigit: char) {
            
            let mut storedEnumerate = String::new(); 

            storedEnumerate.push(currDigit);

            loop{
                let peek = self.peek() as char; 

                if peek >= '0' && peek <= '9'{
                    storedEnumerate.push(peek);
                } else if peek == '.' {

                    let peekPeek = self.peekPeek() as char; 

                    if peekPeek >= '0' && peekPeek <= '9'{
                        storedEnumerate.push('.');
                    } else {
                        handleError("Invalid floating number");
                        break;
                    }
                } else {
                    self.addToken(storedEnumerate, NUMBER);
                    break;
                }

                self.advance();
            } 
        }

        fn handleIdentifier(&mut self){

            let mut storedName = String::new(); 
            storedName.push((self.source.as_bytes()[self.currIndex] as char));

            let mut peeked = self.peek() as char; 

            while peeked >= 'a' && peeked <= 'z' ||  peeked >= 'A' && peeked <= 'Z' || peeked >= '0' && peeked <= '9' || peeked == '_' || peeked == '\0'{
                storedName.push((self.advance() as char));
                peeked = self.peek() as char; 
            }

            //TODO FIGURE OUT WHAT THE &STR AND WHAT STRING IS 

            match storedName.as_str(){
                "and" => self.addToken(storedName, AND),
                "class" => self.addToken(storedName, CLASS),
                "else" => self.addToken(storedName, ELSE),
                "false" => self.addToken(storedName, FALSE),
                "for" => self.addToken(storedName, FOR),
                "fun" => self.addToken(storedName, FUN),
                "if" => self.addToken(storedName, IF),
                "nil" => self.addToken(storedName, NIL),
                "or" => self.addToken(storedName, OR),
                "print" => self.addToken(storedName, PRINT),
                "return" => self.addToken(storedName, RETURN),
                "super" => self.addToken(storedName, SUPER),
                "this" => self.addToken(storedName, THIS),
                "true" => self.addToken(storedName, TRUE),
                "var" => self.addToken(storedName, VAR),
                "while" => self.addToken(storedName, WHILE),
                _=> (self.addToken(storedName, IDENTIFIER))
            } 

        }


        fn scanTokens(&mut self) {

            loop {
                if self.atEndOfSource() {
                    break;
                }

                let character = self.advance() as char; 

                match character {
                    '(' => self.addToken( String::from("("), LEFT_PAREN),
                    ')' => self.addToken( String::from(")"), RIGHT_PAREN),
                    '{' => self.addToken(String::from("{"), LEFT_BRACE),
                    '}' => self.addToken(String::from("}"), RIGHT_BRACE),
                    ',' => self.addToken( String::from(","), COMMA),
                    '.' => self.addToken( String::from("."), DOT),
                    '-' => self.addToken(String::from("-"), MINUS),
                    '+' => self.addToken(String::from("+"), PLUS),
                    ';' => self.addToken( String::from(";"), SEMICOLON),
                    '*' => self.addToken( String::from("*"), STAR),
                    
                    '!' => self.matchDual('=', BANG_EQUAL, BANG), 
                    '=' => self.matchDual('=', EQUAL_EQUAL, EQUAL),
                    '<' => self.matchDual('=', LESS_EQUAL, LESS),
                    '>' => self.matchDual('=', GREATER_EQUAL, GREATER),

                    '/' => {
                        if (self.peek() as char) == '/' {
                            while (self.peek() as char) != '\n' || self.atEndOfSource() {
                                self.advance();
                            }}
                            else {
                                self.addToken(String::from("/"), SLASH);
                            }
                        },
                    '\n' => {self.line += 1;},
                    
                    ' ' | '\r' | '\t' => {}, 

                    '\'' | '\"' => self.handleString(character),
                    

                    '0' => self.handleZero(),
                    '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => self.handleDigit(character),
                    _ => 

                        if character >= 'a' && character <= 'z' || character >= 'A' && character <= 'Z' {
                            self.handleIdentifier();
                        } else {
                            handleError("Unidentified symbol");
                        }
                    ,
                }
            }
        }

        pub fn run(&self) {
            for token in &self.tokenList{
                println!("{}", token.tokenType)
            }
        }

    }


        

    fn run (code: String){  

        let mut scan: Scanner = Scanner::new(code);
        

    }

    fn runFile (location: &str){
        let contents = fs::read_to_string(String::from(location)).expect("Should have been able to read the file");
        run(contents);
    }

    fn runPrompt(){
        loop{
            print!("> "); 
            let mut buffer = String::new();

            if io::stdin().read_line(&mut buffer).is_err() {
                break;
            }

            let input = buffer.trim(); 

            if input == "exit()"{
                break;
            }

            if input.is_empty(){
                continue; 
            }

            run(input.to_string());
        
        }
    }

    fn main(){
        let args: Vec<String> = env::args().collect();

        if args.len() > 2{
            println!("Invalid Arguments"); 
            process::exit(0x0100);

        } else if args.len() == 2 {
            runFile(&args[1]);
        } else {
            runPrompt();
        }

    }
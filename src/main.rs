#![allow(warnings)]

use std::io::{self, Write};
use std::env;
use std::fs;

mod scanner; 
use scanner::scan;    

mod token; 
use token::TokenKind; 

mod ast; 
use ast::Expression; 

mod parser; 
use parser::Parser; 

mod interpreter; 
use interpreter::Interpreter; 
use interpreter::InterpretExpression;

//TODO Things to look into 
//1. lifetimes how the <'a> work and what the fuck is going on with them 
//2. how crates and libs work in rust  

fn main() -> io::Result<()> {

    println!("Dinglebob"); 

    //*This is to use it as a parser */

    // 'parse: loop {
    //     let mut input = String::new();
    //     print!(">>> ");
    //     io::stdout().flush()?; //apparently rust only outputs at every new line

    //     io::stdin().read_line(&mut input)?;
        
    //     let mut scan = Scanner::new(&input); 
    //     scan.convert(); 
    //     scan.debug(); 
    //     let mut token_list= scan.output(); 

    //     if input.trim() == "exit"{
    //         break 'parse;
    //     }
    // }

    //*This is to test it via the test.dingle file */

    let contents = fs::read_to_string("src/test.dingle")
        .expect("Should have been able to read the file");

    let mut token_list= scan(&contents, false);
    let mut parse = Parser::new(token_list);
    let mut interpret = Interpreter::new();

    println!("{:?}", interpret.evaluate(parse.expression()));

    // let mut input = String::from("+ - 123.5 12 hello = \"poop\" 0..3 #1 2 3111 \n hello while != !  e");
    // let mut scan = Scanner::new(&input); 
    // scan.debug(); 
    //println!("{:?}", Expression::Literal(TokenKind::NUMBER(100.0)));
    Ok(())    
}
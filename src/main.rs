use std::io::{self, Write};
use std::env;
use std::fs;

mod scanner; 
use scanner::Scanner;    

mod token; 
use token::TokenKind; 

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

    let mut scan = Scanner::new(&contents); 
        scan.convert(); 
        scan.debug(); 
        let mut token_list= scan.output(); 

    // let mut input = String::from("+ - 123.5 12 hello = \"poop\" 0..3 #1 2 3111 \n hello while != !  e");
    // let mut scan = Scanner::new(&input); 
    // scan.debug(); 

    Ok(())    
}
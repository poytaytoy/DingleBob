use std::io::{self, Write};

pub mod scanner; 
pub use scanner::Scanner;    


fn main() -> io::Result<()> {

    println!("Dinglebob test: "); 

    // 'parse: loop {
    //     let mut input = String::new();
    //     print!(">>> ");
    //     io::stdout().flush()?; //apparently rust only outputs at every new line

    //     io::stdin().read_line(&mut input)?;

        
    //     let mut scan = Scanner::new(&input); 
    //     scan.debug(); 
    //     //println!("You typed: {}", input.trim());

    //     if input.trim() == "exit"{
    //         break 'parse;
    //     }
    // }
    let mut input = String::from("+ - 123.5 12 hello = \"poop\" 0..3");
    let mut scan = Scanner::new(&input); 
    scan.debug(); 

    Ok(())    
}
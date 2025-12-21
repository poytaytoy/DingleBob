use std::io::{self, Write};

pub mod scanner; 
pub use scanner::Scanner;    


fn main() -> io::Result<()> {

    println!("Dinglebob test: "); 

    'parse: loop {
        let mut input = String::new();
        print!(">>> ");
        io::stdout().flush()?; //apparently rust only outputs at every new line

        io::stdin().read_line(&mut input)?;

        println!("You typed: {}", input.trim());

        if input.trim() == "exit"{
            break 'parse;
        }
    }

    Ok(())    
}
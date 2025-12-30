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

mod environment;

mod func;

mod resolver;
use resolver::Resolver;

fn run_source(source: &str, debug: bool) {
    let mut resolver = Resolver::new();
    let mut interpreter = Interpreter::new(true, resolver.give_local());
    let token_list = scan(source, debug);

    let token_result = scan(source, debug);

    if let Err(msg) = token_result{
        eprintln!("{}", msg);
        return (); 
    }

    let mut parser = Parser::new(token_result.unwrap());
    let parsed_result = parser.parse();

    if let Err(msg) = parsed_result{
        eprintln!("{}", msg);
        return (); 
    }

    let resolver_result = resolver.resolve((&parsed_result).clone().unwrap());

    if let Err(msg) = resolver_result{
        eprintln!("{}", msg);
        return (); 
    }

    let interpreter_result = interpreter.prime_interpret(parsed_result.unwrap());

    if let Err(msg) = interpreter_result{
        eprintln!("{}", msg);    
        return (); 
    }
}

fn run_line(source: &str, debug: bool, interpreter: &mut Interpreter, resolver: &mut Resolver){

    let mut resolver_save = resolver.clone();
    let mut intepreter_save = interpreter.clone(resolver_save.give_local());

    let token_result = scan(source, debug);

    if let Err(msg) = token_result{
        eprintln!("{}", msg);
        return (); 
    }

    let mut parser = Parser::new(token_result.unwrap());
    let parsed_result = parser.parse();

    if let Err(msg) = parsed_result{
        eprintln!("{}", msg);
        return (); 
    }

    let resolver_result = resolver.resolve((&parsed_result).clone().unwrap());

    if let Err(msg) = resolver_result{
        eprintln!("{}", msg);
        *resolver = resolver_save;
        return (); 
    }

    let interpreter_result = interpreter.prime_interpret(parsed_result.unwrap());

    if let Err(msg) = interpreter_result{
        eprintln!("{}", msg);
        *resolver = resolver_save;
        *interpreter = intepreter_save;

        return (); 
    }
}

fn run_file(path: &str, debug: bool) {
    let contents = fs::read_to_string(path)
        .unwrap_or_else(|_| {
            eprintln!("Could not read file '{}'", path);
            std::process::exit(1);
        });

    run_source(&contents, debug);
}

fn repl() -> io::Result<()> {
    println!("Dinglebob Interpreter");
    println!("Type 'exit' to quit.\n");

    let mut resolver = Resolver::new();
    let mut interpreter = Interpreter::new(true, resolver.give_local());

    loop {
        let mut input = String::new();
        print!(">>> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == "exit" {
            break;
        }
        run_line(&input, false, &mut interpreter, &mut resolver);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Usage:
    //   dinglebob                -> REPL
    //   dinglebob file.dingle     -> run file
    //   dinglebob -d file.dingle  -> run file with debug (tokens printed)
    match args.len() {
        1 => repl(),
        2 => {
            run_file(&args[1], false);
            Ok(())
        }
        3 => {
            if args[1] == "-d" || args[1] == "--debug" {
                run_file(&args[2], true);
                Ok(())
            } else {
                eprintln!("Usage:\n  dinglebob\n  dinglebob <file>\n  dinglebob -d <file>");
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("Usage:\n  dinglebob\n  dinglebob <file>\n  dinglebob -d <file>");
            std::process::exit(1);
        }
    }
}

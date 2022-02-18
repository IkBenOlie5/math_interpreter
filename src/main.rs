#![feature(let_chains)]
use interpreter::visit;
use lexer::lex;
use parser::Parser;
use std::{io, io::Write};

mod interpreter;
mod lexer;
mod parser;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let mut text = String::new();

        print!(">>> ");
        if let Err(e) = stdout.flush() {
            eprintln!("Error flushing stdout: {}", e);
            continue;
        }
        if let Err(e) = stdin.read_line(&mut text) {
            eprintln!("Error reading stdin: {}", e);
            continue;
        }

        let tokens = match lex(&text) {
            Ok(tokens) => tokens,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        let mut p = Parser::new(&tokens);
        let ast = match p.parse() {
            Ok(ast) => match ast {
                Some(ast) => ast,
                None => continue,
            },
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        let result = match visit(ast) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        println!("{}", result);
    }
}

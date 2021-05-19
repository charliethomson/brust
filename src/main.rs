use interpreter::Interpreter;
use lalrpop_util::lalrpop_mod;
use std::io::{stdin, stdout};
use std::io::{Read, Write};

lalrpop_mod!(pub grammar);

pub mod ast;
pub mod expression_constructors;
pub mod interpreter;

pub type Parser = grammar::FileParser;
fn main() {
    let mut buffer = String::new();
    stdout()
        .write(b"Enter the name of the test you'd like to run (holly, fib): ")
        .unwrap();
    stdout().flush().unwrap();
    stdin().read_line(&mut buffer);
    println!();

    Interpreter::new().interpret(format!("examples/{}.b", buffer.trim()));
}

fn interpret_string(s: String) -> i64 {
    Interpreter::new().interpret_string(s)
}
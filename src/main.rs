use std::io;

use crate::repl::start;

pub mod ast;
pub mod lexer;
pub mod repl;
pub mod token;

fn main() {
    println!("This is the Monkey Programming Language!");
    start(io::stdin(), io::stdout());
}

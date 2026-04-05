mod generator;
mod parser;
mod tokenizer;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        panic!("invalid number of args");
    }

    let tokens = tokenizer::tokenize(&args[1]);
    let mut parser = parser::Parser::new(tokens);
    let tree = parser.parse();

    println!(".text");
    println!(".globl main");

    println!("main:");
    generator::generate(tree);

    generator::pop("a0");

    println!("  li a7, 93");
    println!("  ecall");
}

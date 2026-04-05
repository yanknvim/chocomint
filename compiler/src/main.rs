mod generator;
mod parser;
mod tokenizer;

use std::env;
use std::collections::HashMap;

use crate::generator::Generator;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        panic!("invalid number of args");
    }

    let tokens = tokenizer::tokenize(&args[1]);
    let mut parser = parser::Parser::new(tokens);
    let trees = parser.parse();

    let mut generator = Generator::new(trees);

    println!(".text");
    println!(".globl main");

    println!("main:");

    println!("  addi sp, sp, -16");
    println!("  sd   ra, 8(sp)");
    println!("  sd   fp, 0(sp)");
    println!("  addi fp, sp, 16");

    generator.generate();

    println!("  addi a0, t0, 0");
    println!("  li a7, 93");
    println!("  ecall");
}

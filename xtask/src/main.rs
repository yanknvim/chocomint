use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
struct Args {
    command: String,
}

fn main() {
    let args = Args::parse();
    
    match args.command.as_str() {
        "compile" => {
            let command = Command::new("../../target/debug/compiler")
                .spawn()
                .unwrap();
        },
        _ => println!("invalid command"),
    }
}

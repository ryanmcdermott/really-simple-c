extern crate rscc;
use std::env;
use std::process;

fn help() {
    println!("Usage: rscc <filepath>");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        help();
    }

    let filepath = args[1].clone();

    let tokens = tokenize(filepath);
    let ast = parse(tokens);
    let ir = gen_ir(ast);
    let x86_assembly = gen_x86_assembly(ast);
}
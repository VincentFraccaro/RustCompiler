mod generate;
mod parser;
mod tokenisation;

use crate::generate::Generator;
use crate::parser::{NodeProg, Parser};
use crate::tokenisation::Tokeniser;
use std::process::Command;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        panic!("Incorrect usage... Please use compiler.exe <input.tr>\n");
    }

    let file_path = &args[1];
    println!("{}", &args[0]);
    println!("{}", file_path);

    let contents = fs::read_to_string(file_path).expect("File has been read");

    println!("{}", contents);

    let mut tokeniser = Tokeniser::new(contents);
    let tokens = tokeniser.tokenize();
    for token in &tokens {
        println!("I am this token {:?}", token.token_type);
    }

    let mut parser = Parser::new(tokens);

    let tree: Option<NodeProg> = Some(parser.parse_program().expect("Parse program failed"));

    println!("About to generate");
    let mut generator = Generator::new(tree.unwrap());
    println!("Created generator");
    /*generator.generate_program();
    println!("Generator generated");*/

    fs::write("out.asm", generator.generate_program()).expect("This didn't work lol");

    let run_assembler = Command::new("sh")
        .args(&["-c", "nasm -felf64 out.asm"])
        .output()
        .expect("failed to execute process");

    run_assembler.stdout;

    let run_linker = Command::new("sh")
        .args(&["-c", "ld -o out out.o"])
        .output()
        .expect("failed to execute process");

    run_linker.stdout;
}

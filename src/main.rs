mod tokenisation;
mod generate;

use std::{env, fs};
use std::process::{Command};
use crate::generate::tokens_to_assembly;
use crate::tokenisation::{Token, tokenise};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        panic!("Incorrect usage... Please use compiler.exe <input.tr>\n");
    }

    let file_path = &args[1];
    println!("{}", &args[0]);
    println!("{}", file_path);


    let contents = fs::read_to_string(file_path)
        .expect("File has been read");

    println!("{}", contents);



    let tokens: Vec<Token> = tokenise(contents);
    for x in &tokens {
        println!("{:?}", x.token_type);
    }

    let assembly = tokens_to_assembly(tokens);
    println!("{}", assembly);

    fs::write("out.asm", assembly).expect("This didn't work lol");

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

use std::fmt;
use crate::tokenisation::Token;
use crate::tokenisation::TokenType::{IntLit, RETURN, SEMI};

pub fn tokens_to_assembly(tokens: Vec<Token>) -> String {
    let mut stream = String::from("global _start\n_start:\n");

    let mut i = 0;
    while i < tokens.len() {
        let token: &Token = &tokens[i];
        println!("I am here ");
        if token.token_type == RETURN {
            println!("I am here as a RETURN");
            if (i + 1 < tokens.len()) && (tokens[i + 1].token_type == IntLit) {
                println!("I am here as a INTLIT");
                if (i + 2 < tokens.len()) && (tokens[i + 2].token_type == SEMI) {
                    println!("I am here as a SEMI");
                    fmt::Write::write_str(&mut stream, "    mov rax, 60\n").unwrap();

                    let token_value = match &tokens[i + 1].value {
                        Some(value) => value.as_str(),
                        None => "",
                    };

                    fmt::Write::write_str(&mut stream, &format!("    mov rdi, {}\n", token_value)).unwrap();
                    fmt::Write::write_str(&mut stream, "    syscall\n").unwrap();

                    // Move past the processed tokens
                    i += 3;
                    continue;
                }
            }
        }
        // Increment i if the current token doesn't match the expected pattern
        i += 1;
    }

    stream
}

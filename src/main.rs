use std::env;
use std::env::Args;
use std::ptr::null;

#[derive(Debug)]
enum TokenType {
    RETURN,
    SEMI,
    INT_LIT
}

struct Token {
    token: TokenType,
    value: Option<String>
}

fn tokenise(string: &str) -> Vec<TokenType> {
    let mut tokens: Vec<TokenType> = vec![];
    tokens.push(TokenType::INT_LIT);
    return tokens;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("{}", file_path);

    let mut tokens: Vec<TokenType> = tokenise("Hello");
    for x in tokens {
        println!("{:?}", x);
    }
}

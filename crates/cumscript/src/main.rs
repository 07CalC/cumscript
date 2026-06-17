use std::fs;

use lexer::Lexer;

fn main() {
    let content = fs::read_to_string("expl.cum").unwrap();
    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize();
    for token in tokens {
        println!("{:?}", token);
    }
}

#[macro_use]
extern crate lazy_static;

mod lexer;
mod token;

use lexer::Lexer;

fn main() {
    let input: &str = "=+(){},;";
    let mut lexer: Lexer = Lexer::new(input);
    lexer.read_char();
    let token = lexer.next_token();
}

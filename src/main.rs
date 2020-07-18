mod ast;
mod errors;
mod parser;
mod tokens;

use tokens::Token;

use logos::Logos;

fn main() {
    let code = "
    data Caca = {
        Foo | Bar Int 
    }
    a = { 12 + 8 }
    b x = { x - 8 }
    c (Foo) = { 8 }
    c (Bar x) = { b x }
    ";
    let tokens = Token::lexer(code).spanned();
    for tok in tokens {
        println!("{:#?}", tok);
    }
}

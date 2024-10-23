mod ast;
mod interpreter;
mod lexer;
mod parser;

use interpreter::interpret;
use lexer::lexer;
use parser::parse;

fn main() {
    let code = r#"
      あ("こんにちは、👺");
    "#;

    let tokens = lexer(code);
    println!("Tokens: {:?}", tokens);
    match parse(&tokens) {
        Ok(ast) => {
            interpret(ast);
        }
        Err(e) => println!("Error: {}", e),
    }
}

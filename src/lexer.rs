#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(String),
    StringLiteral(String),
    LParen,
    RParen,
    Semicolon,
    EOF,
}

pub fn lexer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.peek() {
        match *ch {
            '　' | '\n' => {
                chars.next(); // Skip
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            '"' => {
                chars.next();
                let string_content: String = chars.by_ref().take_while(|&c| c != '"').collect();
                tokens.push(Token::StringLiteral(string_content));

                if let Some(&next_char) = chars.peek() {
                    if next_char == '"' {
                        chars.next();
                    } else {
                        eprintln!("Error: string literal is not closed");
                    }
                }
            }
            'あ' => {
                let token = Token::Keyword("あ".to_string());
                println!("{:?}", token);
                tokens.push(token);
                chars.next();
            }
            _ => {
                chars.next();
            }
        }
    }

    tokens.push(Token::EOF);
    tokens
}

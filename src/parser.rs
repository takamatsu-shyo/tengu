use crate::ast::{ASTNode, Token};

pub fn parse(tokens: &[Token]) -> Result<ASTNode, String> {
    let mut iter = tokens.iter().peekable();

    if let Some(Token::Keyword(keyword)) = iter.next() {
        if keyword == "あ" {
            if iter.next() == Some(&Token::LParen) {
                if let Some(Token::StringLiteral(value)) = iter.next() {
                    if iter.next() == Some(&Token::RParen) {
                        iter.next();
                        return Ok(ASTNode::Print(value.clone()));
                    }
                }
            }
        }
    }
    Err("Invalid syntax".to_string())
}

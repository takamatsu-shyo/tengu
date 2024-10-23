pub use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Print(String),
}

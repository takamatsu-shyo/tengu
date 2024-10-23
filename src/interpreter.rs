use crate::ast::ASTNode;

pub fn interpret(ast: ASTNode) {
    match ast {
        ASTNode::Print(value) => {
            println!("{}", value);
        }
    }
}

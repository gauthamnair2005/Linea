use linea_core::Result;

pub mod lexer;
pub mod token;
pub mod ast;
pub mod parser;

pub use lexer::Lexer;
pub use token::{Token, TokenType};
pub use parser::Parser;
pub use ast::{Program, Statement, Expression, BinaryOp, UnaryOp};

/// Parse Linea source code into an AST
pub fn parse(source: &str) -> Result<Program> {
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.parse()
}

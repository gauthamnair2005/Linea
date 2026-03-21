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

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn parses_basic_program() {
        let src = "var x @ int = 1\ndisplay x";
        let program = parse(src).expect("parser should succeed");
        assert!(!program.statements.is_empty());
    }
}

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Syntax error at line {line}, column {column}: {message}")]
    Syntax {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Type error: {0}")]
    TypeError(String),

    #[error("Variable '{0}' not found")]
    VariableNotFound(String),

    #[error("Variable '{0}' already declared")]
    VariableAlreadyDeclared(String),

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error("Return called")]
    Return(crate::value::Value),

    #[error("IO error: {0}")]
    IoError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

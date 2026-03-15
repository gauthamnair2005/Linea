use linea_core::{Result, Error, Type};
use crate::token::{Token, TokenType};
use crate::ast::{Program, Statement, Expression, BinaryOp, UnaryOp};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        match &self.current_token().token_type {
            TokenType::Var => self.parse_var_declaration(),
            TokenType::VarUpdate => self.parse_var_update(),
            TokenType::Display => self.parse_display(),
            TokenType::For => self.parse_for(),
            TokenType::While => self.parse_while(),
            TokenType::If => self.parse_if(),
            TokenType::Function => self.parse_function_decl(),
            TokenType::Return => self.parse_return(),
            TokenType::Break => {
                self.advance();
                Ok(Statement::Break)
            }
            TokenType::Continue => {
                self.advance();
                Ok(Statement::Continue)
            }
            _ => {
                let expr = self.parse_expression()?;
                self.consume_optional(&TokenType::Semicolon);
                Ok(Statement::Expression(expr))
            }
        }
    }

    fn parse_var_declaration(&mut self) -> Result<Statement> {
        self.expect(&TokenType::Var)?;
        let name = self.expect_identifier()?;
        self.expect(&TokenType::Equal)?;
        let expr = self.parse_expression()?;
        self.consume_optional(&TokenType::Semicolon);
        Ok(Statement::VarDeclaration { name, expr })
    }

    fn parse_var_update(&mut self) -> Result<Statement> {
        self.expect(&TokenType::VarUpdate)?;
        let name = self.expect_identifier()?;
        self.expect(&TokenType::Equal)?;
        let expr = self.parse_expression()?;
        self.consume_optional(&TokenType::Semicolon);
        Ok(Statement::VarUpdate { name, expr })
    }

    fn parse_display(&mut self) -> Result<Statement> {
        self.expect(&TokenType::Display)?;
        let expr = self.parse_expression()?;
        self.consume_optional(&TokenType::Semicolon);
        Ok(Statement::Display(expr))
    }

    fn parse_for(&mut self) -> Result<Statement> {
        self.expect(&TokenType::For)?;
        let var = self.expect_identifier()?;
        self.expect(&TokenType::From)?;
        let start = self.parse_expression()?;
        self.expect(&TokenType::Range)?;
        let end = self.parse_expression()?;
        self.consume_optional(&TokenType::Semicolon);
        
        let body = if self.current_token().token_type == TokenType::LeftBrace {
            self.parse_block()?
        } else {
            vec![self.parse_statement()?]
        };

        Ok(Statement::For { var, start, end, body })
    }

    fn parse_while(&mut self) -> Result<Statement> {
        self.expect(&TokenType::While)?;
        let condition = self.parse_expression()?;
        let body = if self.current_token().token_type == TokenType::LeftBrace {
            self.parse_block()?
        } else {
            vec![self.parse_statement()?]
        };
        Ok(Statement::While { condition, body })
    }

    fn parse_if(&mut self) -> Result<Statement> {
        self.expect(&TokenType::If)?;
        let condition = self.parse_expression()?;
        let then_body = if self.current_token().token_type == TokenType::LeftBrace {
            self.parse_block()?
        } else {
            vec![self.parse_statement()?]
        };
        
        let else_body = if self.consume_optional(&TokenType::Else) {
            Some(if self.current_token().token_type == TokenType::LeftBrace {
                self.parse_block()?
            } else {
                vec![self.parse_statement()?]
            })
        } else {
            None
        };

        Ok(Statement::If { condition, then_body, else_body })
    }

    fn parse_function_decl(&mut self) -> Result<Statement> {
        self.expect(&TokenType::Function)?;
        let name = self.expect_identifier()?;
        self.expect(&TokenType::LeftParen)?;

        let mut params = Vec::new();
        if self.current_token().token_type != TokenType::RightParen {
            loop {
                let param_name = self.expect_identifier()?;
                self.expect(&TokenType::Colon)?;
                let param_type = self.parse_type()?;
                params.push((param_name, param_type));

                if !self.consume_optional(&TokenType::Comma) {
                    break;
                }
            }
        }
        self.expect(&TokenType::RightParen)?;
        self.expect(&TokenType::Arrow)?;
        let return_type = self.parse_type()?;

        let body = self.parse_block()?;

        Ok(Statement::FunctionDecl { name, params, return_type, body })
    }

    fn parse_return(&mut self) -> Result<Statement> {
        self.expect(&TokenType::Return)?;
        let expr = if self.current_token().token_type == TokenType::Semicolon
            || self.current_token().token_type == TokenType::Eof {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.consume_optional(&TokenType::Semicolon);
        Ok(Statement::Return(expr))
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>> {
        self.expect(&TokenType::LeftBrace)?;
        let mut statements = Vec::new();

        while self.current_token().token_type != TokenType::RightBrace && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.expect(&TokenType::RightBrace)?;
        Ok(statements)
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expression> {
        let mut left = self.parse_and()?;

        while self.current_token().token_type == TokenType::Or {
            self.advance();
            let right = self.parse_and()?;
            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expression> {
        let mut left = self.parse_equality()?;

        while self.current_token().token_type == TokenType::And {
            self.advance();
            let right = self.parse_equality()?;
            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expression> {
        let mut left = self.parse_comparison()?;

        loop {
            let op = match self.current_token().token_type {
                TokenType::EqualEqual => BinaryOp::Equal,
                TokenType::NotEqual => BinaryOp::NotEqual,
                _ => break,
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expression> {
        let mut left = self.parse_addition()?;

        loop {
            let op = match self.current_token().token_type {
                TokenType::Less => BinaryOp::Less,
                TokenType::Greater => BinaryOp::Greater,
                TokenType::LessEqual => BinaryOp::LessEqual,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                _ => break,
            };
            self.advance();
            let right = self.parse_addition()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_addition(&mut self) -> Result<Expression> {
        let mut left = self.parse_multiplication()?;

        loop {
            let op = match self.current_token().token_type {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Subtract,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplication()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expression> {
        let mut left = self.parse_power()?;

        loop {
            let op = match self.current_token().token_type {
                TokenType::Star => BinaryOp::Multiply,
                TokenType::Slash => BinaryOp::Divide,
                TokenType::Percent => BinaryOp::Modulo,
                _ => break,
            };
            self.advance();
            let right = self.parse_power()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expression> {
        let mut left = self.parse_unary()?;

        if self.current_token().token_type == TokenType::Caret {
            self.advance();
            let right = self.parse_power()?;
            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOp::Power,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expression> {
        match self.current_token().token_type {
            TokenType::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expression::Unary {
                    op: UnaryOp::Negate,
                    expr: Box::new(expr),
                })
            }
            TokenType::Not => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expression::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expression> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.current_token().token_type {
                TokenType::LeftParen => {
                    self.advance();
                    let mut args = Vec::new();
                    if self.current_token().token_type != TokenType::RightParen {
                        loop {
                            args.push(self.parse_expression()?);
                            if !self.consume_optional(&TokenType::Comma) {
                                break;
                            }
                        }
                    }
                    self.expect(&TokenType::RightParen)?;
                    expr = Expression::Call {
                        func: Box::new(expr),
                        args,
                    };
                }
                TokenType::Dot => {
                    self.advance();
                    let member = self.expect_identifier()?;
                    expr = Expression::MemberAccess {
                        object: Box::new(expr),
                        member,
                    };
                }
                TokenType::LeftBracket => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect(&TokenType::RightBracket)?;
                    expr = Expression::Index {
                        expr: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expression> {
        match &self.current_token().token_type.clone() {
            TokenType::Integer(n) => {
                let val = *n;
                self.advance();
                Ok(Expression::Integer(val))
            }
            TokenType::Float(f) => {
                let val = *f;
                self.advance();
                Ok(Expression::Float(val))
            }
            TokenType::String(s) => {
                let val = s.clone();
                self.advance();
                Ok(Expression::String(val))
            }
            TokenType::True => {
                self.advance();
                Ok(Expression::Bool(true))
            }
            TokenType::False => {
                self.advance();
                Ok(Expression::Bool(false))
            }
            TokenType::Yes => {
                self.advance();
                Ok(Expression::Bool(true))
            }
            TokenType::No => {
                self.advance();
                Ok(Expression::Bool(false))
            }
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expression::Identifier(name))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(&TokenType::RightParen)?;
                Ok(expr)
            }
            TokenType::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();
                if self.current_token().token_type != TokenType::RightBracket {
                    loop {
                        elements.push(self.parse_expression()?);
                        if !self.consume_optional(&TokenType::Comma) {
                            break;
                        }
                    }
                }
                self.expect(&TokenType::RightBracket)?;
                Ok(Expression::Array(elements))
            }
            TokenType::TypeCast => {
                self.advance();
                let expr = self.parse_postfix()?;
                self.expect(&TokenType::Equal)?;
                let target_type = self.parse_type()?;
                Ok(Expression::TypeCast {
                    expr: Box::new(expr),
                    target_type,
                })
            }
            _ => Err(Error::Syntax {
                line: self.current_token().line,
                column: self.current_token().column,
                message: format!("Unexpected token: {:?}", self.current_token().token_type),
            })
        }
    }

    fn parse_type(&mut self) -> Result<Type> {
        let type_name = self.expect_identifier()?;
        match type_name.as_str() {
            "int" => Ok(Type::Int),
            "float" => Ok(Type::Float),
            "string" => Ok(Type::String),
            "bool" => Ok(Type::Bool),
            _ => Err(Error::TypeError(format!("Unknown type: {}", type_name))),
        }
    }

    fn current_token(&self) -> Token {
        self.tokens.get(self.position)
            .cloned()
            .unwrap_or(Token::new(TokenType::Eof, 0, 0))
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_token().token_type == TokenType::Eof
    }

    fn expect(&mut self, expected: &TokenType) -> Result<()> {
        if std::mem::discriminant(&self.current_token().token_type) == std::mem::discriminant(expected) {
            self.advance();
            Ok(())
        } else {
            Err(Error::Syntax {
                line: self.current_token().line,
                column: self.current_token().column,
                message: format!("Expected {:?}, found {:?}", expected, self.current_token().token_type),
            })
        }
    }

    fn expect_identifier(&mut self) -> Result<String> {
        match &self.current_token().token_type.clone() {
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(Error::Syntax {
                line: self.current_token().line,
                column: self.current_token().column,
                message: "Expected identifier".to_string(),
            })
        }
    }

    fn consume_optional(&mut self, token_type: &TokenType) -> bool {
        if std::mem::discriminant(&self.current_token().token_type) == std::mem::discriminant(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }
}

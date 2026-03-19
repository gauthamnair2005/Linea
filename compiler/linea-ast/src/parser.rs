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
            TokenType::Import => self.parse_import(),
            TokenType::Var => self.parse_var_declaration(),
            TokenType::Obj => self.parse_obj_declaration(),
            TokenType::VarUpdate => self.parse_var_update(),
            TokenType::Display => self.parse_display(),
            TokenType::For => self.parse_for(),
            TokenType::While => self.parse_while(),
            TokenType::If => self.parse_if(),
            TokenType::Switch => self.parse_switch(),
            TokenType::Function => self.parse_function_decl(),
            TokenType::Class => self.parse_class_decl(),
            TokenType::MacroRules => self.parse_macro_decl(),
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
                if self.consume_optional(&TokenType::Equal) {
                    let value = self.parse_expression()?;
                    self.consume_optional(&TokenType::Semicolon);
                    return Ok(Statement::Assignment { target: expr, expr: value });
                }
                self.consume_optional(&TokenType::Semicolon);
                Ok(Statement::Expression(expr))
            }
        }
    }

    fn parse_import(&mut self) -> Result<Statement> {
        self.expect(&TokenType::Import)?;
        let module = self.expect_identifier()?;
        
        let items = if self.current_token().token_type == TokenType::LeftBrace {
            self.advance();
            let mut items = vec![];
            while self.current_token().token_type != TokenType::RightBrace && !self.is_at_end() {
                items.push(self.expect_identifier()?);
                if self.current_token().token_type == TokenType::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
            self.expect(&TokenType::RightBrace)?;
            items
        } else {
            vec!["*".to_string()]
        };
        
        self.consume_optional(&TokenType::Semicolon);
        Ok(Statement::Import { module, items })
    }

    fn parse_var_declaration(&mut self) -> Result<Statement> {
        self.expect(&TokenType::Var)?;
        let name = self.expect_identifier()?;
        
        // Check for @ type annotation (v4.0 syntax: var x @ int = 42)
        let type_annotation = if self.current_token().token_type == TokenType::At {
            self.advance();
            Some(self.parse_type_annotation()?)
        } else {
            None
        };
        
        self.expect(&TokenType::Equal)?;
        let expr = self.parse_expression()?;
        self.consume_optional(&TokenType::Semicolon);
        Ok(Statement::VarDeclaration { name, type_annotation, expr })
    }

    fn parse_obj_declaration(&mut self) -> Result<Statement> {
        self.expect(&TokenType::Obj)?;
        let name = self.expect_identifier()?;
        self.expect(&TokenType::At)?;
        let class_name = self.expect_identifier()?;
        if Self::is_builtin_type_name(&class_name) {
            return Err(Error::TypeError(
                "Objects can only be created from classes. Built-in datatypes can only be used with 'var' declarations.".to_string(),
            ));
        }
        self.expect(&TokenType::Equal)?;
        let constructor = self.parse_expression()?;
        self.consume_optional(&TokenType::Semicolon);
        Ok(Statement::ObjDeclaration { name, class_name, constructor })
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
        self.expect(&TokenType::Tilde)?;
        let end = self.parse_expression()?;
        
        // Optional step modifier
        let step = if self.current_token().token_type == TokenType::Identifier("step".to_string()) {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        self.consume_optional(&TokenType::Semicolon);
        
        let body = if self.current_token().token_type == TokenType::LeftBrace {
            self.parse_block()?
        } else {
            vec![self.parse_statement()?]
        };

        Ok(Statement::For { var, start, end, step, body })
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

    fn parse_switch(&mut self) -> Result<Statement> {
        self.expect(&TokenType::Switch)?;
        let expr = self.parse_expression()?;
        self.expect(&TokenType::LeftBrace)?;

        let mut cases: Vec<(Expression, Vec<Statement>)> = Vec::new();
        let mut default: Option<Vec<Statement>> = None;

        while self.current_token().token_type != TokenType::RightBrace && !self.is_at_end() {
            match self.current_token().token_type.clone() {
                TokenType::Case => {
                    self.advance();
                    let case_expr = self.parse_expression()?;
                    self.expect(&TokenType::Colon)?;
                    let case_body = self.parse_switch_case_body()?;
                    cases.push((case_expr, case_body));
                }
                TokenType::Default => {
                    self.advance();
                    self.expect(&TokenType::Colon)?;
                    default = Some(self.parse_switch_case_body()?);
                }
                _ => {
                    return Err(Error::Syntax {
                        line: self.current_token().line,
                        column: self.current_token().column,
                        message: "Expected `case` or `default` inside switch block".to_string(),
                    });
                }
            }
        }

        self.expect(&TokenType::RightBrace)?;
        Ok(Statement::Switch { expr, cases, default })
    }

    fn parse_switch_case_body(&mut self) -> Result<Vec<Statement>> {
        if self.current_token().token_type == TokenType::LeftBrace {
            return self.parse_block();
        }

        let mut body = Vec::new();
        while !self.is_at_end() {
            match self.current_token().token_type {
                TokenType::Case | TokenType::Default | TokenType::RightBrace => break,
                _ => body.push(self.parse_statement()?),
            }
        }
        Ok(body)
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

    fn parse_class_decl(&mut self) -> Result<Statement> {
        self.expect(&TokenType::Class)?;
        let name = self.expect_identifier()?;
        let super_class = if self.consume_optional(&TokenType::From) {
            Some(self.expect_identifier()?)
        } else {
            None
        };

        self.expect(&TokenType::LeftBrace)?;
        let mut body = Vec::new();
        while self.current_token().token_type != TokenType::RightBrace && !self.is_at_end() {
            body.push(self.parse_class_member_statement()?);
        }
        self.expect(&TokenType::RightBrace)?;
        Ok(Statement::ClassDecl { name, super_class, body })
    }

    fn parse_class_member_statement(&mut self) -> Result<Statement> {
        match self.current_token().token_type {
            TokenType::Var => self.parse_var_declaration(),
            TokenType::Function => self.parse_function_decl(),
            _ => Err(Error::Syntax {
                line: self.current_token().line,
                column: self.current_token().column,
                message: "Class bodies can only contain field declarations (var) and methods (func)".to_string(),
            }),
        }
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

    fn parse_macro_decl(&mut self) -> Result<Statement> {
        self.expect(&TokenType::MacroRules)?;
        self.consume_optional(&TokenType::Exclamation);
        let name = self.expect_identifier()?;
        self.expect(&TokenType::LeftParen)?;

        let mut params = Vec::new();
        if self.current_token().token_type != TokenType::RightParen {
            loop {
                params.push(self.expect_identifier()?);
                if !self.consume_optional(&TokenType::Comma) {
                    break;
                }
            }
        }
        self.expect(&TokenType::RightParen)?;

        let body = if self.consume_optional(&TokenType::FatArrow) {
            let expr = self.parse_expression()?;
            self.consume_optional(&TokenType::Semicolon);
            expr
        } else if self.current_token().token_type == TokenType::LeftBrace {
            self.advance();
            let expr = self.parse_expression()?;
            self.expect(&TokenType::RightBrace)?;
            self.consume_optional(&TokenType::Semicolon);
            expr
        } else {
            return Err(Error::Syntax {
                line: self.current_token().line,
                column: self.current_token().column,
                message: "Expected '=>' or '{ ... }' in macro_rules declaration".to_string(),
            });
        };

        Ok(Statement::MacroDecl { name, params, body })
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
        self.parse_ternary()
    }

    fn parse_ternary(&mut self) -> Result<Expression> {
        let mut condition = self.parse_or()?;

        if self.consume_optional(&TokenType::Question) {
            let then_expr = self.parse_expression()?;
            self.expect(&TokenType::Colon)?;
            let else_expr = self.parse_expression()?;
            condition = Expression::Ternary {
                condition: Box::new(condition),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            };
        }

        Ok(condition)
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
            TokenType::Not | TokenType::Exclamation => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expression::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                })
            }
            TokenType::Ampersand => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expression::Unary {
                    op: UnaryOp::AddressOf,
                    expr: Box::new(expr),
                })
            }
            TokenType::Star => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expression::Unary {
                    op: UnaryOp::Dereference,
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
                TokenType::Exclamation => {
                    if let Expression::Identifier(name) = &expr {
                        self.advance();
                        self.expect(&TokenType::LeftParen)?;
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
                        expr = Expression::MacroCall {
                            name: name.clone(),
                            args,
                        };
                    } else {
                        break;
                    }
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
                    
                    // Check for slice vs simple index
                    let mut start: Option<Box<Expression>> = None;
                    let mut end: Option<Box<Expression>> = None;
                    let mut step: Option<Box<Expression>> = None;
                    let mut is_slice = false;
                    
                    // Parse start or detect slice
                    if !matches!(self.current_token().token_type, TokenType::Colon | TokenType::RightBracket) {
                        start = Some(Box::new(self.parse_expression()?));
                    }
                    
                    // Check if slice notation (`:`)
                    if matches!(self.current_token().token_type, TokenType::Colon) {
                        is_slice = true;
                        self.advance();
                        
                        // Parse end
                        if !matches!(self.current_token().token_type, TokenType::Colon | TokenType::RightBracket) {
                            end = Some(Box::new(self.parse_expression()?));
                        }
                        
                        // Parse step if exists
                        if matches!(self.current_token().token_type, TokenType::Colon) {
                            self.advance();
                            if !matches!(self.current_token().token_type, TokenType::RightBracket) {
                                step = Some(Box::new(self.parse_expression()?));
                            }
                        }
                    }
                    
                    self.expect(&TokenType::RightBracket)?;
                    
                    if is_slice {
                        expr = Expression::Slice {
                            expr: Box::new(expr),
                            start,
                            end,
                            step,
                        };
                    } else if let Some(index) = start {
                        expr = Expression::Index {
                            expr: Box::new(expr),
                            index,
                        };
                    } else {
                        return Err(Error::Syntax {
                            line: self.current_token().line,
                            column: self.current_token().column,
                            message: "Invalid array indexing".to_string(),
                        });
                    }
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
                let mut name = name.clone();
                self.advance();
                // Handle module::function syntax (e.g., csv::parse)
                while self.current_token().token_type == TokenType::DoubleColon {
                    self.advance();
                    if let TokenType::Identifier(next_name) = &self.current_token().token_type {
                        name.push_str("::");
                        name.push_str(next_name);
                        self.advance();
                    } else {
                        return Err(Error::Syntax { 
                            line: self.current_token().line, 
                            column: self.current_token().column, 
                            message: "Expected identifier after '::'".to_string() 
                        });
                    }
                }
                Ok(Expression::Identifier(name))
            }
            TokenType::This => {
                self.advance();
                Ok(Expression::Identifier("this".to_string()))
            }
            TokenType::Super => {
                self.advance();
                Ok(Expression::Identifier("super".to_string()))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(&TokenType::RightParen)?;
                Ok(expr)
            }
            TokenType::If => {
                self.advance();
                let condition = self.parse_expression()?;
                self.expect(&TokenType::LeftBrace)?;
                let then_expr = self.parse_expression()?;
                self.expect(&TokenType::RightBrace)?;
                self.expect(&TokenType::Else)?;
                self.expect(&TokenType::LeftBrace)?;
                let else_expr = self.parse_expression()?;
                self.expect(&TokenType::RightBrace)?;
                Ok(Expression::IfExpression {
                    condition: Box::new(condition),
                    then_expr: Box::new(then_expr),
                    else_expr: Box::new(else_expr),
                })
            }
            TokenType::Pipe => {
                self.advance();
                let mut params = Vec::new();
                if self.current_token().token_type != TokenType::Pipe {
                    loop {
                        let param_name = self.expect_identifier()?;
                        if self.consume_optional(&TokenType::Colon) {
                            let _ = self.parse_type()?;
                        }
                        params.push(param_name);
                        if !self.consume_optional(&TokenType::Comma) {
                            break;
                        }
                    }
                }
                self.expect(&TokenType::Pipe)?;
                self.expect(&TokenType::FatArrow)?;
                let body = self.parse_expression()?;
                Ok(Expression::Lambda {
                    params,
                    body: Box::new(body),
                })
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
        let base_type = match type_name.as_str() {
            "int" => Type::Int,
            "float" => Type::Float,
            "str" => Type::String,
            "string" => Type::String,
            "bool" => Type::Bool,
            "any" => Type::Unknown,
            _ => return Err(Error::TypeError(format!("Unknown type: {}", type_name))),
        };
        
        let mut dims = 0;
        while self.consume_optional(&TokenType::LeftBracket) {
            self.expect(&TokenType::RightBracket)?;
            dims += 1;
        }
        
        match dims {
            0 => Ok(base_type),
            1 => Ok(Type::Array(Box::new(base_type))),
            2 => Ok(Type::Matrix(Box::new(base_type))),
            3 => Ok(Type::Tensor(Box::new(base_type))),
            _ => Ok(Type::Tensor(Box::new(base_type))),
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
                let mut name = name.clone();
                self.advance();
                
                // Allow scoped identifiers (e.g. module::func)
                while self.current_token().token_type == TokenType::DoubleColon {
                    self.advance();
                    if let TokenType::Identifier(next_name) = &self.current_token().token_type {
                        name.push_str("::");
                        name.push_str(next_name);
                        self.advance();
                    } else {
                        return Err(Error::Syntax {
                            line: self.current_token().line,
                            column: self.current_token().column,
                            message: "Expected identifier after '::'".to_string(),
                        });
                    }
                }
                
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

    fn parse_type_annotation(&mut self) -> Result<String> {
        // Parse type like: int, float, str, bool, [int], {str: int}
        let mut type_str = String::new();
        
        match &self.current_token().token_type {
            TokenType::LeftBracket => {
                // Array type like [int]
                self.advance();
                type_str.push('[');
                type_str.push_str(&self.parse_type_annotation()?);
                self.expect(&TokenType::RightBracket)?;
                type_str.push(']');
            }
            TokenType::LeftBrace => {
                // Map type like {str: int}
                self.advance();
                type_str.push('{');
                type_str.push_str(&self.parse_type_annotation()?);
                self.expect(&TokenType::Colon)?;
                type_str.push(':');
                type_str.push_str(&self.parse_type_annotation()?);
                self.expect(&TokenType::RightBrace)?;
                type_str.push('}');
            }
            TokenType::Identifier(ident) => {
                // Primitive or generic type
                type_str.push_str(ident);
                self.advance();
                
                // Check for generic parameters like Vector<int>
                if self.current_token().token_type == TokenType::Less {
                    self.advance();
                    type_str.push('<');
                    type_str.push_str(&self.parse_type_annotation()?);
                    self.expect(&TokenType::Greater)?;
                    type_str.push('>');
                }
            }
            _ => {
                return Err(Error::Syntax {
                    line: self.current_token().line,
                    column: self.current_token().column,
                    message: "Expected type annotation".to_string(),
                });
            }
        }
        
        Ok(type_str)
    }

    fn is_builtin_type_name(name: &str) -> bool {
        matches!(
            name,
            "int"
                | "float"
                | "str"
                | "string"
                | "bool"
                | "ptr"
                | "i32"
                | "i64"
                | "f32"
                | "f64"
                | "any"
        ) || name.starts_with('[')
            || name.starts_with('{')
            || name.starts_with("Vector")
            || name.starts_with("Matrix")
            || name.starts_with("Tensor")
    }
}

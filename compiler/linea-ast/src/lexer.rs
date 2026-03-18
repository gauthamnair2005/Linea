use linea_core::{Result, Error};
use crate::token::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace_and_comments();

            if self.is_at_end() {
                tokens.push(Token::new(TokenType::Eof, self.line, self.column));
                break;
            }

            let token = self.next_token()?;
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token> {
        let line = self.line;
        let column = self.column;
        let ch = self.current_char();

        match ch {
            '+' => { self.advance(); Ok(Token::new(TokenType::Plus, line, column)) }
            '-' => { 
                self.advance();
                if self.current_char() == '>' {
                    self.advance();
                    Ok(Token::new(TokenType::Arrow, line, column))
                } else {
                    Ok(Token::new(TokenType::Minus, line, column))
                }
            }
            '*' => { self.advance(); Ok(Token::new(TokenType::Star, line, column)) }
            '/' => { self.advance(); Ok(Token::new(TokenType::Slash, line, column)) }
            '%' => { self.advance(); Ok(Token::new(TokenType::Percent, line, column)) }
            '^' => { self.advance(); Ok(Token::new(TokenType::Caret, line, column)) }
            '=' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::new(TokenType::EqualEqual, line, column))
                } else if self.current_char() == '>' {
                    self.advance();
                    Ok(Token::new(TokenType::FatArrow, line, column))
                } else {
                    Ok(Token::new(TokenType::Equal, line, column))
                }
            }
            '!' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::new(TokenType::NotEqual, line, column))
                } else {
                    Ok(Token::new(TokenType::Exclamation, line, column))
                }
            }
            '<' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::new(TokenType::LessEqual, line, column))
                } else {
                    Ok(Token::new(TokenType::Less, line, column))
                }
            }
            '>' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::new(TokenType::GreaterEqual, line, column))
                } else {
                    Ok(Token::new(TokenType::Greater, line, column))
                }
            }
            '&' => {
                self.advance();
                if self.current_char() == '&' {
                    self.advance();
                    Ok(Token::new(TokenType::And, line, column))
                } else {
                    // Single & for pointers (address-of)
                    Ok(Token::new(TokenType::Ampersand, line, column))
                }
            }
            '|' => {
                self.advance();
                if self.current_char() == '|' {
                    self.advance();
                    Ok(Token::new(TokenType::Or, line, column))
                } else {
                    Ok(Token::new(TokenType::Pipe, line, column))
                }
            }
            '.' => { self.advance(); Ok(Token::new(TokenType::Dot, line, column)) }
            ',' => { self.advance(); Ok(Token::new(TokenType::Comma, line, column)) }
            ';' => { self.advance(); Ok(Token::new(TokenType::Semicolon, line, column)) }
            ':' => { 
                self.advance(); 
                if self.current_char() == ':' {
                    self.advance();
                    Ok(Token::new(TokenType::DoubleColon, line, column))
                } else {
                    Ok(Token::new(TokenType::Colon, line, column))
                }
            }
            '~' => { self.advance(); Ok(Token::new(TokenType::Tilde, line, column)) }
            '(' => { self.advance(); Ok(Token::new(TokenType::LeftParen, line, column)) }
            ')' => { self.advance(); Ok(Token::new(TokenType::RightParen, line, column)) }
            '{' => { self.advance(); Ok(Token::new(TokenType::LeftBrace, line, column)) }
            '}' => { self.advance(); Ok(Token::new(TokenType::RightBrace, line, column)) }
            '[' => { self.advance(); Ok(Token::new(TokenType::LeftBracket, line, column)) }
            ']' => { self.advance(); Ok(Token::new(TokenType::RightBracket, line, column)) }
            '"' => self.read_string(),
            '\'' => self.read_string(),
            '@' => {
                // Check if @ is followed by an identifier (decorator) or standalone (type operator)
                // Peek ahead without advancing
                let next_pos = self.position + 1;
                let is_decorator = next_pos < self.input.len() && 
                    (self.input[next_pos].is_alphabetic() || self.input[next_pos] == '_');
                
                self.advance();
                if is_decorator {
                    // It's a decorator like @gpu, @async
                    let ident = self.read_identifier();
                    Ok(Token::new(TokenType::Identifier(format!("@{}", ident)), line, column))
                } else {
                    // It's the type operator @
                    Ok(Token::new(TokenType::At, line, column))
                }
            }
            _ if ch.is_alphabetic() || ch == '_' => {
                let ident = self.read_identifier();
                let token_type = match ident.as_str() {
                    "var" => TokenType::Var,
                    "varUpd" => TokenType::VarUpdate,
                    "display" => TokenType::Display,
                    "for" => TokenType::For,
                    "from" => TokenType::From,
                    "use" => TokenType::Use,
                    "import" => TokenType::Import,
                    "func" => TokenType::Function,
                    "macro_rules" => TokenType::MacroRules,
                    "return" => TokenType::Return,
                    "if" => TokenType::If,
                    "else" => TokenType::Else,
                    "while" => TokenType::While,
                    "break" => TokenType::Break,
                    "continue" => TokenType::Continue,
                    "class" => TokenType::Class,
                    "obj" => TokenType::Obj,
                    "this" => TokenType::This,
                    "super" => TokenType::Super,
                    "typeCast" => TokenType::TypeCast,
                    "True" => TokenType::True,
                    "False" => TokenType::False,
                    "Yes" => TokenType::Yes,
                    "No" => TokenType::No,
                    "not" => TokenType::Not,
                    _ => TokenType::Identifier(ident),
                };
                Ok(Token::new(token_type, line, column))
            }
            _ if ch.is_numeric() => self.read_number(),
            _ => Err(Error::Syntax { line, column, message: format!("Unexpected character: '{}'", ch) })
        }
    }

    fn read_string(&mut self) -> Result<Token> {
        let line = self.line;
        let column = self.column;
        let quote = self.current_char();
        self.advance();

        let mut value = String::new();
        while !self.is_at_end() && self.current_char() != quote {
            if self.current_char() == '\\' {
                self.advance();
                match self.current_char() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    '\'' => value.push('\''),
                    _ => {
                        value.push('\\');
                        value.push(self.current_char());
                    }
                }
                self.advance();
            } else {
                value.push(self.current_char());
                self.advance();
            }
        }

        if self.is_at_end() {
            return Err(Error::Syntax { line, column, message: "Unterminated string".to_string() });
        }

        self.advance();
        Ok(Token::new(TokenType::String(value), line, column))
    }

    fn read_identifier(&mut self) -> String {
        let mut value = String::new();
        while !self.is_at_end() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            value.push(self.current_char());
            self.advance();
        }
        value
    }

    fn read_number(&mut self) -> Result<Token> {
        let line = self.line;
        let column = self.column;
        let mut value = String::new();
        let mut is_float = false;

        while !self.is_at_end() && (self.current_char().is_numeric() || self.current_char() == '.') {
            if self.current_char() == '.' {
                if is_float {
                    break;
                }
                is_float = true;
            }
            value.push(self.current_char());
            self.advance();
        }

        if is_float {
            let num = value.parse::<f64>()
                .map_err(|_| Error::Syntax { line, column, message: "Invalid float".to_string() })?;
            Ok(Token::new(TokenType::Float(num), line, column))
        } else {
            let num = value.parse::<i64>()
                .map_err(|_| Error::Syntax { line, column, message: "Invalid integer".to_string() })?;
            Ok(Token::new(TokenType::Integer(num), line, column))
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while !self.is_at_end() {
            match self.current_char() {
                ' ' | '\t' | '\r' => self.advance(),
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                }
                '#' => {
                    while !self.is_at_end() && self.current_char() != '\n' {
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }

    fn current_char(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.position]
        }
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position += 1;
            self.column += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
}

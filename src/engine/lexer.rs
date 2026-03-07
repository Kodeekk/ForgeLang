//! ForgeLang Lexer - Tokenizes source code into tokens

use std::fmt;
use std::rc::Rc;
use crate::error::{ErrorCollector, CompileError, Span, codes};

#[derive(Debug, Clone, PartialEq)]
pub enum StringInterpSegment {
    Text(String),
    Expr(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    
    // Identifiers and keywords
    Ident(String),
    
    // Keywords
    Fn,
    Var,
    Const,
    Class,
    Interface,
    Implement,
    Module,
    Enum,
    Type,
    For,
    While,
    If,
    Else,
    Match,
    Return,
    Import,
    As,
    From,
    In,
    True,
    False,
    Void,
    Implements,
    Self_,
    Underscore,
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    EqEq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,
    Not,
    Arrow,      // ->
    FatArrow,   // =>
    
    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Colon,
    Semi,
    Dot,
    
    // Special
    Eof,
    Newline,
    Comment(String),
    InterpolatedString(Vec<StringInterpSegment>),
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Int(i) => write!(f, "Int({})", i),
            TokenType::Float(fl) => write!(f, "Float({})", fl),
            TokenType::Str(s) => write!(f, "Str({})", s),
            TokenType::Bool(b) => write!(f, "Bool({})", b),
            TokenType::Ident(s) => write!(f, "Ident({})", s),
            TokenType::Fn => write!(f, "fn"),
            TokenType::Var => write!(f, "var"),
            TokenType::Const => write!(f, "const"),
            TokenType::Class => write!(f, "class"),
            TokenType::Interface => write!(f, "interface"),
            TokenType::Implement => write!(f, "implement"),
            TokenType::Module => write!(f, "module"),
            TokenType::Enum => write!(f, "enum"),
            TokenType::Type => write!(f, "type"),
            TokenType::For => write!(f, "for"),
            TokenType::While => write!(f, "while"),
            TokenType::If => write!(f, "if"),
            TokenType::Else => write!(f, "else"),
            TokenType::Match => write!(f, "match"),
            TokenType::Return => write!(f, "return"),
            TokenType::Import => write!(f, "import"),
            TokenType::As => write!(f, "as"),
            TokenType::From => write!(f, "from"),
            TokenType::In => write!(f, "in"),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::Void => write!(f, "void"),
            TokenType::Implements => write!(f, "implements"),
            TokenType::Self_ => write!(f, "self"),
            TokenType::Underscore => write!(f, "_"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Star => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Percent => write!(f, "%"),
            TokenType::Eq => write!(f, "="),
            TokenType::EqEq => write!(f, "=="),
            TokenType::NotEq => write!(f, "!="),
            TokenType::Lt => write!(f, "<"),
            TokenType::LtEq => write!(f, "<="),
            TokenType::Gt => write!(f, ">"),
            TokenType::GtEq => write!(f, ">="),
            TokenType::And => write!(f, "&&"),
            TokenType::Or => write!(f, "||"),
            TokenType::Not => write!(f, "!"),
            TokenType::Arrow => write!(f, "->"),
            TokenType::FatArrow => write!(f, "=>"),
            TokenType::LParen => write!(f, "("),
            TokenType::RParen => write!(f, ")"),
            TokenType::LBrace => write!(f, "{{"),
            TokenType::RBrace => write!(f, "}}"),
            TokenType::LBracket => write!(f, "["),
            TokenType::RBracket => write!(f, "]"),
            TokenType::Comma => write!(f, ","),
            TokenType::Colon => write!(f, ":"),
            TokenType::Semi => write!(f, ";"),
            TokenType::Dot => write!(f, "."),
            TokenType::Eof => write!(f, "EOF"),
            TokenType::Newline => write!(f, "NEWLINE"),
            TokenType::Comment(c) => write!(f, "Comment({})", c),
            TokenType::InterpolatedString(_) => write!(f, "InterpolatedString"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Token { token_type, line, column }
    }
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
    errors: ErrorCollector,
    source: Rc<String>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let source = Rc::new(input.to_string());
        Lexer {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
            errors: ErrorCollector::new().with_source(Rc::clone(&source)),
            source,
        }
    }

    fn current_span(&self) -> Span {
        Span::new(self.line, self.column, Rc::clone(&self.source))
    }
    
    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }
    
    fn peek_next(&self) -> Option<char> {
        self.input.get(self.pos + 1).copied()
    }
    
    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        if let Some(c) = ch {
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        ch
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            match ch {
                ' ' | '\t' | '\r' | '\n' => { self.advance(); }
                '/' if self.peek_next() == Some('/') => {
                    // Skip comment until end of line
                    while let Some(c) = self.peek() {
                        if c == '\n' { break; }
                        self.advance();
                    }
                }
                // Skip box-drawing and other special Unicode characters often used in comments
                '╔' | '╗' | '╚' | '╝' | '║' | '═' | 
                '╠' | '╣' | '╦' | '╩' | '╬' | '│' | '─' | '┌' | '┐' | '└' | '┘' |
                '●' | '★' | '☆' | '✓' | '✗' | '→' | '←' | '↑' | '↓' |
                '—' | '–' | '…' | '•' | '°' | '±' | '×' | '÷' => {
                    // Skip until end of line (these are decorative comment characters)
                    while let Some(c) = self.peek() {
                        if c == '\n' { break; }
                        self.advance();
                    }
                }
                // Skip other Unicode whitespace
                c if c.is_whitespace() => { self.advance(); }
                _ => return,
            }
        }
    }
    
    fn read_string(&mut self) -> Result<String, String> {
        self.advance(); // skip opening quote
        let mut result = String::new();

        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance();
                return Ok(result);
            }
            if ch == '\\' {
                self.advance();
                if let Some(escape) = self.peek() {
                    match escape {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        '{' => result.push('{'),
                        '}' => result.push('}'),
                        _ => {
                            result.push('\\');
                            result.push(escape);
                        }
                    }
                    self.advance();
                }
            } else {
                result.push(ch);
                self.advance();
            }
        }
        Err("Unterminated string".to_string())
    }

    fn read_interpolated_string(&mut self) -> Result<Vec<StringInterpSegment>, String> {
        self.advance(); // skip opening quote
        let mut segments = Vec::new();
        let mut current_text = String::new();
        
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance();
                if !current_text.is_empty() {
                    segments.push(StringInterpSegment::Text(current_text));
                }
                return Ok(segments);
            }
            if ch == '\\' {
                self.advance();
                if let Some(escape) = self.peek() {
                    match escape {
                        'n' => current_text.push('\n'),
                        't' => current_text.push('\t'),
                        'r' => current_text.push('\r'),
                        '\\' => current_text.push('\\'),
                        '"' => current_text.push('"'),
                        '{' => current_text.push('{'),
                        '}' => current_text.push('}'),
                        _ => {
                            current_text.push('\\');
                            current_text.push(escape);
                        }
                    }
                    self.advance();
                }
            } else if ch == '{' {
                // Start of interpolation
                if !current_text.is_empty() {
                    segments.push(StringInterpSegment::Text(current_text));
                    current_text = String::new();
                }
                self.advance(); // consume '{'
                
                // Read the expression until '}'
                let expr_text = self.read_interpolation_expr()?;
                segments.push(StringInterpSegment::Expr(expr_text));
            } else {
                current_text.push(ch);
                self.advance();
            }
        }
        Err("Unterminated string".to_string())
    }
    
    fn read_interpolation_expr(&mut self) -> Result<String, String> {
        let mut result = String::new();
        let mut brace_depth = 1;
        
        while let Some(ch) = self.peek() {
            if ch == '}' {
                brace_depth -= 1;
                if brace_depth == 0 {
                    self.advance(); // consume '}'
                    return Ok(result.trim().to_string());
                }
                result.push(ch);
                self.advance();
            } else if ch == '{' {
                brace_depth += 1;
                result.push(ch);
                self.advance();
            } else if ch == '"' {
                // String inside interpolation - skip it
                result.push(ch);
                self.advance();
                while let Some(inner_ch) = self.peek() {
                    if inner_ch == '"' {
                        result.push(inner_ch);
                        self.advance();
                        break;
                    }
                    if inner_ch == '\\' {
                        result.push(inner_ch);
                        self.advance();
                        if let Some(escape) = self.peek() {
                            result.push(escape);
                            self.advance();
                        }
                    } else {
                        result.push(inner_ch);
                        self.advance();
                    }
                }
            } else {
                result.push(ch);
                self.advance();
            }
        }
        Err("Unterminated interpolation".to_string())
    }

    fn read_number(&mut self) -> TokenType {
        let mut num_str = String::new();
        let mut is_float = false;
        
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' && !is_float {
                if let Some(next) = self.peek_next() {
                    if next.is_ascii_digit() {
                        is_float = true;
                        num_str.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        if is_float {
            TokenType::Float(num_str.parse().unwrap_or(0.0))
        } else {
            TokenType::Int(num_str.parse().unwrap_or(0))
        }
    }
    
    fn read_ident(&mut self) -> TokenType {
        let mut ident = String::new();
        
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        // Check for keywords
        match ident.as_str() {
            "fn" => TokenType::Fn,
            "var" => TokenType::Var,
            "const" => TokenType::Const,
            "class" => TokenType::Class,
            "interface" => TokenType::Interface,
            "implement" => TokenType::Implement,
            "module" => TokenType::Module,
            "enum" => TokenType::Enum,
            "type" => TokenType::Type,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "match" => TokenType::Match,
            "return" => TokenType::Return,
            "import" => TokenType::Import,
            "as" => TokenType::As,
            "from" => TokenType::From,
            "in" => TokenType::In,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "void" => TokenType::Void,
            "implements" => TokenType::Implements,
            "self" => TokenType::Self_,
            "_" => TokenType::Underscore,
            _ => TokenType::Ident(ident),
        }
    }
    
    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        
        let line = self.line;
        let column = self.column;
        
        let ch = match self.peek() {
            Some(c) => c,
            None => return Ok(Token::new(TokenType::Eof, line, column)),
        };
        
        // String literal
        if ch == '"' {
            // Try to read as interpolated string
            match self.read_interpolated_string() {
                Ok(segments) => {
                    return Ok(Token::new(TokenType::InterpolatedString(segments), line, column));
                }
                Err(e) => {
                    // Fall back to regular string
                    return Err(e);
                }
            }
        }
        
        // Number literal
        if ch.is_ascii_digit() {
            return Ok(Token::new(self.read_number(), line, column));
        }
        
        // Identifier or keyword
        if ch.is_alphabetic() || ch == '_' {
            return Ok(Token::new(self.read_ident(), line, column));
        }
        
        // Operators and delimiters
        self.advance();
        let token_type = match ch {
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            '{' => TokenType::LBrace,
            '}' => TokenType::RBrace,
            '[' => TokenType::LBracket,
            ']' => TokenType::RBracket,
            ',' => TokenType::Comma,
            ':' => TokenType::Colon,
            ';' => TokenType::Semi,
            '.' => TokenType::Dot,
            '+' => TokenType::Plus,
            '-' => {
                if self.peek() == Some('>') {
                    self.advance();
                    TokenType::Arrow
                } else {
                    TokenType::Minus
                }
            }
            '*' => TokenType::Star,
            '/' => TokenType::Slash,
            '%' => TokenType::Percent,
            '=' => {
                if self.peek() == Some('>') {
                    self.advance();
                    TokenType::FatArrow
                } else if self.peek() == Some('=') {
                    self.advance();
                    TokenType::EqEq
                } else {
                    TokenType::Eq
                }
            }
            '!' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenType::NotEq
                } else {
                    TokenType::Not
                }
            }
            '<' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenType::LtEq
                } else {
                    TokenType::Lt
                }
            }
            '>' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenType::GtEq
                } else {
                    TokenType::Gt
                }
            }
            '&' => {
                if self.peek() == Some('&') {
                    self.advance();
                    TokenType::And
                } else {
                    self.errors.error(
                        CompileError::lexical_error(codes::INVALID_CHAR, "Unexpected character: &")
                            .with_span(self.current_span())
                            .with_hint("Did you mean '&&' for logical AND?")
                    );
                    TokenType::And // Return a valid token to continue parsing
                }
            }
            '|' => {
                if self.peek() == Some('|') {
                    self.advance();
                    TokenType::Or
                } else {
                    self.errors.error(
                        CompileError::lexical_error(codes::INVALID_CHAR, "Unexpected character: |")
                            .with_span(self.current_span())
                            .with_hint("Did you mean '||' for logical OR?")
                    );
                    TokenType::Or // Return a valid token to continue parsing
                }
            }
            _ => {
                self.errors.error(
                    CompileError::lexical_error(codes::INVALID_CHAR, format!("Unexpected character: {}", ch))
                        .with_span(self.current_span())
                );
                self.advance();
                return self.next_token(); // Skip this character and try the next
            }
        };

        Ok(Token::new(token_type, line, column))
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, &ErrorCollector> {
        let mut tokens = Vec::new();

        loop {
            match self.next_token() {
                Ok(token) => {
                    let is_eof = token.token_type == TokenType::Eof;
                    tokens.push(token);
                    if is_eof {
                        break;
                    }
                }
                Err(e) => {
                    // Convert to our error type
                    let span = self.current_span();
                    self.errors.error(
                        CompileError::lexical_error(codes::UNEXPECTED_TOKEN, e)
                            .with_span(span)
                    );
                    // Try to continue by skipping this character
                    if self.pos >= self.input.len() {
                        break;
                    }
                    self.advance();
                }
            }
        }

        if self.errors.has_errors() {
            Err(&self.errors)
        } else {
            Ok(tokens)
        }
    }

    /// Get the error collector (for accessing errors after tokenization)
    pub fn into_errors(self) -> ErrorCollector {
        self.errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("var x: int = 42;");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::Var);
        assert_eq!(tokens[1].token_type, TokenType::Ident("x".to_string()));
        assert_eq!(tokens[2].token_type, TokenType::Colon);
        assert_eq!(tokens[3].token_type, TokenType::Ident("int".to_string()));
        assert_eq!(tokens[4].token_type, TokenType::Eq);
        assert_eq!(tokens[5].token_type, TokenType::Int(42));
        assert_eq!(tokens[6].token_type, TokenType::Semi);
    }
}

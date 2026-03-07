//! ForgeLang Parser - Builds AST from tokens

use crate::lexer::{Token, TokenType, Lexer, StringInterpSegment};
use crate::ast::*;
use crate::error::{ErrorCollector, CompileError, Span, codes};
use std::rc::Rc;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: ErrorCollector,
    source: Rc<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, source: Rc<String>) -> Self {
        Parser {
            tokens,
            current: 0,
            errors: ErrorCollector::new().with_source(Rc::clone(&source)),
            source,
        }
    }

    fn current_span(&self) -> Span {
        let token = self.peek();
        Span::new(token.line, token.column, Rc::clone(&self.source))
    }
    
    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or({
            &Token { token_type: TokenType::Eof, line: 0, column: 0 }
        })
    }
    
    fn previous(&self) -> &Token {
        self.tokens.get(self.current.saturating_sub(1)).unwrap_or({
            &Token { token_type: TokenType::Eof, line: 0, column: 0 }
        })
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
    
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() { return false; }
        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(&token_type)
    }
    
    fn matches(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn expect(&mut self, token_type: TokenType, message: &str) -> Result<(), String> {
        if self.check(token_type.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {} at line {}: found '{}'", 
                message, self.peek().line, self.peek().token_type))
        }
    }
    
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semi {
                return;
            }
            match self.peek().token_type {
                TokenType::Fn | TokenType::Class | TokenType::Interface |
                TokenType::Module | TokenType::Enum | TokenType::Type |
                TokenType::Var | TokenType::Const | TokenType::For |
                TokenType::While | TokenType::If | TokenType::Return => return,
                _ => {}
            }
            self.advance();
        }
    }
    
    pub fn parse(&mut self) -> Result<Program, &ErrorCollector> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    // Convert to our error type
                    let span = self.current_span();
                    self.errors.error(
                        CompileError::syntax_error(codes::UNEXPECTED_TOKEN, e)
                            .with_span(span)
                    );
                    // Try to synchronize and continue
                    self.synchronize();
                }
            }
        }

        if self.errors.has_errors() {
            Err(&self.errors)
        } else {
            Ok(Program { statements })
        }
    }

    /// Get the error collector (for accessing errors after parsing)
    pub fn into_errors(self) -> ErrorCollector {
        self.errors
    }
    
    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.matches(TokenType::Import) {
            return self.import_declaration();
        }
        if self.matches(TokenType::Module) {
            return self.module_declaration();
        }
        if self.matches(TokenType::Type) {
            return self.type_alias_declaration();
        }
        if self.matches(TokenType::Const) {
            return self.const_declaration();
        }
        if self.matches(TokenType::Var) {
            return self.var_declaration();
        }
        if self.matches(TokenType::Fn) {
            return self.function_declaration();
        }
        if self.matches(TokenType::Class) {
            return self.class_declaration();
        }
        if self.matches(TokenType::Interface) {
            return self.interface_declaration();
        }
        if self.matches(TokenType::Implement) {
            return self.implement_declaration();
        }
        if self.matches(TokenType::Enum) {
            return self.enum_declaration();
        }
        self.statement()
    }
    
    fn import_declaration(&mut self) -> Result<Stmt, String> {
        // import [fn1, fn2 as f2] from std.module;
        // import std.module;
        // import std.module as alias;

        if self.check(TokenType::LBracket) {
            self.advance(); // consume [
            let mut items = Vec::new();

            loop {
                let name_token = self.advance();
                let name = match &name_token.token_type {
                    TokenType::Ident(s) => s.clone(),
                    _ => return Err("Expected identifier in import list".to_string()),
                };

                if self.matches(TokenType::As) {
                    let alias_token = self.advance();
                    let alias = match &alias_token.token_type {
                        TokenType::Ident(s) => s.clone(),
                        _ => return Err("Expected alias identifier".to_string()),
                    };
                    items.push(ImportItem::Aliased { name, alias });
                } else {
                    items.push(ImportItem::Simple(name));
                }

                if !self.matches(TokenType::Comma) {
                    break;
                }
            }

            self.expect(TokenType::RBracket, "close import list")?;
            self.expect(TokenType::From, "expected 'from'")?;

            let module = self.parse_module_path()?;
            self.expect(TokenType::Semi, "end import")?;

            Ok(Stmt::Import { module, alias: None, items: Some(items), location: None })
        } else {
            let module = self.parse_module_path()?;

            let alias = if self.matches(TokenType::As) {
                let alias_token = self.advance();
                match &alias_token.token_type {
                    TokenType::Ident(s) => Some(s.clone()),
                    _ => return Err("Expected alias identifier".to_string()),
                }
            } else {
                None
            };

            self.expect(TokenType::Semi, "end import")?;
            Ok(Stmt::Import { module, alias, items: None, location: None })
        }
    }

    fn module_declaration(&mut self) -> Result<Stmt, String> {
        let path = self.parse_module_path()?;
        self.expect(TokenType::Semi, "end module declaration")?;
        Ok(Stmt::Module { path, location: None })
    }

    fn type_alias_declaration(&mut self) -> Result<Stmt, String> {
        let name_token = self.advance();
        let name = match &name_token.token_type {
            TokenType::Ident(s) => s.clone(),
            _ => return Err("Expected type alias name".to_string()),
        };

        // Parse optional generic parameters <T, A, ...>
        let mut type_params = Vec::new();
        if self.matches(TokenType::Lt) {
            loop {
                let param_token = self.advance();
                let param = match &param_token.token_type {
                    TokenType::Ident(s) => s.clone(),
                    _ => return Err("Expected generic type parameter".to_string()),
                };
                type_params.push(param);

                // Skip type bounds like `: Comparable<T>`
                if self.matches(TokenType::Colon) {
                    // Skip until we hit a comma or >, tracking angle bracket depth
                    let mut depth = 0;
                    while !self.is_at_end() {
                        if self.check(TokenType::Lt) {
                            depth += 1;
                            self.advance();
                        } else if self.check(TokenType::Gt) {
                            if depth == 0 {
                                break;  // This is the closing >
                            }
                            depth -= 1;
                            self.advance();
                        } else if self.check(TokenType::Comma) && depth == 0 {
                            break;  // This is the separator for the next type parameter
                        } else {
                            self.advance();
                        }
                    }
                }

                if !self.matches(TokenType::Comma) {
                    break;
                }
            }
            self.expect(TokenType::Gt, "expected '>' after generic parameters")?;
        }

        self.expect(TokenType::Eq, "expected '=' after type alias name")?;
        let alias_type = self.parse_type()?;
        self.expect(TokenType::Semi, "end type alias declaration")?;

        Ok(Stmt::TypeAlias { name, type_params, alias_type, location: None })
    }
    
    fn parse_module_path(&mut self) -> Result<String, String> {
        let mut path = String::new();
        
        let first = self.advance();
        match &first.token_type {
            TokenType::Ident(s) => path.push_str(s),
            _ => return Err("Expected identifier in module path".to_string()),
        }
        
        while self.matches(TokenType::Dot) {
            path.push('.');
            let part = self.advance();
            match &part.token_type {
                TokenType::Ident(s) => path.push_str(s),
                _ => return Err("Expected identifier after '.' in module path".to_string()),
            }
        }
        
        Ok(path)
    }
    
    fn const_declaration(&mut self) -> Result<Stmt, String> {
        let name_token = self.advance();
        let name = match &name_token.token_type {
            TokenType::Ident(s) => s.clone(),
            _ => return Err("Expected constant name".to_string()),
        };
        let location = Location::new(name_token.line, Rc::clone(&self.source));

        let const_type = if self.matches(TokenType::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect(TokenType::Eq, "expected '=' after constant name")?;
        let value = self.expression()?;
        self.expect(TokenType::Semi, "end constant declaration")?;

        Ok(Stmt::ConstDecl { name, const_type, value, location: Some(location) })
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let pattern = self.parse_pattern()?;
        let location = Location::new(self.previous().line, Rc::clone(&self.source));

        let var_type = if self.matches(TokenType::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };

        let initializer = if self.matches(TokenType::Eq) {
            Some(self.expression()?)
        } else {
            None
        };

        self.expect(TokenType::Semi, "end variable declaration")?;

        Ok(Stmt::VarDecl { pattern, var_type, initializer, location: Some(location) })
    }

    fn parse_pattern(&mut self) -> Result<Pattern, String> {
        let token = self.peek().token_type.clone();

        match token {
            TokenType::Ident(s) => {
                self.advance();
                Ok(Pattern::Ident(s))
            }
            TokenType::Underscore => {
                self.advance();
                Ok(Pattern::Underscore)
            }
            TokenType::LParen => {
                self.advance();
                let mut patterns = Vec::new();

                if !self.check(TokenType::RParen) {
                    loop {
                        patterns.push(self.parse_pattern()?);
                        if !self.matches(TokenType::Comma) {
                            break;
                        }
                    }
                }

                self.expect(TokenType::RParen, "expected ')' after tuple pattern")?;
                Ok(Pattern::Tuple(patterns))
            }
            _ => Err(format!("Expected pattern (identifier or tuple), got {:?}", token)),
        }
    }
    
    fn function_declaration(&mut self) -> Result<Stmt, String> {
        let name_token = self.advance();
        let name = match &name_token.token_type {
            TokenType::Ident(s) => s.clone(),
            _ => return Err("Expected function name".to_string()),
        };

        // Parse optional generic parameters <T, A, ...>
        let mut type_params = Vec::new();
        if self.matches(TokenType::Lt) {
            loop {
                let param_token = self.advance();
                let param = match &param_token.token_type {
                    TokenType::Ident(s) => s.clone(),
                    _ => return Err("Expected generic type parameter".to_string()),
                };
                type_params.push(param);

                // Skip type bounds like `: Comparable<T>`
                if self.matches(TokenType::Colon) {
                    // Skip until we hit a comma or >, tracking angle bracket depth
                    let mut depth = 0;
                    while !self.is_at_end() {
                        if self.check(TokenType::Lt) {
                            depth += 1;
                            self.advance();
                        } else if self.check(TokenType::Gt) {
                            if depth == 0 {
                                break;  // This is the closing >
                            }
                            depth -= 1;
                            self.advance();
                        } else if self.check(TokenType::Comma) && depth == 0 {
                            break;  // This is the separator for the next type parameter
                        } else {
                            self.advance();
                        }
                    }
                }

                if !self.matches(TokenType::Comma) {
                    break;
                }
            }
            self.expect(TokenType::Gt, "expected '>' after generic parameters")?;
        }

        self.expect(TokenType::LParen, "expected '(' after function name")?;
        let params = self.parse_params()?;
        self.expect(TokenType::RParen, "expected ')' after parameters")?;

        let return_type = if self.matches(TokenType::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect(TokenType::LBrace, "expected '{' before function body")?;
        let body = self.parse_block()?;
        self.expect(TokenType::RBrace, "expected '}' after function body")?;

        Ok(Stmt::FnDecl { name, type_params, params, return_type, body, location: None })
    }
    
    fn parse_params(&mut self) -> Result<Vec<Param>, String> {
        let mut params = Vec::new();

        if self.check(TokenType::RParen) {
            return Ok(params);
        }

        loop {
            let name_token = self.advance();
            let name = match &name_token.token_type {
                TokenType::Ident(s) => s.clone(),
                TokenType::Self_ => "self".to_string(),
                _ => return Err("Expected parameter name".to_string()),
            };

            let param_type = if self.matches(TokenType::Colon) {
                Some(self.parse_type()?)
            } else {
                None
            };

            params.push(Param { name, param_type });

            if !self.matches(TokenType::Comma) {
                break;
            }
        }

        Ok(params)
    }
    
    fn parse_type(&mut self) -> Result<TypeAnnotation, String> {
        let token = self.peek().token_type.clone();

        match token {
            TokenType::Ident(ref s) => {
                self.advance();
                if s == "int" {
                    Ok(TypeAnnotation::Int)
                } else if s == "float" {
                    Ok(TypeAnnotation::Float)
                } else if s == "str" {
                    Ok(TypeAnnotation::Str)
                } else if s == "bool" {
                    Ok(TypeAnnotation::Bool)
                } else if s == "void" {
                    Ok(TypeAnnotation::Void)
                } else if s == "float" || s == "f64" {
                    Ok(TypeAnnotation::Float)
                } else if s == "int" || s == "i64" {
                    Ok(TypeAnnotation::Int)
                } else if s == "list" {
                    self.expect(TokenType::Lt, "expected '<' after 'list'")?;
                    let inner = Box::new(self.parse_type()?);
                    self.expect(TokenType::Gt, "expected '>' after list type")?;
                    Ok(TypeAnnotation::List(inner))
                } else {
                    // Check for generic type like ClassName<T, A>
                    if self.matches(TokenType::Lt) {
                        let mut args = Vec::new();
                        loop {
                            args.push(self.parse_type()?);
                            if !self.matches(TokenType::Comma) {
                                break;
                            }
                        }
                        self.expect(TokenType::Gt, "expected '>' after generic type arguments")?;
                        Ok(TypeAnnotation::GenericClass(s.clone(), args))
                    } else {
                        Ok(TypeAnnotation::Class(s.clone()))
                    }
                }
            }
            TokenType::Void => {
                self.advance();
                Ok(TypeAnnotation::Void)
            }
            TokenType::Self_ => {
                self.advance();
                Ok(TypeAnnotation::Self_)
            }
            TokenType::Fn => {
                self.advance();
                self.expect(TokenType::LParen, "expected '(' after 'fn'")?;
                let mut arg_types = Vec::new();

                if !self.check(TokenType::RParen) {
                    loop {
                        arg_types.push(self.parse_type()?);
                        if !self.matches(TokenType::Comma) {
                            break;
                        }
                    }
                }

                self.expect(TokenType::RParen, "expected ')' after function args")?;
                self.expect(TokenType::Arrow, "expected '->' after function args")?;
                let ret_type = Box::new(self.parse_type()?);

                Ok(TypeAnnotation::Fn(arg_types, ret_type))
            }
            TokenType::LParen => {
                self.advance();
                let mut types = Vec::new();

                if !self.check(TokenType::RParen) {
                    loop {
                        types.push(self.parse_type()?);
                        if !self.matches(TokenType::Comma) {
                            break;
                        }
                    }
                }

                self.expect(TokenType::RParen, "expected ')' after tuple type")?;
                Ok(TypeAnnotation::Tuple(types))
            }
            _ => Err(format!("Expected type, got {:?}", token)),
        }
    }
    
    fn class_declaration(&mut self) -> Result<Stmt, String> {
        let name_token = self.advance();
        let name = match &name_token.token_type {
            TokenType::Ident(s) => s.clone(),
            _ => return Err("Expected class name".to_string()),
        };

        // Parse optional generic parameters <T, A, ...>
        let mut type_params = Vec::new();
        if self.matches(TokenType::Lt) {
            loop {
                let param_token = self.advance();
                let param = match &param_token.token_type {
                    TokenType::Ident(s) => s.clone(),
                    _ => return Err("Expected generic type parameter".to_string()),
                };
                type_params.push(param);

                // Skip type bounds like `: Comparable<T>`
                if self.matches(TokenType::Colon) {
                    // Skip until we hit a comma or >, tracking angle bracket depth
                    let mut depth = 0;
                    while !self.is_at_end() {
                        if self.check(TokenType::Lt) {
                            depth += 1;
                            self.advance();
                        } else if self.check(TokenType::Gt) {
                            if depth == 0 {
                                break;  // This is the closing >
                            }
                            depth -= 1;
                            self.advance();
                        } else if self.check(TokenType::Comma) && depth == 0 {
                            break;  // This is the separator for the next type parameter
                        } else {
                            self.advance();
                        }
                    }
                }

                if !self.matches(TokenType::Comma) {
                    break;
                }
            }
            self.expect(TokenType::Gt, "expected '>' after generic parameters")?;
        }

        let mut implements = Vec::new();
        if self.matches(TokenType::Implements) {
            loop {
                let iface = self.parse_type()?;
                implements.push(iface);

                if !self.matches(TokenType::Comma) {
                    break;
                }
            }
        }

        self.expect(TokenType::LBrace, "expected '{' before class body")?;

        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            if self.check(TokenType::Var) {
                self.advance(); // consume 'var'
                let name_token = self.advance();
                let name = match &name_token.token_type {
                    TokenType::Ident(s) => s.clone(),
                    _ => return Err("Expected field name".to_string()),
                };

                let field_type = if self.matches(TokenType::Colon) {
                    Some(self.parse_type()?)
                } else {
                    None
                };

                self.expect(TokenType::Semi, "end field declaration")?;
                fields.push(Field { name, field_type });
            } else if self.check(TokenType::Fn) {
                methods.push(self.parse_method()?);
            } else {
                return Err("Expected 'var' or 'fn' in class body".to_string());
            }
        }

        self.expect(TokenType::RBrace, "expected '}' after class body")?;

        Ok(Stmt::ClassDecl { name, type_params, implements, fields, methods, location: None })
    }
    
    fn parse_method(&mut self) -> Result<Method, String> {
        self.advance(); // consume 'fn'

        let name_token = self.advance();
        let name = match &name_token.token_type {
            TokenType::Ident(s) => s.clone(),
            _ => return Err("Expected method name".to_string()),
        };

        // Parse optional generic parameters <T, A, ...>
        let mut type_params = Vec::new();
        if self.matches(TokenType::Lt) {
            loop {
                let param_token = self.advance();
                let param = match &param_token.token_type {
                    TokenType::Ident(s) => s.clone(),
                    _ => return Err("Expected generic type parameter".to_string()),
                };
                type_params.push(param);

                // Skip type bounds like `: Comparable<T>`
                if self.matches(TokenType::Colon) {
                    // Skip until we hit a comma or >, tracking angle bracket depth
                    let mut depth = 0;
                    while !self.is_at_end() {
                        if self.check(TokenType::Lt) {
                            depth += 1;
                            self.advance();
                        } else if self.check(TokenType::Gt) {
                            if depth == 0 {
                                break;  // This is the closing >
                            }
                            depth -= 1;
                            self.advance();
                        } else if self.check(TokenType::Comma) && depth == 0 {
                            break;  // This is the separator for the next type parameter
                        } else {
                            self.advance();
                        }
                    }
                }

                if !self.matches(TokenType::Comma) {
                    break;
                }
            }
            self.expect(TokenType::Gt, "expected '>' after generic parameters")?;
        }

        let is_static = name != "self";

        self.expect(TokenType::LParen, "expected '(' after method name")?;
        let params = self.parse_params()?;
        self.expect(TokenType::RParen, "expected ')' after parameters")?;

        let return_type = if self.matches(TokenType::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect(TokenType::LBrace, "expected '{' before method body")?;
        let body = self.parse_block()?;
        self.expect(TokenType::RBrace, "expected '}' after method body")?;

        Ok(Method { name, params, return_type, body, is_static, type_params })
    }
    
    fn interface_declaration(&mut self) -> Result<Stmt, String> {
        let name_token = self.advance();
        let name = match &name_token.token_type {
            TokenType::Ident(s) => s.clone(),
            _ => return Err("Expected interface name".to_string()),
        };

        // Parse optional generic parameters <T, A, ...>
        let mut type_params = Vec::new();
        if self.matches(TokenType::Lt) {
            loop {
                let param_token = self.advance();
                let param = match &param_token.token_type {
                    TokenType::Ident(s) => s.clone(),
                    _ => return Err("Expected generic type parameter".to_string()),
                };
                type_params.push(param);

                // Skip type bounds like `: Comparable<T>`
                if self.matches(TokenType::Colon) {
                    // Skip until we hit a comma or >, tracking angle bracket depth
                    let mut depth = 0;
                    while !self.is_at_end() {
                        if self.check(TokenType::Lt) {
                            depth += 1;
                            self.advance();
                        } else if self.check(TokenType::Gt) {
                            if depth == 0 {
                                break;  // This is the closing >
                            }
                            depth -= 1;
                            self.advance();
                        } else if self.check(TokenType::Comma) && depth == 0 {
                            break;  // This is the separator for the next type parameter
                        } else {
                            self.advance();
                        }
                    }
                }

                if !self.matches(TokenType::Comma) {
                    break;
                }
            }
            self.expect(TokenType::Gt, "expected '>' after generic parameters")?;
        }

        self.expect(TokenType::LBrace, "expected '{' before interface body")?;

        let mut methods = Vec::new();

        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            if self.check(TokenType::Fn) {
                self.advance(); // consume 'fn'

                let name_token = self.advance();
                let name = match &name_token.token_type {
                    TokenType::Ident(s) => s.clone(),
                    _ => return Err("Expected method name".to_string()),
                };

                self.expect(TokenType::LParen, "expected '(' after method name")?;
                let params = self.parse_params()?;
                self.expect(TokenType::RParen, "expected ')' after parameters")?;

                let return_type = if self.matches(TokenType::Arrow) {
                    Some(self.parse_type()?)
                } else {
                    None
                };

                self.expect(TokenType::Semi, "end interface method")?;

                methods.push(InterfaceMethod { name, params, return_type });
            } else {
                self.advance(); // skip unknown
            }
        }

        self.expect(TokenType::RBrace, "expected '}' after interface body")?;

        Ok(Stmt::InterfaceDecl { name, type_params, methods, location: None })
    }
    
    fn implement_declaration(&mut self) -> Result<Stmt, String> {
        let iface_token = self.advance();
        let interface_name = match &iface_token.token_type {
            TokenType::Ident(s) => s.clone(),
            _ => return Err("Expected interface name".to_string()),
        };

        self.expect(TokenType::For, "expected 'for' after interface name")?;

        let class_token = self.advance();
        let class_name = match &class_token.token_type {
            TokenType::Ident(s) => s.clone(),
            _ => return Err("Expected class name".to_string()),
        };

        self.expect(TokenType::LBrace, "expected '{' before implement body")?;

        let mut methods = Vec::new();

        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            methods.push(self.parse_method()?);
        }

        self.expect(TokenType::RBrace, "expected '}' after implement body")?;

        Ok(Stmt::ImplementDecl { interface_name, class_name, methods, location: None })
    }

    fn enum_declaration(&mut self) -> Result<Stmt, String> {
        let name_token = self.advance();
        let name = match &name_token.token_type {
            TokenType::Ident(s) => s.clone(),
            _ => return Err("Expected enum name".to_string()),
        };

        // Parse optional generic parameters <T, A, ...>
        let mut type_params = Vec::new();
        if self.matches(TokenType::Lt) {
            loop {
                let param_token = self.advance();
                let param = match &param_token.token_type {
                    TokenType::Ident(s) => s.clone(),
                    _ => return Err("Expected generic type parameter".to_string()),
                };
                type_params.push(param);

                // Skip type bounds like `: Comparable<T>`
                if self.matches(TokenType::Colon) {
                    // Skip until we hit a comma or >, tracking angle bracket depth
                    let mut depth = 0;
                    while !self.is_at_end() {
                        if self.check(TokenType::Lt) {
                            depth += 1;
                            self.advance();
                        } else if self.check(TokenType::Gt) {
                            if depth == 0 {
                                break;  // This is the closing >
                            }
                            depth -= 1;
                            self.advance();
                        } else if self.check(TokenType::Comma) && depth == 0 {
                            break;  // This is the separator for the next type parameter
                        } else {
                            self.advance();
                        }
                    }
                }

                if !self.matches(TokenType::Comma) {
                    break;
                }
            }
            self.expect(TokenType::Gt, "expected '>' after generic parameters")?;
        }

        self.expect(TokenType::LBrace, "expected '{' before enum body")?;

        let mut variants = Vec::new();

        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            // Parse variant name
            let variant_token = self.advance();
            let variant_name = match &variant_token.token_type {
                TokenType::Ident(s) => s.clone(),
                _ => return Err("Expected variant name".to_string()),
            };

            // Check for named fields like Variant(field: Type, ...)
            let mut fields = Vec::new();
            if self.matches(TokenType::LParen) {
                loop {
                    let field_name_token = self.advance();
                    let field_name = match &field_name_token.token_type {
                        TokenType::Ident(s) => s.clone(),
                        _ => return Err("Expected field name in variant".to_string()),
                    };

                    self.expect(TokenType::Colon, "expected ':' after field name in variant")?;
                    let field_type = Some(self.parse_type()?);
                    fields.push((field_name, field_type));

                    if !self.matches(TokenType::Comma) {
                        break;
                    }
                }
                self.expect(TokenType::RParen, "expected ')' after variant fields")?;
            }

            variants.push(EnumVariant { name: variant_name, fields });

            // Consume comma or end of enum
            if !self.matches(TokenType::Comma) {
                break;
            }
        }

        self.expect(TokenType::RBrace, "expected '}' after enum body")?;

        Ok(Stmt::EnumDecl { name, type_params, variants, location: None })
    }
    
    fn statement(&mut self) -> Result<Stmt, String> {
        if self.matches(TokenType::For) {
            return self.for_statement();
        }
        if self.matches(TokenType::While) {
            return self.while_statement();
        }
        if self.matches(TokenType::If) {
            return self.if_statement();
        }
        if self.matches(TokenType::Match) {
            return self.match_statement();
        }
        if self.matches(TokenType::Return) {
            return self.return_statement();
        }
        if self.matches(TokenType::LBrace) {
            return Ok(Stmt::Block(self.parse_block()?));
        }
        
        // Expression or assignment
        let expr = self.expression()?;

        if self.matches(TokenType::Eq) {
            // Assignment
            let value = self.expression()?;
            let location = Location::new(self.peek().line, Rc::clone(&self.source));
            self.expect(TokenType::Semi, "end assignment")?;

            return match expr {
                Expr::Ident(name) => {
                    Ok(Stmt::Assignment { name, value, location: Some(location) })
                }
                Expr::PropertyAccess { object, property } => {
                    Ok(Stmt::AssignmentField { object, field: property, value, location: Some(location) })
                }
                Expr::Index { object, index } => {
                    Ok(Stmt::AssignmentIndex { object, index, value, location: Some(location) })
                }
                _ => Err("Invalid assignment target".to_string()),
            }
        }

        self.expect(TokenType::Semi, "end statement")?;
        Ok(Stmt::ExprStmt(expr))
    }
    
    fn for_statement(&mut self) -> Result<Stmt, String> {
        let pattern = self.parse_pattern()?;

        self.expect(TokenType::In, "expected 'in' after for pattern")?;
        let iterable = self.expression()?;

        self.expect(TokenType::LBrace, "expected '{' before for body")?;
        let body = self.parse_block()?;
        self.expect(TokenType::RBrace, "expected '}' after for body")?;

        Ok(Stmt::For { pattern, iterable, body, location: None })
    }
    
    fn while_statement(&mut self) -> Result<Stmt, String> {
        let condition = self.expression()?;
        
        self.expect(TokenType::LBrace, "expected '{' before while body")?;
        let body = self.parse_block()?;
        self.expect(TokenType::RBrace, "expected '}' after while body")?;
        
        Ok(Stmt::While { condition, body, location: None })
    }
    
    fn if_statement(&mut self) -> Result<Stmt, String> {
        let condition = self.expression()?;
        
        self.expect(TokenType::LBrace, "expected '{' before if body")?;
        let then_branch = self.parse_block()?;
        self.expect(TokenType::RBrace, "expected '}' after if body")?;
        
        let mut else_if_branches = Vec::new();
        
        while self.matches(TokenType::Else) {
            if self.matches(TokenType::If) {
                let ei_condition = self.expression()?;
                self.expect(TokenType::LBrace, "expected '{' before else-if body")?;
                let ei_body = self.parse_block()?;
                self.expect(TokenType::RBrace, "expected '}' after else-if body")?;
                else_if_branches.push((ei_condition, ei_body));
            } else {
                self.expect(TokenType::LBrace, "expected '{' before else body")?;
                let else_body = self.parse_block()?;
                self.expect(TokenType::RBrace, "expected '}' after else body")?;
                
                return Ok(Stmt::If {
                    condition,
                    then_branch,
                    else_if_branches,
                    else_branch: Some(else_body), location: None,
                });
            }
        }
        
        Ok(Stmt::If {
            condition,
            then_branch,
            else_if_branches,
            else_branch: None, location: None,
        })
    }
    
    fn match_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;

        self.expect(TokenType::LBrace, "expected '{' before match arms")?;

        let mut arms = Vec::new();

        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            let pattern = self.parse_match_pattern()?;
            self.expect(TokenType::FatArrow, "expected '=>' after match pattern")?;

            // Match arm body: expression or block
            let body = if self.check(TokenType::LBrace) {
                self.advance();
                let block = self.parse_block()?;
                self.expect(TokenType::RBrace, "expected '}' after match arm")?;
                block
            } else {
                // Single expression (function call, etc.)
                let expr_stmt = self.expression()?;
                // Optionally consume semicolon if present
                self.matches(TokenType::Semi);
                vec![Stmt::ExprStmt(expr_stmt)]
            };

            // Consume comma separator if present
            self.matches(TokenType::Comma);
            
            arms.push(MatchArm { pattern, body });
        }

        self.expect(TokenType::RBrace, "expected '}' after match arms")?;

        Ok(Stmt::Match { expr, arms, location: None })
    }
    
    fn parse_match_pattern(&mut self) -> Result<MatchPattern, String> {
        if self.matches(TokenType::Underscore) {
            return Ok(MatchPattern::Underscore);
        }

        // Check for tuple pattern
        if self.check(TokenType::LParen) {
            self.advance(); // consume '('
            let mut patterns = Vec::new();

            if !self.check(TokenType::RParen) {
                loop {
                    patterns.push(self.parse_match_pattern()?);
                    if !self.matches(TokenType::Comma) {
                        break;
                    }
                }
            }

            self.expect(TokenType::RParen, "expected ')' after tuple pattern")?;
            return Ok(MatchPattern::Tuple(patterns));
        }

        // Check for literal
        let token_type = self.peek().token_type.clone();
        match token_type {
            TokenType::Int(i) => {
                self.advance();
                Ok(MatchPattern::Literal(Literal::Int(i)))
            }
            TokenType::Float(f) => {
                self.advance();
                Ok(MatchPattern::Literal(Literal::Float(f)))
            }
            TokenType::Str(s) => {
                self.advance();
                Ok(MatchPattern::Literal(Literal::Str(s)))
            }
            TokenType::True => {
                self.advance();
                Ok(MatchPattern::Literal(Literal::Bool(true)))
            }
            TokenType::False => {
                self.advance();
                Ok(MatchPattern::Literal(Literal::Bool(false)))
            }
            TokenType::Ident(s) => {
                self.advance();
                // Check for qualified name like LogLevel.Debug or Shape.Circle
                let mut full_name = s.clone();
                while self.matches(TokenType::Dot) {
                    let part_token = self.advance();
                    let part = match &part_token.token_type {
                        TokenType::Ident(p) => p.clone(),
                        _ => return Err("Expected identifier after '.' in match pattern".to_string()),
                    };
                    full_name.push('.');
                    full_name.push_str(&part);
                }
                
                // Check for variant pattern like Shape.Circle(r) or Shape.Circle(r, g, b)
                if self.matches(TokenType::LParen) {
                    let mut fields = Vec::new();
                    
                    if !self.check(TokenType::RParen) {
                        loop {
                            let field_token = self.advance();
                            let field = match &field_token.token_type {
                                TokenType::Ident(f) => f.clone(),
                                TokenType::Underscore => "_".to_string(),  // Wildcard
                                _ => return Err("Expected field name in variant pattern".to_string()),
                            };
                            fields.push(field);
                            
                            if !self.matches(TokenType::Comma) {
                                break;
                            }
                        }
                    }
                    
                    self.expect(TokenType::RParen, "expected ')' after variant pattern fields")?;
                    Ok(MatchPattern::Variant { name: full_name, fields })
                } else {
                    Ok(MatchPattern::Ident(full_name))
                }
            }
            _ => Err("Expected match pattern".to_string()),
        }
    }
    
    fn return_statement(&mut self) -> Result<Stmt, String> {
        let value = if self.check(TokenType::Semi) || self.check(TokenType::RBrace) {
            None
        } else {
            Some(self.expression()?)
        };
        
        self.expect(TokenType::Semi, "end return statement")?;
        Ok(Stmt::Return(value))
    }
    
    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        
        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        
        Ok(statements)
    }
    
    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }
    
    fn assignment(&mut self) -> Result<Expr, String> {
        // Handled in statement parsing
        self.or_expr()
    }
    
    fn or_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.and_expr()?;
        
        while self.matches(TokenType::Or) {
            let op = BinaryOp::Or;
            let right = self.and_expr()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        
        Ok(expr)
    }
    
    fn and_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;
        
        while self.matches(TokenType::And) {
            let op = BinaryOp::And;
            let right = self.equality()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        
        Ok(expr)
    }
    
    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        
        loop {
            let op = if self.matches(TokenType::EqEq) {
                BinaryOp::Eq
            } else if self.matches(TokenType::NotEq) {
                BinaryOp::NotEq
            } else {
                break;
            };
            
            let right = self.comparison()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        
        Ok(expr)
    }
    
    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        
        loop {
            let op = if self.matches(TokenType::Lt) {
                BinaryOp::Lt
            } else if self.matches(TokenType::LtEq) {
                BinaryOp::LtEq
            } else if self.matches(TokenType::Gt) {
                BinaryOp::Gt
            } else if self.matches(TokenType::GtEq) {
                BinaryOp::GtEq
            } else {
                break;
            };
            
            let right = self.term()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        
        Ok(expr)
    }
    
    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        
        while self.matches(TokenType::Plus) || self.matches(TokenType::Minus) {
            let op = if self.previous().token_type == TokenType::Plus {
                BinaryOp::Add
            } else {
                BinaryOp::Sub
            };
            
            let right = self.factor()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        
        Ok(expr)
    }
    
    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        
        while self.matches(TokenType::Star) || self.matches(TokenType::Slash) || self.matches(TokenType::Percent) {
            let op = if self.previous().token_type == TokenType::Star {
                BinaryOp::Mul
            } else if self.previous().token_type == TokenType::Slash {
                BinaryOp::Div
            } else {
                BinaryOp::Mod
            };
            
            let right = self.unary()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        
        Ok(expr)
    }
    
    fn unary(&mut self) -> Result<Expr, String> {
        if self.matches(TokenType::Not) {
            let expr = self.unary()?;
            return Ok(Expr::Unary { op: UnaryOp::Not, expr: Box::new(expr) });
        }
        
        if self.matches(TokenType::Minus) {
            let expr = self.unary()?;
            return Ok(Expr::Unary { op: UnaryOp::Neg, expr: Box::new(expr) });
        }
        
        self.call()
    }
    
    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        loop {
            if self.matches(TokenType::LParen) {
                let args = self.parse_arguments()?;
                self.expect(TokenType::RParen, "expected ')' after arguments")?;
                expr = Expr::Call { callee: Box::new(expr), args };
            } else if self.matches(TokenType::Dot) {
                let name_token = self.advance();
                let name = match &name_token.token_type {
                    TokenType::Ident(s) => s.clone(),
                    _ => return Err("Expected method/property name".to_string()),
                };

                if self.matches(TokenType::LParen) {
                    let args = self.parse_arguments()?;
                    self.expect(TokenType::RParen, "expected ')' after arguments")?;
                    expr = Expr::MethodCall { object: Box::new(expr), method: name, args };
                } else {
                    expr = Expr::PropertyAccess { object: Box::new(expr), property: name };
                }
            } else if self.matches(TokenType::LBracket) {
                let index = self.expression()?;
                self.expect(TokenType::RBracket, "expected ']' after index")?;
                expr = Expr::Index { object: Box::new(expr), index: Box::new(index) };
            } else {
                break;
            }
        }

        Ok(expr)
    }
    
    fn parse_arguments(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();

        if !self.check(TokenType::RParen) {
            loop {
                // Check for lambda
                if self.check(TokenType::Fn) {
                    self.advance();  // consume 'fn'
                    args.push(self.lambda_expr()?);
                } else {
                    args.push(self.expression()?);
                }

                if !self.matches(TokenType::Comma) {
                    break;
                }
            }
        }

        Ok(args)
    }
    
    fn primary(&mut self) -> Result<Expr, String> {
        if self.matches(TokenType::True) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }
        if self.matches(TokenType::False) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }
        
        if self.matches(TokenType::Self_) {
            return Ok(Expr::Self_);
        }
        
        if let TokenType::Int(i) = self.peek().token_type.clone() {
            self.advance();
            return Ok(Expr::Literal(Literal::Int(i)));
        }
        
        if let TokenType::Float(f) = self.peek().token_type.clone() {
            self.advance();
            return Ok(Expr::Literal(Literal::Float(f)));
        }
        
        if let TokenType::Str(s) = self.peek().token_type.clone() {
            self.advance();
            return Ok(Expr::Literal(Literal::Str(s)));
        }

        if let TokenType::InterpolatedString(segments) = self.peek().token_type.clone() {
            self.advance();
            // Convert lexer segments to AST parts
            let mut parts = Vec::new();
            for seg in segments {
                match seg {
                    StringInterpSegment::Text(text) => {
                        parts.push(StringInterpPart::Text(text));
                    }
                    StringInterpSegment::Expr(expr_str) => {
                        // Parse the expression string
                        // This is a simplified approach - in a real implementation,
                        // we'd need to properly track positions
                        let mut inner_lexer = Lexer::new(&expr_str);
                        let tokens = inner_lexer.tokenize().map_err(|e| format!("Interpolation error: {:?}", e))?;
                        let mut inner_parser = Parser::new(tokens, Rc::clone(&self.source));
                        let expr = inner_parser.expression().map_err(|e| format!("Interpolation parse error: {}", e))?;
                        parts.push(StringInterpPart::Expr(expr));
                    }
                }
            }
            return Ok(Expr::InterpolatedString { parts });
        }
        
        if self.matches(TokenType::Fn) {
            return self.lambda_expr();
        }
        
        if self.matches(TokenType::LBracket) {
            let mut elements = Vec::new();

            if !self.check(TokenType::RBracket) {
                loop {
                    elements.push(self.expression()?);
                    if !self.matches(TokenType::Comma) {
                        break;
                    }
                }
            }

            self.expect(TokenType::RBracket, "expected ']' after list elements")?;
            return Ok(Expr::ListLiteral(elements));
        }

        if self.check(TokenType::LBrace) {
            return self.map_literal();
        }

        if self.matches(TokenType::Match) {
            return self.match_expr();
        }

        let token = self.advance();
        match &token.token_type {
            TokenType::Ident(name) => {
                let name_clone = name.clone();
                // Check if this is followed by LBrace - could be class literal
                if self.check(TokenType::LBrace) {
                    // Peek ahead to check if this looks like a class literal
                    // Class literal: starts with uppercase letter
                    if name_clone.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                        self.advance(); // consume LBrace

                        let mut fields = Vec::new();

                        while !self.check(TokenType::RBrace) {
                            let field_name_token = self.advance();
                            let field_name = match &field_name_token.token_type {
                                TokenType::Ident(s) => s.clone(),
                                _ => {
                                    return Err("Expected field name in class literal".to_string());
                                }
                            };

                            self.expect(TokenType::Colon, "expected ':' after field name")?;
                            let field_value = self.expression()?;
                            fields.push((field_name, field_value));

                            if !self.matches(TokenType::Comma) {
                                break;
                            }
                        }

                        self.expect(TokenType::RBrace, "expected '}' after class literal")?;
                        Ok(Expr::ClassLiteral { class_name: name_clone, fields })
                    } else {
                        // Not a class name, just an identifier
                        Ok(Expr::Ident(name_clone))
                    }
                } else {
                    Ok(Expr::Ident(name_clone))
                }
            }
            TokenType::LParen => {
                if !self.check(TokenType::RParen) {
                    let mut elements = Vec::new();
                    elements.push(self.expression()?);

                    if self.check(TokenType::Comma) {
                        while self.matches(TokenType::Comma) {
                            elements.push(self.expression()?);
                        }
                        self.expect(TokenType::RParen, "expected ')' after tuple")?;
                        Ok(Expr::TupleLiteral(elements))
                    } else {
                        self.expect(TokenType::RParen, "expected ')' after expression")?;
                        Ok(elements.remove(0))
                    }
                } else {
                    self.expect(TokenType::RParen, "expected ')' after expression")?;
                    Err("Empty parentheses are not valid".to_string())
                }
            }
            _ => Err(format!("Expected expression, got {:?}", token.token_type)),
        }
    }

    fn map_literal(&mut self) -> Result<Expr, String> {
        self.expect(TokenType::LBrace, "expected '{' at start of map literal")?;
        
        let mut entries = Vec::new();
        
        while !self.check(TokenType::RBrace) {
            let key = self.expression()?;
            self.expect(TokenType::Colon, "expected ':' after map key")?;
            let value = self.expression()?;
            entries.push((key, value));
            
            if !self.matches(TokenType::Comma) {
                break;
            }
        }
        
        self.expect(TokenType::RBrace, "expected '}' after map literal")?;
        Ok(Expr::MapLiteral(entries))
    }

    fn match_expr(&mut self) -> Result<Expr, String> {
        let expr = self.expression()?;

        self.expect(TokenType::LBrace, "expected '{' before match arms")?;

        let mut arms = Vec::new();

        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            let pattern = self.parse_match_pattern()?;
            self.expect(TokenType::FatArrow, "expected '=>' after match pattern")?;

            // Match arm body: expression (not statement)
            let body_expr = if self.check(TokenType::LBrace) {
                // Block expression
                self.advance();
                let block = self.parse_block()?;
                self.expect(TokenType::RBrace, "expected '}' after match arm")?;
                // Convert block to expression (last statement is the value)
                if block.is_empty() {
                    Expr::Literal(Literal::Void)
                } else {
                    // For simplicity, treat block as sequence of expressions
                    // In a real implementation, we'd need block expressions
                    Expr::Literal(Literal::Void)  // Placeholder
                }
            } else {
                self.expression()?
            };

            // Consume comma separator if present
            self.matches(TokenType::Comma);

            arms.push(MatchArm { 
                pattern, 
                body: vec![Stmt::ExprStmt(body_expr)] 
            });
        }

        self.expect(TokenType::RBrace, "expected '}' after match arms")?;

        Ok(Expr::Match { expr: Box::new(expr), arms })
    }

    fn lambda_expr(&mut self) -> Result<Expr, String> {
        // Note: caller (primary) already consumed 'fn' token via matches()
        self.expect(TokenType::LParen, "expected '(' after 'fn' in lambda")?;
        let params = self.parse_params()?;
        self.expect(TokenType::RParen, "expected ')' after lambda parameters")?;

        let return_type = if self.matches(TokenType::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect(TokenType::LBrace, "expected '{' before lambda body")?;
        let body = self.parse_block()?;
        self.expect(TokenType::RBrace, "expected '}' after lambda body")?;

        Ok(Expr::Lambda { params, return_type, body })
    }
}

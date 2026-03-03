mod lexer;
mod parser;
mod ast;
mod runtime;
mod interpreter;
mod error;
mod analyzer;

use std::fs;
use std::env;
use std::process;
use std::rc::Rc;

use error::{ErrorReport, CompileError, codes};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input.fl>", args[0]);
        process::exit(1);
    }

    // Read source file
    let source = match fs::read_to_string(&args[1]) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{}: Cannot read file '{}'", 
                if e.kind() == std::io::ErrorKind::NotFound { "error" } else { "error" },
                args[1]);
            eprintln!("  {}", e);
            process::exit(1);
        }
    };

    let source_rc = Rc::new(source.clone());

    println!("{} {} '{}'", 
        style::cyan("Checking"),
        style::blue("ForgeLang"),
        args[1]);
    println!();

    // Phase 1: Lexing
    let mut lexer = lexer::Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(errors) => {
            let report = ErrorReport {
                errors: errors.errors().to_vec(),
                warnings: errors.warnings().to_vec(),
                source: Some(Rc::clone(&source_rc)),
            };
            eprintln!("{}", report.display());
            process::exit(1);
        }
    };

    // Also check for lexer warnings
    let lexer_errors = lexer.into_errors();
    let mut all_errors = lexer_errors.errors().to_vec();
    let mut all_warnings = lexer_errors.warnings().to_vec();

    // Phase 2: Parsing
    let mut parser = parser::Parser::new(tokens, Rc::clone(&source_rc));
    let program = match parser.parse() {
        Ok(program) => program,
        Err(errors) => {
            // Collect parser errors
            for err in errors.errors() {
                all_errors.push(err.clone());
            }
            for warn in errors.warnings() {
                all_warnings.push(warn.clone());
            }
            
            let report = ErrorReport {
                errors: all_errors,
                warnings: all_warnings,
                source: Some(source_rc),
            };
            eprintln!("{}", report.display());
            process::exit(1);
        }
    };

    // Collect parser warnings
    let parser_errors = parser.into_errors();
    for err in parser_errors.errors() {
        all_errors.push(err.clone());
    }
    for warn in parser_errors.warnings() {
        all_warnings.push(warn.clone());
    }

    // Phase 3: Semantic Analysis
    match analyzer::analyze(&program, Rc::clone(&source_rc)) {
        Ok(()) => {}
        Err(report) => {
            for err in report.errors {
                all_errors.push(err);
            }
            for warn in report.warnings {
                all_warnings.push(warn);
            }
        }
    }

    // Report all errors and warnings
    if !all_errors.is_empty() || !all_warnings.is_empty() {
        let report = ErrorReport {
            errors: all_errors.clone(),
            warnings: all_warnings,
            source: Some(source_rc.clone()),
        };
        eprintln!("{}", report.display());
        
        if !all_errors.is_empty() {
            process::exit(1);
        }
    }

    // Phase 4: Interpretation (only if no errors)
    let mut interpreter = interpreter::Interpreter::new();
    match interpreter.interpret(&program) {
        Ok(_) => {
            // Call main function if it exists
            if let Ok(main_val) = interpreter.env.get("main") {
                if let runtime::Value::Function(main_fn) = main_val {
                    match interpreter.call_function(&main_fn, &[], None) {
                        Ok(_) => {}
                        Err(e) => {
                            if !e.starts_with("__RETURN") && !e.starts_with("RETURN") {
                                let report = ErrorReport {
                                    errors: vec![
                                        CompileError::runtime_error(codes::INVALID_OPERATION, e)
                                    ],
                                    warnings: vec![],
                                    source: Some(source_rc.clone()),
                                };
                                eprintln!("{}", report.display());
                                process::exit(1);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            if !e.starts_with("__RETURN") && !e.starts_with("RETURN") {
                let report = ErrorReport {
                    errors: vec![
                        CompileError::runtime_error(codes::INVALID_OPERATION, e)
                    ],
                    warnings: vec![],
                    source: Some(source_rc.clone()),
                };
                eprintln!("{}", report.display());
                process::exit(1);
            }
        }
    }
}

// Simple ANSI color helpers
mod style {
    pub fn cyan(s: &str) -> String {
        if supports_color() {
            format!("\x1b[36m{}\x1b[0m", s)
        } else {
            s.to_string()
        }
    }
    
    pub fn blue(s: &str) -> String {
        if supports_color() {
            format!("\x1b[34m{}\x1b[0m", s)
        } else {
            s.to_string()
        }
    }
    
    pub fn green(s: &str) -> String {
        if supports_color() {
            format!("\x1b[32m{}\x1b[0m", s)
        } else {
            s.to_string()
        }
    }
    
    pub fn yellow(s: &str) -> String {
        if supports_color() {
            format!("\x1b[33m{}\x1b[0m", s)
        } else {
            s.to_string()
        }
    }
    
    pub fn red(s: &str) -> String {
        if supports_color() {
            format!("\x1b[31m{}\x1b[0m", s)
        } else {
            s.to_string()
        }
    }
    
    pub fn bold(s: &str) -> String {
        if supports_color() {
            format!("\x1b[1m{}\x1b[0m", s)
        } else {
            s.to_string()
        }
    }
    
    fn supports_color() -> bool {
        std::env::var("NO_COLOR").is_err() && 
        (std::env::var("COLORTERM").is_ok() || 
         std::env::var("TERM").map_or(false, |t| t != "dumb"))
    }
}

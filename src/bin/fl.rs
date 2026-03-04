use std::fs;
use std::env;
use std::process;
use std::rc::Rc;

use forgelang::engine::{ErrorReport, CompileError, codes, Lexer, Parser, Interpreter, analyze};
use forgelang::engine::runtime::Value;
use forgelang::cli::style;

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
    let mut lexer = Lexer::new(&source);
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
    let mut parser = Parser::new(tokens, Rc::clone(&source_rc));
    let program = match parser.parse() {
        Ok(program) => program,
        Err(errors) => {
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
    match analyze(&program, Rc::clone(&source_rc)) {
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
    let mut interpreter = Interpreter::new();
    match interpreter.interpret(&program) {
        Ok(_) => {
            // Call main function if it exists
            if let Ok(main_val) = interpreter.env.get("main") {
                if let Value::Function(main_fn) = main_val {
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

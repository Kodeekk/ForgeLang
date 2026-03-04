use std::fs;
use std::env;
use std::process;
use std::rc::Rc;

use forgelang::engine::{ErrorReport, CompileError, codes, Lexer, Parser, Interpreter, analyze};
use forgelang::engine::runtime::Value;
use forgelang::cli::style;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for --check flag
    let check_only = args.iter().any(|a| a == "--check");

    // Find the input file (skip flags and args[0] which is the binary path)
    let input_file = args.iter()
        .skip(1)  // Skip args[0] (binary path)
        .find(|a| !a.starts_with('-'))
        .map(|s| s.as_str());

    let input_file = match input_file {
        Some(f) => f,
        None => {
            eprintln!("Usage: {} [--check] <input.fl>", args[0]);
            process::exit(1);
        }
    };

    // Read source file
    let source = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{}: Cannot read file '{}'",
                if e.kind() == std::io::ErrorKind::NotFound { "error" } else { "error" },
                input_file);
            eprintln!("  {}", e);
            process::exit(1);
        }
    };

    let source_rc = Rc::new(source.clone());

    if check_only {
        println!("{} {} '{}'",
            style::cyan("Checking"),
            style::blue("ForgeLang"),
            input_file);
    } else {
        println!("{} {} '{}'",
            style::cyan("Running"),
            style::blue("ForgeLang"),
            input_file);
    }
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

    // If check-only mode, stop here
    if check_only {
        if all_errors.is_empty() {
            println!(
                "{} {} in {}",
                style::green("Finished"),
                style::blue("check"),
                style::bold("0.0s")
            );
        }
        process::exit(0);
    }

    // Phase 4: Interpretation (only if no errors and not check-only)
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

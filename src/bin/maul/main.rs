use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::rc::Rc;

use clap::{Parser as ClapParser, Subcommand};
use serde::{Deserialize, Serialize};

use forgelang::engine::{ErrorReport, CompileError, codes, Lexer, Parser, Interpreter, analyze};
use forgelang::engine::runtime::Value;
use forgelang::cli::{style, setup_stdlib_path};

/// Maul - The ForgeLang package manager (NOT a build tool - ForgeLang is interpreted!)
#[derive(ClapParser)]
#[command(name = "maul")]
#[command(author, version, about = "ForgeLang Package Manager - runs, checks, and manages projects", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new ForgeLang project
    New {
        /// Project name
        name: String,
    },
    /// Initialize a ForgeLang project in the current directory
    Init,
    /// Run the project (interprets directly, no compilation)
    Run {
        /// Arguments to pass to the program
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Type and syntax check only
    Check,
    /// Clean build artifacts (target directory from previous runs)
    Clean,
}

/// maul.yaml manifest structure
#[derive(Debug, Serialize, Deserialize)]
struct Manifest {
    package: Package,
}

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    name: String,
    version: String,
    #[serde(default = "default_entry")]
    entry: String,
}

fn default_entry() -> String {
    "src/main.fl".to_string()
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => cmd_new(&name),
        Commands::Init => cmd_init(),
        Commands::Run { args } => cmd_run(args),
        Commands::Check => cmd_check(),
        Commands::Clean => cmd_clean(),
    }
}

// ============================================================================
// Command Implementations
// ============================================================================

fn cmd_new(name: &str) {
    let project_dir = PathBuf::from(name);

    if project_dir.exists() {
        eprintln!("{}: Directory '{}' already exists", style::red("error"), name);
        process::exit(1);
    }

    // Create project structures
    fs::create_dir_all(&project_dir.join("src")).expect("Failed to create src directory");

    // Write maul.yaml
    let manifest = format!(
        r#"package:
  name: "{}"
  version: "0.1.0"
  entry: "src/main.fl"
"#,
        name
    );
    fs::write(project_dir.join("maul.yaml"), manifest)
        .expect("Failed to write maul.yaml");

    // Write src/main.fl
    let main_fl = r#"// Welcome to ForgeLang!
// Run with: maul run

import [println] from std.io;

fn main() -> int {
    println("Hello, ForgeLang!");
    return 0;
}
"#;
    fs::write(project_dir.join("src/main.fl"), main_fl)
        .expect("Failed to write src/main.fl");

    // Write .gitignore
    let gitignore = r#"target/
*.flc
maul.yaml
"#;
    fs::write(project_dir.join(".gitignore"), gitignore)
        .expect("Failed to write .gitignore");

    println!(
        "{} {} project '{}'",
        style::green("Created"),
        style::cyan("ForgeLang"),
        style::bold(name)
    );
    println!();
    println!("To get started, run:");
    println!("  cd {}", name);
    println!("  maul run");
}

fn cmd_init() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let manifest_path = current_dir.join("maul.yaml");

    if manifest_path.exists() {
        eprintln!(
            "{}: maul.yaml already exists",
            style::red("error")
        );
        process::exit(1);
    }

    let dir_name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("my_project");

    let manifest = format!(
        r#"package:
  name: "{}"
  version: "0.1.0"
  entry: "src/main.fl"
"#,
        dir_name
    );
    fs::write(&manifest_path, manifest).expect("Failed to write maul.yaml");

    // Create src directory if it doesn't exist
    let src_dir = current_dir.join("src");
    if !src_dir.exists() {
        fs::create_dir_all(&src_dir).expect("Failed to create src directory");
    }

    // Create main.fl if it doesn't exist
    let main_fl_path = src_dir.join("main.fl");
    if !main_fl_path.exists() {
        let main_fl = r#"// Welcome to ForgeLang!
// Run with: maul run

fn main() -> int {
    builtin_println("Hello, ForgeLang!");
    return 0;
}
"#;
        fs::write(&main_fl_path, main_fl).expect("Failed to write src/main.fl");
    }

    println!(
        "{} {} in current directory",
        style::green("Initialized"),
        style::cyan("ForgeLang project")
    );
}

fn cmd_run(args: Vec<String>) {
    let manifest = load_manifest();
    let entry_path = PathBuf::from(&manifest.package.entry);

    if !entry_path.exists() {
        eprintln!(
            "{}: Entry file '{}' not found",
            style::red("error"),
            entry_path.display()
        );
        process::exit(1);
    }

    // Set FORGELANG_STDLIB_PATH to global stdlib location
    setup_stdlib_path();

    println!(
        "{} {} '{}'",
        style::cyan("Running"),
        style::blue(&manifest.package.name),
        entry_path.display()
    );
    println!();

    match run_file(&entry_path) {
        Ok(_) => {}
        Err(_) => {
            process::exit(1);
        }
    }

    // Suppress unused variable warning for now
    let _ = args;
}

fn cmd_check() {
    let manifest = load_manifest();
    let entry_path = PathBuf::from(&manifest.package.entry);

    if !entry_path.exists() {
        eprintln!(
            "{}: Entry file '{}' not found",
            style::red("error"),
            entry_path.display()
        );
        process::exit(1);
    }

    // Set FORGELANG_STDLIB_PATH to global stdlib location
    setup_stdlib_path();

    println!(
        "{} {} '{}'",
        style::cyan("Checking"),
        style::blue(&manifest.package.name),
        entry_path.display()
    );
    println!();

    match check_file(&entry_path) {
        Ok(_) => {
            println!(
                "{} {} in {}",
                style::green("Finished"),
                style::blue("check"),
                style::bold("0.0s")
            );
        }
        Err(_) => {
            process::exit(1);
        }
    }
}

fn cmd_clean() {
    let target_dir = PathBuf::from("target");

    if target_dir.exists() {
        fs::remove_dir_all(&target_dir).expect("Failed to remove target directory");
        println!(
            "{} {} directory",
            style::green("Removed"),
            style::bold("target")
        );
    } else {
        println!("{}: Nothing to clean", style::yellow("info"));
    }
}

// ============================================================================
// Compiler Integration
// ============================================================================

fn load_manifest() -> Manifest {
    let manifest_path = PathBuf::from("maul.yaml");

    if !manifest_path.exists() {
        eprintln!(
            "{}: maul.yaml not found. Run '{}' first.",
            style::red("error"),
            style::cyan("maul init")
        );
        process::exit(1);
    }

    let content = fs::read_to_string(&manifest_path)
        .expect("Failed to read maul.yaml");

    serde_yaml::from_str(&content).expect("Failed to parse maul.yaml")
}

fn compile_file(path: &Path) -> Result<(), ()> {
    let source = fs::read_to_string(path).expect("Failed to read source file");
    let source_rc = Rc::new(source.clone());

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
            return Err(());
        }
    };

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
            return Err(());
        }
    };

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

    if !all_errors.is_empty() {
        let report = ErrorReport {
            errors: all_errors,
            warnings: all_warnings,
            source: Some(source_rc),
        };
        eprintln!("{}", report.display());
        return Err(());
    }

    Ok(())
}

fn check_file(path: &Path) -> Result<(), ()> {
    compile_file(path)
}

fn run_file(path: &Path) -> Result<(), ()> {
    let source = fs::read_to_string(path).expect("Failed to read source file");
    let source_rc = Rc::new(source.clone());

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
            return Err(());
        }
    };

    let mut all_errors = Vec::new();
    let mut all_warnings = Vec::new();

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
                source: Some(Rc::clone(&source_rc)),
            };
            eprintln!("{}", report.display());
            return Err(());
        }
    };

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

    if !all_errors.is_empty() {
        let report = ErrorReport {
            errors: all_errors,
            warnings: all_warnings,
            source: Some(Rc::clone(&source_rc)),
        };
        eprintln!("{}", report.display());
        return Err(());
    }

    // Phase 4: Interpretation
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
                                    source: Some(source_rc),
                                };
                                eprintln!("{}", report.display());
                                return Err(());
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
                    source: Some(source_rc),
                };
                eprintln!("{}", report.display());
                return Err(());
            }
        }
    }

    Ok(())
}

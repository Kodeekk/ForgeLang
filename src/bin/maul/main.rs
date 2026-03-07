use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command};

use clap::{Parser as ClapParser, Subcommand};
use serde::{Deserialize, Serialize};

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
    let main_fl = r#"import [println] from std.io;

fn main() -> int {
    println("Hello, ForgeLang!");
    return 0;
}
"#;
    fs::write(project_dir.join("src/main.fl"), main_fl)
        .expect("Failed to write src/main.fl");

    // Write .gitignore
    let gitignore = r#"comp/
*.flb
"#;
    fs::write(project_dir.join(".gitignore"), gitignore)
        .expect("Failed to write .gitignore");

    println!(
        "{} '{}' project",
        style::green("Created"),
        style::bold(name)
    );
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
        let main_fl = r#"fn main() -> int {
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

    // Get the path to the fl binary (same directory as maul)
    let fl_path = get_fl_binary_path();
    
    // Run the file using fl
    let mut cmd = Command::new(&fl_path);
    cmd.arg(&entry_path);
    
    // Pass through any additional arguments
    for arg in &args {
        cmd.arg(arg);
    }

    let status = cmd.status().expect("Failed to run fl");
    
    if !status.success() {
        process::exit(status.code().unwrap_or(1));
    }
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

    // Get the path to the fl binary (same directory as maul)
    let fl_path = get_fl_binary_path();

    // Run fl with --check flag (syntax/type check only, no execution)
    let status = Command::new(&fl_path)
        .arg(&entry_path)
        .arg("--check")
        .status()
        .expect("Failed to run fl");

    if !status.success() {
        process::exit(status.code().unwrap_or(1));
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
// Helpers
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

/// Get the path to the fl binary
/// Looks in the same directory as the maul binary
fn get_fl_binary_path() -> PathBuf {
    // Try to get the directory where maul is located
    if let Ok(exe) = env::current_exe() {
        if let Some(parent) = exe.parent() {
            let fl_path = parent.join("fl");
            if fl_path.exists() {
                return fl_path;
            }
        }
    }
    
    // Fallback: assume fl is in PATH
    PathBuf::from("fl")
}

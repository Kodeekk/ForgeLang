// Shared utilities for ForgeLang binaries

use std::env;

/// ANSI styling helpers
pub mod style {
    use std::env;
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
        env::var("NO_COLOR").is_err() &&
        (env::var("COLORTERM").is_ok() ||
         env::var("TERM").map_or(false, |t| t != "dumb"))
    }
}

/// Setup FORGELANG_STDLIB_PATH to point to global stdlib location
/// Priority:
/// 1. FORGELANG_STDLIB_PATH env var (already set)
/// 2. ~/.forgelang/stdlib/
/// 3. <executable_dir>/stdlib/ (for development)
pub fn setup_stdlib_path() {
    // If already set, use it
    if env::var("FORGELANG_STDLIB_PATH").is_ok() {
        return;
    }

    // Try ~/.forgelang/stdlib/
    if let Some(home) = dirs_home() {
        let global_stdlib = home.join(".forgelang").join("stdlib");
        if global_stdlib.exists() {
            unsafe { env::set_var("FORGELANG_STDLIB_PATH", &global_stdlib); }
            return;
        }
    }

    // Try <executable_dir>/stdlib/
    if let Ok(exe) = env::current_exe() {
        if let Some(parent) = exe.parent() {
            let exe_stdlib = parent.join("stdlib");
            if exe_stdlib.exists() {
                unsafe { env::set_var("FORGELANG_STDLIB_PATH", &exe_stdlib); }
                return;
            }
        }
    }

    // Try current directory's stdlib (for development)
    if let Ok(current_dir) = env::current_dir() {
        let stdlib_path = current_dir.join("stdlib");
        if stdlib_path.exists() {
            unsafe { env::set_var("FORGELANG_STDLIB_PATH", &stdlib_path); }
        }
    }
}

fn dirs_home() -> Option<std::path::PathBuf> {
    env::var("HOME").ok().map(std::path::PathBuf::from)
}

//! ForgeLang Error System - Comprehensive error reporting with spans and hints

use std::fmt;
use std::rc::Rc;

/// Represents a location in source code
#[derive(Debug, Clone)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub end_line: usize,
    pub end_column: usize,
    pub source: Rc<String>,
}

impl Span {
    pub fn new(line: usize, column: usize, source: Rc<String>) -> Self {
        Span {
            line,
            column,
            end_line: line,
            end_column: column,
            source,
        }
    }

    pub fn with_end(mut self, end_line: usize, end_column: usize) -> Self {
        self.end_line = end_line;
        self.end_column = end_column;
        self
    }

    /// Get the source line for this span
    pub fn get_line(&self) -> Option<&str> {
        self.source.lines().nth(self.line.saturating_sub(1))
    }

    /// Get multiple lines for context
    pub fn get_context_lines(&self, context: usize) -> Vec<(usize, &str)> {
        let lines: Vec<&str> = self.source.lines().collect();
        let start = self.line.saturating_sub(context + 1);
        let end = std::cmp::min(self.line + context, lines.len());
        
        (start..end)
            .filter_map(|i| lines.get(i).map(|line| (i + 1, *line)))
            .collect()
    }
}

/// Categories of errors
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCategory {
    Lexical,
    Syntax,
    Semantic,
    Type,
    Runtime,
    Warning,
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCategory::Lexical => write!(f, "lexical"),
            ErrorCategory::Syntax => write!(f, "syntax"),
            ErrorCategory::Semantic => write!(f, "semantic"),
            ErrorCategory::Type => write!(f, "type"),
            ErrorCategory::Runtime => write!(f, "runtime"),
            ErrorCategory::Warning => write!(f, "warning"),
        }
    }
}

/// A compiler error with rich context
#[derive(Debug, Clone)]
pub struct CompileError {
    pub category: ErrorCategory,
    pub code: &'static str,
    pub message: String,
    pub span: Option<Span>,
    pub hint: Option<String>,
    pub related: Vec<RelatedError>,
}

impl CompileError {
    pub fn new(category: ErrorCategory, code: &'static str, message: impl Into<String>) -> Self {
        CompileError {
            category,
            code,
            message: message.into(),
            span: None,
            hint: None,
            related: Vec::new(),
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn with_related(mut self, related: RelatedError) -> Self {
        self.related.push(related);
        self
    }
}

/// A related error note (like "defined here")
#[derive(Debug, Clone)]
pub struct RelatedError {
    pub message: String,
    pub span: Span,
}

impl RelatedError {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        RelatedError {
            message: message.into(),
            span,
        }
    }
}

/// Collects all errors during compilation
#[derive(Debug, Default)]
pub struct ErrorCollector {
    errors: Vec<CompileError>,
    warnings: Vec<CompileError>,
    pub(crate) source: Option<Rc<String>>,
}

impl ErrorCollector {
    pub fn new() -> Self {
        ErrorCollector {
            errors: Vec::new(),
            warnings: Vec::new(),
            source: None,
        }
    }

    pub fn with_source(mut self, source: Rc<String>) -> Self {
        self.source = Some(source);
        self
    }

    pub fn set_source(&mut self, source: Rc<String>) {
        self.source = Some(source);
    }

    /// Add an error and return false (for use in boolean contexts)
    pub fn error(&mut self, error: CompileError) -> bool {
        self.errors.push(error);
        false
    }

    /// Add a warning
    pub fn warning(&mut self, warning: CompileError) {
        self.warnings.push(warning);
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Get all errors
    pub fn errors(&self) -> &[CompileError] {
        &self.errors
    }

    /// Get all warnings
    pub fn warnings(&self) -> &[CompileError] {
        &self.warnings
    }

    /// Get count of errors
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Get count of warnings
    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }

    /// Finish collection and return result
    pub fn finish(self) -> Result<(), ErrorReport> {
        if self.has_errors() {
            Err(ErrorReport {
                errors: self.errors,
                warnings: self.warnings,
                source: self.source,
            })
        } else {
            Ok(())
        }
    }
}

/// Final error report for display
#[derive(Debug)]
pub struct ErrorReport {
    pub errors: Vec<CompileError>,
    pub warnings: Vec<CompileError>,
    pub source: Option<Rc<String>>,
}

impl ErrorReport {
    pub fn display(&self) -> ErrorDisplay<'_> {
        ErrorDisplay { report: self }
    }
}

/// Helper for displaying errors with colors
pub struct ErrorDisplay<'a> {
    report: &'a ErrorReport,
}

impl<'a> fmt::Display for ErrorDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let use_colors = supports_color();
        
        // Display errors first
        for error in &self.report.errors {
            write!(f, "{}", FormattedError::new(error, use_colors))?;
        }

        // Display warnings
        for warning in &self.report.warnings {
            write!(f, "{}", FormattedError::new(warning, use_colors))?;
        }

        // Summary
        let error_count = self.report.errors.len();
        let warning_count = self.report.warnings.len();
        
        writeln!(f)?;
        
        let summary = if error_count > 0 && warning_count > 0 {
            format!("{} errors, {} warnings", error_count, warning_count)
        } else if error_count > 0 {
            format!("{} errors", error_count)
        } else if warning_count > 0 {
            format!("{} warnings", warning_count)
        } else {
            String::new()
        };

        if !summary.is_empty() {
            if use_colors {
                if error_count > 0 {
                    writeln!(f, "\x1b[1m\x1b[31merror\x1b[0m: aborting due to {}", summary)?;
                } else {
                    writeln!(f, "\x1b[1m\x1b[33mwarning\x1b[0m: {}", summary)?;
                }
            } else {
                if error_count > 0 {
                    writeln!(f, "error: aborting due to {}", summary)?;
                } else {
                    writeln!(f, "warning: {}", summary)?;
                }
            }
        }

        Ok(())
    }
}

struct FormattedError<'a> {
    error: &'a CompileError,
    use_colors: bool,
}

impl<'a> FormattedError<'a> {
    fn new(error: &'a CompileError, use_colors: bool) -> Self {
        FormattedError { error, use_colors }
    }
}

impl<'a> fmt::Display for FormattedError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let e = self.error;
        let c = self.use_colors;

        // Severity label
        let (severity_color, _severity_text) = match e.category {
            ErrorCategory::Lexical | ErrorCategory::Syntax | 
            ErrorCategory::Semantic | ErrorCategory::Type => {
                if c { ("\x1b[1m\x1b[31merror\x1b[0m", "error") } else { ("error", "error") }
            }
            ErrorCategory::Runtime => {
                if c { ("\x1b[1m\x1b[31merror\x1b[0m", "error") } else { ("error", "error") }
            }
            ErrorCategory::Warning => {
                if c { ("\x1b[1m\x1b[33mwarning\x1b[0m", "warning") } else { ("warning", "warning") }
            }
        };

        // Error code
        let code_str = if c {
            format!("\x1b[1m\x1b[38;5;214m[{}]\x1b[0m", e.code)
        } else {
            format!("[{}]", e.code)
        };

        writeln!(f, "{} {}: {}", severity_color, code_str, e.message)?;

        // Display source snippet if we have a span
        if let Some(span) = &e.span {
            writeln!(f)?;
            
            let lines = span.get_context_lines(1);
            let max_line_num_width = lines.iter()
                .map(|(num, _)| num.to_string().len())
                .max()
                .unwrap_or(1);

            for (line_num, line_content) in lines {
                // Line number
                if c {
                    write!(f, "\x1b[1m\x1b[38;5;245m{:>width$}\x1b[0m | ", 
                           line_num, width = max_line_num_width)?;
                } else {
                    write!(f, "{:>width$} | ", line_num, width = max_line_num_width)?;
                }

                // Line content
                writeln!(f, "{}", line_content)?;

                // Underline for the error line
                if line_num == span.line {
                    // Empty line with just the pipe
                    if c {
                        write!(f, "\x1b[1m\x1b[38;5;245m{:>width$}\x1b[0m | ", 
                               "", width = max_line_num_width)?;
                    } else {
                        write!(f, "{:>width$} | ", "", width = max_line_num_width)?;
                    }

                    // Calculate underline position
                    let underline_start = span.column.saturating_sub(1);
                    let underline_len = if span.end_column > span.column {
                        span.end_column - span.column
                    } else {
                        1.max(line_content.len().saturating_sub(underline_start))
                    };

                    // Spaces before underline
                    for _ in 0..underline_start {
                        write!(f, " ")?;
                    }

                    // The underline with caret
                    if c {
                        write!(f, "\x1b[1m\x1b[31m")?;
                    }
                    write!(f, "^")?;
                    for _ in 1..underline_len {
                        write!(f, "~")?;
                    }
                    if c {
                        write!(f, "\x1b[0m")?;
                    }
                    writeln!(f)?;
                }
            }
        }

        // Display hint
        if let Some(hint) = &e.hint {
            if c {
                writeln!(f, "\x1b[1m\x1b[38;5;214mhelp\x1b[0m: {}", hint)?;
            } else {
                writeln!(f, "help: {}", hint)?;
            }
        }

        // Display related errors
        for related in &e.related {
            if c {
                writeln!(f, "\x1b[1m\x1b[38;5;214mnote\x1b[0m: {}", related.message)?;
            } else {
                writeln!(f, "note: {}", related.message)?;
            }
            
            if let Some(related_line) = related.span.get_line() {
                let line_num = related.span.line;
                if c {
                    writeln!(f, "\x1b[1m\x1b[38;5;245m  --> \x1b[0m:{}:{}\n", line_num, related.span.column)?;
                    writeln!(f, "\x1b[1m\x1b[38;5;245m   | \x1b[0m{}\n", related_line)?;
                } else {
                    writeln!(f, "  --> :{}:{}\n", line_num, related.span.column)?;
                    writeln!(f, "   | {}\n", related_line)?;
                }
            }
        }

        writeln!(f)?;
        Ok(())
    }
}

/// Check if the terminal supports colors
fn supports_color() -> bool {
    // Simple check - could be enhanced with actual terminal detection
    std::env::var("NO_COLOR").is_err() && 
    (std::env::var("COLORTERM").is_ok() || 
     std::env::var("TERM").map_or(false, |t| t != "dumb"))
}

// ============ Convenience constructors ============

impl CompileError {
    // Lexical errors
    pub fn lexical_error(code: &'static str, message: impl Into<String>) -> Self {
        CompileError::new(ErrorCategory::Lexical, code, message)
    }

    // Syntax errors
    pub fn syntax_error(code: &'static str, message: impl Into<String>) -> Self {
        CompileError::new(ErrorCategory::Syntax, code, message)
    }

    // Semantic errors
    pub fn semantic_error(code: &'static str, message: impl Into<String>) -> Self {
        CompileError::new(ErrorCategory::Semantic, code, message)
    }

    // Type errors
    pub fn type_error(code: &'static str, message: impl Into<String>) -> Self {
        CompileError::new(ErrorCategory::Type, code, message)
    }

    // Runtime errors
    pub fn runtime_error(code: &'static str, message: impl Into<String>) -> Self {
        CompileError::new(ErrorCategory::Runtime, code, message)
    }

    // Warnings
    pub fn warning(code: &'static str, message: impl Into<String>) -> Self {
        CompileError::new(ErrorCategory::Warning, code, message)
    }
}

// ============ Common error codes ============

pub mod codes {
    // Lexical errors: L001-L099
    pub const INVALID_CHAR: &str = "L001";
    pub const UNTERMINATED_STRING: &str = "L002";
    pub const UNTERMINATED_COMMENT: &str = "L003";
    pub const INVALID_NUMBER: &str = "L004";

    // Syntax errors: S001-S099
    pub const UNEXPECTED_TOKEN: &str = "S001";
    pub const MISSING_SEMICOLON: &str = "S002";
    pub const MISSING_CLOSING: &str = "S003";
    pub const EXPECTED_IDENTIFIER: &str = "S004";
    pub const EXPECTED_EXPRESSION: &str = "S005";
    pub const INVALID_PATTERN: &str = "S006";

    // Semantic errors: M001-M099
    pub const UNDEFINED_VARIABLE: &str = "M001";
    pub const UNDEFINED_FUNCTION: &str = "M002";
    pub const UNDEFINED_CLASS: &str = "M003";
    pub const UNDEFINED_METHOD: &str = "M004";
    pub const DUPLICATE_DEFINITION: &str = "M005";
    pub const UNDEFINED_MODULE: &str = "M006";
    pub const UNDEFINED_MEMBER: &str = "M007";
    pub const SELF_OUTSIDE_CLASS: &str = "M008";
    pub const INVALID_IMPORT: &str = "M009";

    // Type errors: T001-T099
    pub const TYPE_MISMATCH: &str = "T001";
    pub const UNKNOWN_TYPE: &str = "T002";
    pub const MISSING_RETURN: &str = "T003";
    pub const INVALID_OPERATION: &str = "T004";
    pub const NOT_CALLABLE: &str = "T005";
    pub const WRONG_ARITY: &str = "T006";
    pub const NOT_INDEXABLE: &str = "T007";
    pub const NOT_ITERABLE: &str = "T008";

    // Warnings: W001-W099
    pub const UNUSED_VARIABLE: &str = "W001";
    pub const UNUSED_FUNCTION: &str = "W002";
    pub const SHADOWING: &str = "W003";
    pub const UNREACHABLE_CODE: &str = "W004";
    pub const MISSING_DOCS: &str = "W005";
}

//! ForgeLang Semantic Analyzer - Type checking and semantic validation

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::ast::*;
use crate::error::*;

/// Type information for type checking
#[derive(Debug, Clone, PartialEq)]
pub enum TypeInfo {
    Int,
    Float,
    Str,
    Bool,
    Void,
    List(Box<TypeInfo>),
    Class(String),
    Unknown,
}

impl std::fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeInfo::Int => write!(f, "int"),
            TypeInfo::Float => write!(f, "float"),
            TypeInfo::Str => write!(f, "str"),
            TypeInfo::Bool => write!(f, "bool"),
            TypeInfo::Void => write!(f, "void"),
            TypeInfo::List(t) => write!(f, "list<{}>", t),
            TypeInfo::Class(name) => write!(f, "{}", name),
            TypeInfo::Unknown => write!(f, "unknown"),
        }
    }
}

/// Scope tracker for semantic analysis
struct ScopeAnalyzer {
    errors: ErrorCollector,
    /// Stack of scopes, each scope contains defined variables with their types
    scopes: Vec<HashMap<String, TypeInfo>>,
    /// All defined functions at module level
    functions: HashSet<String>,
    /// All defined classes at module level
    classes: HashSet<String>,
    /// All defined interfaces at module level
    interfaces: HashSet<String>,
    /// Track used variables for potential unused warnings
    _used_vars: HashSet<String>,
    /// Current expected return type (for return statement validation)
    expected_return_type: Option<TypeInfo>,
}

impl ScopeAnalyzer {
    pub fn new(source: Rc<String>) -> Self {
        ScopeAnalyzer {
            errors: ErrorCollector::new().with_source(source),
            scopes: vec![HashMap::new()], // Start with global scope
            functions: HashSet::new(),
            classes: HashSet::new(),
            interfaces: HashSet::new(),
            _used_vars: HashSet::new(),
            expected_return_type: None,
        }
    }

    /// Enter a new scope
    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Exit current scope
    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Define a variable in current scope with its type
    fn define_var(&mut self, name: &str, ty: TypeInfo) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), ty);
        }
    }

    /// Get type of identifier from any scope
    fn get_type(&self, name: &str) -> Option<TypeInfo> {
        // Check all scopes from innermost to outermost
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        // Check module-level definitions
        if self.functions.contains(name) {
            return Some(TypeInfo::Class("Function".to_string()));
        }
        if self.classes.contains(name) {
            return Some(TypeInfo::Class(name.to_string()));
        }
        if self.interfaces.contains(name) {
            return Some(TypeInfo::Class("Interface".to_string()));
        }
        if self.is_builtin_type(name) {
            return Some(TypeInfo::Class(name.to_string()));
        }
        None
    }

    /// Check if identifier is defined in any scope
    fn is_defined(&self, name: &str) -> bool {
        self.get_type(name).is_some()
    }

    /// Mark variable as used
    fn use_var(&mut self, name: &str) {
        self._used_vars.insert(name.to_string());
    }

    fn is_builtin(&self, name: &str) -> bool {
        matches!(name, "print" | "println" | "math" | "list" | "fs" | "env" | "time" | "self")
    }

    fn is_builtin_type(&self, name: &str) -> bool {
        matches!(name, "int" | "float" | "str" | "bool" | "void" |
                       "Point" | "Rectangle" | "Counter" | "User" | "Cat")
    }

    pub fn analyze(mut self, program: &Program) -> Result<(), ErrorReport> {
        // First pass: collect all top-level definitions and check for duplicates
        for stmt in &program.statements {
            self.collect_definitions(stmt);
        }

        // Second pass: analyze all statements with proper scoping
        for stmt in &program.statements {
            self.analyze_stmt(stmt);
        }

        self.errors.finish()
    }

    fn collect_definitions(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::FnDecl { name, .. } => {
                if self.functions.contains(name) {
                    self.errors.error(
                        CompileError::semantic_error(codes::DUPLICATE_DEFINITION,
                            format!("Duplicate definition of function '{}'", name))
                    );
                }
                self.functions.insert(name.clone());
            }
            Stmt::ClassDecl { name, .. } => {
                if self.classes.contains(name) {
                    self.errors.error(
                        CompileError::semantic_error(codes::DUPLICATE_DEFINITION,
                            format!("Duplicate definition of class '{}'", name))
                    );
                }
                self.classes.insert(name.clone());
            }
            Stmt::InterfaceDecl { name, .. } => {
                if self.interfaces.contains(name) {
                    self.errors.error(
                        CompileError::semantic_error(codes::DUPLICATE_DEFINITION,
                            format!("Duplicate definition of interface '{}'", name))
                    );
                }
                self.interfaces.insert(name.clone());
            }
            _ => {}
        }
    }

    fn analyze_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDecl { name, var_type, initializer, location } => {
                if let Some(init) = initializer {
                    let init_type = self.infer_expr_type(init);
                    // Check type compatibility if type annotation exists
                    if let Some(ann_type) = var_type {
                        let expected = self.type_from_annotation(ann_type);
                        if !self.types_compatible(&init_type, &expected) {
                            let span = location.as_ref()
                                .map(|loc| Span::new(loc.line, 1, Rc::clone(&loc.source)))
                                .unwrap_or_else(|| Span::new(0, 0, self.errors.source.clone().unwrap()));
                            self.errors.error(
                                CompileError::type_error(codes::TYPE_MISMATCH,
                                    format!("Cannot assign value of type '{}' to variable of type '{}'", 
                                        init_type, expected))
                                    .with_span(span)
                                    .with_hint(format!("Consider changing the type to '{}' or use a compatible value", init_type))
                            );
                        }
                    }
                    self.define_var(name, init_type);
                } else {
                    // No initializer - use annotated type or Unknown
                    let ty = var_type
                        .as_ref()
                        .map(|t| self.type_from_annotation(t))
                        .unwrap_or(TypeInfo::Unknown);
                    self.define_var(name, ty);
                }
            }
            Stmt::ConstDecl { name, const_type, value, location } => {
                let val_type = self.infer_expr_type(value);
                if let Some(ann_type) = const_type {
                    let expected = self.type_from_annotation(ann_type);
                    if !self.types_compatible(&val_type, &expected) {
                        let span = location.as_ref()
                            .map(|loc| Span::new(loc.line, 1, Rc::clone(&loc.source)))
                            .unwrap_or_else(|| Span::new(0, 0, self.errors.source.clone().unwrap()));
                        self.errors.error(
                            CompileError::type_error(codes::TYPE_MISMATCH,
                                format!("Cannot assign value of type '{}' to const of type '{}'", 
                                    val_type, expected))
                                .with_span(span)
                        );
                    }
                }
                self.define_var(name, val_type);
            }
            Stmt::FnDecl { name: _, params, return_type, body, location: _ } => {
                self.push_scope();
                // Add parameters to scope
                for param in params {
                    self.define_var(&param.name, TypeInfo::Unknown);
                }
                // Set expected return type
                let old_expected = self.expected_return_type.take();
                self.expected_return_type = return_type.as_ref().map(|rt| self.type_from_annotation(rt));
                // Analyze function body
                for s in body {
                    self.analyze_stmt(s);
                }
                // Restore previous expected return type
                self.expected_return_type = old_expected;
                self.pop_scope();
            }
            Stmt::ClassDecl { name: _, fields: _, methods, implements, .. } => {
                // Validate implemented interfaces
                for iface in implements {
                    if !self.interfaces.contains(iface) && !self.is_builtin(iface) {
                        self.errors.error(
                            CompileError::semantic_error(codes::UNDEFINED_MEMBER,
                                format!("Interface '{}' is not defined", iface))
                        );
                    }
                }

                // Analyze methods
                for method in methods {
                    self.push_scope();
                    // Add 'self' for instance methods (methods that have 'self' as first param)
                    let is_instance_method = method.params.first()
                        .map(|p| p.name == "self")
                        .unwrap_or(false);
                    if is_instance_method {
                        self.define_var("self", TypeInfo::Unknown);
                    }
                    // Add parameters (skip 'self' if it's the first param)
                    for param in &method.params {
                        if param.name != "self" {
                            self.define_var(&param.name, TypeInfo::Unknown);
                        }
                    }
                    // Set expected return type for method
                    let old_expected = self.expected_return_type.take();
                    self.expected_return_type = method.return_type.as_ref().map(|rt| self.type_from_annotation(rt));
                    // Analyze method body
                    for s in &method.body {
                        self.analyze_stmt(s);
                    }
                    // Restore previous expected return type
                    self.expected_return_type = old_expected;
                    self.pop_scope();
                }
            }
            Stmt::InterfaceDecl { .. } => {
                // Interfaces are just declarations
            }
            Stmt::ImplementDecl { interface_name, class_name: _, methods, .. } => {
                // Validate interface exists
                if !self.interfaces.contains(interface_name) && !self.is_builtin(interface_name) {
                    self.errors.error(
                        CompileError::semantic_error(codes::UNDEFINED_MEMBER,
                            format!("Interface '{}' is not defined", interface_name))
                    );
                }

                // Analyze methods
                for method in methods {
                    self.push_scope();
                    let is_instance_method = method.params.first()
                        .map(|p| p.name == "self")
                        .unwrap_or(false);
                    if is_instance_method {
                        self.define_var("self", TypeInfo::Unknown);
                    }
                    for param in &method.params {
                        if param.name != "self" {
                            self.define_var(&param.name, TypeInfo::Unknown);
                        }
                    }
                    // Set expected return type for method
                    let old_expected = self.expected_return_type.take();
                    self.expected_return_type = method.return_type.as_ref().map(|rt| self.type_from_annotation(rt));
                    for s in &method.body {
                        self.analyze_stmt(s);
                    }
                    // Restore previous expected return type
                    self.expected_return_type = old_expected;
                    self.pop_scope();
                }
            }
            Stmt::ExprStmt(expr) => {
                self.analyze_expr(expr);
            }
            Stmt::Return(expr) => {
                if let Some(e) = expr {
                    let return_type = self.infer_expr_type(e);
                    // Check if return type matches expected
                    if let Some(expected) = &self.expected_return_type {
                        if !self.types_compatible(&return_type, expected) {
                            self.errors.error(
                                CompileError::type_error(codes::TYPE_MISMATCH,
                                    format!("Cannot return value of type '{}' from function with return type '{}'", 
                                        return_type, expected))
                            );
                        }
                    }
                } else {
                    // Return without value - only valid for void functions
                    if let Some(expected) = &self.expected_return_type {
                        if expected != &TypeInfo::Void {
                            self.errors.error(
                                CompileError::type_error(codes::MISSING_RETURN,
                                    format!("Expected to return value of type '{}', found return without value", 
                                        expected))
                            );
                        }
                    }
                }
            }
            Stmt::If { condition, then_branch, else_if_branches, else_branch, .. } => {
                self.analyze_expr(condition);
                self.push_scope();
                for s in then_branch {
                    self.analyze_stmt(s);
                }
                self.pop_scope();
                
                for (cond, body) in else_if_branches {
                    self.analyze_expr(cond);
                    self.push_scope();
                    for s in body {
                        self.analyze_stmt(s);
                    }
                    self.pop_scope();
                }
                
                if let Some(else_body) = else_branch {
                    self.push_scope();
                    for s in else_body {
                        self.analyze_stmt(s);
                    }
                    self.pop_scope();
                }
            }
            Stmt::Match { expr, arms, .. } => {
                self.analyze_expr(expr);
                for arm in arms {
                    self.push_scope();
                    // Pattern binding (if identifier pattern)
                    if let MatchPattern::Ident(name) = &arm.pattern {
                        self.define_var(name, TypeInfo::Unknown);
                    }
                    for s in &arm.body {
                        self.analyze_stmt(s);
                    }
                    self.pop_scope();
                }
            }
            Stmt::For { var_name, iterable, body, .. } => {
                self.analyze_expr(iterable);
                self.push_scope();
                self.define_var(var_name, TypeInfo::Unknown);
                for s in body {
                    self.analyze_stmt(s);
                }
                self.pop_scope();
            }
            Stmt::While { condition, body, .. } => {
                self.analyze_expr(condition);
                self.push_scope();
                for s in body {
                    self.analyze_stmt(s);
                }
                self.pop_scope();
            }
            Stmt::Block(statements) => {
                self.push_scope();
                for s in statements {
                    self.analyze_stmt(s);
                }
                self.pop_scope();
            }
            Stmt::Assignment { name, value, .. } => {
                self.analyze_expr(value);
                // Check if variable is defined
                if !self.is_defined(name) {
                    self.errors.error(
                        CompileError::semantic_error(codes::UNDEFINED_VARIABLE,
                            format!("Use of undeclared variable '{}'", name))
                    );
                }
                self.use_var(name);
            }
            Stmt::AssignmentField { object, field: _, value, .. } => {
                self.analyze_expr(object);
                self.analyze_expr(value);
            }
            Stmt::Import { module, items, .. } => {
                // Validate module exists (basic check for std modules)
                if !module.starts_with("std.") {
                    self.errors.warning(
                        CompileError::warning(codes::INVALID_IMPORT,
                            format!("Unknown module '{}'. Only std.* modules are supported.", module))
                    );
                }
                
                // Define imported items
                if let Some(item_list) = items {
                    for item in item_list {
                        match item {
                            ImportItem::Simple(name) => {
                                self.define_var(name, TypeInfo::Unknown);
                            }
                            ImportItem::Aliased { alias, .. } => {
                                self.define_var(alias, TypeInfo::Unknown);
                            }
                        }
                    }
                } else if let Some(alias) = module.split('.').last() {
                    // import std.module or import std.module as alias
                    self.define_var(alias, TypeInfo::Unknown);
                }
            }
        }
    }

    fn analyze_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(_) => {}
            Expr::Ident(name) => {
                if !self.is_defined(name) {
                    self.errors.error(
                        CompileError::semantic_error(codes::UNDEFINED_VARIABLE,
                            format!("Use of undeclared variable '{}'", name))
                    );
                }
                self.use_var(name);
            }
            Expr::Binary { left, right, .. } => {
                self.analyze_expr(left);
                self.analyze_expr(right);
            }
            Expr::Unary { expr, .. } => {
                self.analyze_expr(expr);
            }
            Expr::Call { callee, args } => {
                self.analyze_expr(callee);
                for arg in args {
                    self.analyze_expr(arg);
                }
            }
            Expr::MethodCall { object, method: _, args } => {
                self.analyze_expr(object);
                for arg in args {
                    self.analyze_expr(arg);
                }
            }
            Expr::PropertyAccess { object, property: _ } => {
                self.analyze_expr(object);
            }
            Expr::Index { object, index } => {
                self.analyze_expr(object);
                self.analyze_expr(index);
            }
            Expr::Lambda { params, body, .. } => {
                self.push_scope();
                for param in params {
                    self.define_var(&param.name, TypeInfo::Unknown);
                }
                // Lambdas have their own return type context - clear expected return type
                let old_expected = self.expected_return_type.take();
                for s in body {
                    self.analyze_stmt(s);
                }
                self.expected_return_type = old_expected;
                self.pop_scope();
            }
            Expr::ListLiteral(elements) => {
                for elem in elements {
                    self.analyze_expr(elem);
                }
            }
            Expr::ClassLiteral { class_name, fields } => {
                // Validate class exists
                if !self.classes.contains(class_name) && !self.is_builtin(class_name) {
                    self.errors.error(
                        CompileError::semantic_error(codes::UNDEFINED_CLASS,
                            format!("Class '{}' is not defined", class_name))
                    );
                }
                for (_, field_expr) in fields {
                    self.analyze_expr(field_expr);
                }
            }
            Expr::InterpolatedString { parts } => {
                for part in parts {
                    if let StringInterpPart::Expr(e) = part {
                        self.analyze_expr(e);
                    }
                }
            }
            Expr::Self_ => {
                if !self.is_defined("self") {
                    self.errors.error(
                        CompileError::semantic_error(codes::SELF_OUTSIDE_CLASS,
                            "Cannot use 'self' outside of class context")
                    );
                }
            }
        }
    }

    /// Infer the type of an expression
    fn infer_expr_type(&mut self, expr: &Expr) -> TypeInfo {
        // Also analyze the expression for semantic errors
        self.analyze_expr(expr);
        
        match expr {
            Expr::Literal(lit) => self.type_from_literal(lit),
            Expr::Ident(name) => {
                self.get_type(name).unwrap_or(TypeInfo::Unknown)
            }
            Expr::Binary { left, op, right } => {
                let left_type = self.infer_expr_type(left);
                let right_type = self.infer_expr_type(right);
                
                // Check type compatibility for binary operations
                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                        if !self.types_compatible(&left_type, &right_type) {
                            self.errors.error(
                                CompileError::type_error(codes::INVALID_OPERATION,
                                    format!("Cannot apply operator to types '{}' and '{}'",
                                        left_type, right_type))
                            );
                        }
                    }
                    BinaryOp::Eq | BinaryOp::NotEq | BinaryOp::Lt | BinaryOp::LtEq |
                    BinaryOp::Gt | BinaryOp::GtEq => {
                        if !self.types_compatible(&left_type, &right_type) {
                            self.errors.error(
                                CompileError::type_error(codes::TYPE_MISMATCH,
                                    format!("Cannot compare types '{}' and '{}'",
                                        left_type, right_type))
                            );
                        }
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        if left_type != TypeInfo::Bool || right_type != TypeInfo::Bool {
                            self.errors.error(
                                CompileError::type_error(codes::TYPE_MISMATCH,
                                    format!("Logical operators require bool operands, got '{}' and '{}'",
                                        left_type, right_type))
                            );
                        }
                    }
                }
                
                // Return type based on operator
                match op {
                    BinaryOp::Eq | BinaryOp::NotEq | BinaryOp::Lt | BinaryOp::LtEq | 
                    BinaryOp::Gt | BinaryOp::GtEq | BinaryOp::And | BinaryOp::Or => {
                        TypeInfo::Bool
                    }
                    BinaryOp::Add if left_type == TypeInfo::Str || right_type == TypeInfo::Str => {
                        TypeInfo::Str
                    }
                    _ => {
                        if left_type == TypeInfo::Float || right_type == TypeInfo::Float {
                            TypeInfo::Float
                        } else {
                            TypeInfo::Int
                        }
                    }
                }
            }
            Expr::Unary { op, expr } => {
                let inner_type = self.infer_expr_type(expr);
                match op {
                    UnaryOp::Neg => {
                        if inner_type != TypeInfo::Int && inner_type != TypeInfo::Float {
                            self.errors.error(
                                CompileError::type_error(codes::INVALID_OPERATION,
                                    format!("Cannot negate type '{}'", inner_type))
                            );
                        }
                        inner_type
                    }
                    UnaryOp::Not => {
                        if inner_type != TypeInfo::Bool {
                            self.errors.error(
                                CompileError::type_error(codes::TYPE_MISMATCH,
                                    format!("Cannot apply 'not' to type '{}'", inner_type))
                            );
                        }
                        TypeInfo::Bool
                    }
                }
            }
            Expr::Call { callee, .. } => {
                self.infer_expr_type(callee);
                TypeInfo::Unknown // Could be improved with function signature lookup
            }
            Expr::MethodCall { .. } => {
                TypeInfo::Unknown // Could be improved with method signature lookup
            }
            Expr::PropertyAccess { .. } => {
                TypeInfo::Unknown // Could be improved with class field lookup
            }
            Expr::Index { object, .. } => {
                let obj_type = self.infer_expr_type(object);
                match obj_type {
                    TypeInfo::List(inner) => *inner,
                    TypeInfo::Str => TypeInfo::Str,
                    _ => TypeInfo::Unknown
                }
            }
            Expr::Lambda { .. } => {
                TypeInfo::Class("Function".to_string())
            }
            Expr::ListLiteral(elements) => {
                if elements.is_empty() {
                    TypeInfo::List(Box::new(TypeInfo::Unknown))
                } else {
                    let elem_type = self.infer_expr_type(&elements[0]);
                    TypeInfo::List(Box::new(elem_type))
                }
            }
            Expr::ClassLiteral { class_name, .. } => {
                TypeInfo::Class(class_name.clone())
            }
            Expr::InterpolatedString { .. } => {
                TypeInfo::Str
            }
            Expr::Self_ => {
                self.get_type("self").unwrap_or(TypeInfo::Unknown)
            }
        }
    }

    /// Get type from a literal
    fn type_from_literal(&self, lit: &Literal) -> TypeInfo {
        match lit {
            Literal::Int(_) => TypeInfo::Int,
            Literal::Float(_) => TypeInfo::Float,
            Literal::Str(_) => TypeInfo::Str,
            Literal::Bool(_) => TypeInfo::Bool,
            Literal::Void => TypeInfo::Void,
        }
    }

    /// Get type from a type annotation
    fn type_from_annotation(&self, ann: &TypeAnnotation) -> TypeInfo {
        match ann {
            TypeAnnotation::Int => TypeInfo::Int,
            TypeAnnotation::Float => TypeInfo::Float,
            TypeAnnotation::Str => TypeInfo::Str,
            TypeAnnotation::Bool => TypeInfo::Bool,
            TypeAnnotation::Void => TypeInfo::Void,
            TypeAnnotation::List(inner) => {
                TypeInfo::List(Box::new(self.type_from_annotation(inner)))
            }
            TypeAnnotation::Class(name) => TypeInfo::Class(name.clone()),
            TypeAnnotation::Fn(_, _) => TypeInfo::Class("Function".to_string()),
        }
    }

    /// Check if two types are compatible
    fn types_compatible(&self, a: &TypeInfo, b: &TypeInfo) -> bool {
        // Same types are always compatible
        if a == b {
            return true;
        }
        
        // Unknown is compatible with anything (for type inference)
        if a == &TypeInfo::Unknown || b == &TypeInfo::Unknown {
            return true;
        }
        
        // Int and Float are compatible (Int can be promoted to Float)
        if (a == &TypeInfo::Int && b == &TypeInfo::Float) ||
           (a == &TypeInfo::Float && b == &TypeInfo::Int) {
            return true;
        }
        
        // list<unknown> is compatible with any list type (for empty list literals)
        if let (TypeInfo::List(inner_a), TypeInfo::List(inner_b)) = (a, b) {
            if inner_a.as_ref() == &TypeInfo::Unknown || inner_b.as_ref() == &TypeInfo::Unknown {
                return true;
            }
        }
        
        false
    }
}

/// Run semantic analysis on a program
pub fn analyze(program: &Program, source: Rc<String>) -> Result<(), ErrorReport> {
    let analyzer = ScopeAnalyzer::new(source);
    analyzer.analyze(program)
}

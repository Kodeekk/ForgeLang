//! ForgeLang AST - Abstract Syntax Tree definitions

use std::fmt;
use std::rc::Rc;

/// Source code location for error reporting
#[derive(Debug, Clone)]
pub struct Location {
    pub line: usize,
    pub source: Rc<String>,
}

impl Location {
    pub fn new(line: usize, source: Rc<String>) -> Self {
        Location { line, source }
    }
    
    /// Get the source line for this location
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

#[derive(Debug, Clone)]
pub enum TypeAnnotation {
    Int,
    Float,
    Str,
    Bool,
    Void,
    List(Box<TypeAnnotation>),
    Class(String),
    Fn(Vec<TypeAnnotation>, Box<TypeAnnotation>),
    Tuple(Vec<TypeAnnotation>),
}

impl fmt::Display for TypeAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeAnnotation::Int => write!(f, "int"),
            TypeAnnotation::Float => write!(f, "float"),
            TypeAnnotation::Str => write!(f, "str"),
            TypeAnnotation::Bool => write!(f, "bool"),
            TypeAnnotation::Void => write!(f, "void"),
            TypeAnnotation::List(t) => write!(f, "list<{}>", t),
            TypeAnnotation::Class(name) => write!(f, "{}", name),
            TypeAnnotation::Fn(args, ret) => {
                let args_str: Vec<String> = args.iter().map(|a| format!("{}", a)).collect();
                write!(f, "fn({}) -> {}", args_str.join(", "), ret)
            }
            TypeAnnotation::Tuple(types) => {
                let types_str: Vec<String> = types.iter().map(|t| format!("{}", t)).collect();
                write!(f, "({})", types_str.join(", "))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Void,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Ident(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    PropertyAccess {
        object: Box<Expr>,
        property: String,
    },
    Lambda {
        params: Vec<Param>,
        return_type: Option<TypeAnnotation>,
        body: Vec<Stmt>,
    },
    ListLiteral(Vec<Expr>),
    ClassLiteral {
        class_name: String,
        fields: Vec<(String, Expr)>,
    },
    InterpolatedString {
        parts: Vec<StringInterpPart>,
    },
    Self_,
    TupleLiteral(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum StringInterpPart {
    Text(String),
    Expr(Expr),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::Mod => write!(f, "%"),
            BinaryOp::Eq => write!(f, "=="),
            BinaryOp::NotEq => write!(f, "!="),
            BinaryOp::Lt => write!(f, "<"),
            BinaryOp::LtEq => write!(f, "<="),
            BinaryOp::Gt => write!(f, ">"),
            BinaryOp::GtEq => write!(f, ">="),
            BinaryOp::And => write!(f, "&&"),
            BinaryOp::Or => write!(f, "||"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub param_type: Option<TypeAnnotation>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub field_type: Option<TypeAnnotation>,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Vec<Stmt>,
    pub is_static: bool,
}

#[derive(Debug, Clone)]
pub struct InterfaceMethod {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<TypeAnnotation>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl {
        pattern: Pattern,
        var_type: Option<TypeAnnotation>,
        initializer: Option<Expr>,
        location: Option<Location>,
    },
    ConstDecl {
        name: String,
        const_type: Option<TypeAnnotation>,
        value: Expr,
        location: Option<Location>,
    },
    FnDecl {
        name: String,
        params: Vec<Param>,
        return_type: Option<TypeAnnotation>,
        body: Vec<Stmt>,
        location: Option<Location>,
    },
    ClassDecl {
        name: String,
        implements: Vec<String>,
        fields: Vec<Field>,
        methods: Vec<Method>,
        location: Option<Location>,
    },
    InterfaceDecl {
        name: String,
        methods: Vec<InterfaceMethod>,
        location: Option<Location>,
    },
    ImplementDecl {
        interface_name: String,
        class_name: String,
        methods: Vec<Method>,
        location: Option<Location>,
    },
    ExprStmt(Expr),
    Return(Option<Expr>),
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_if_branches: Vec<(Expr, Vec<Stmt>)>,
        else_branch: Option<Vec<Stmt>>,
        location: Option<Location>,
    },
    Match {
        expr: Expr,
        arms: Vec<MatchArm>,
        location: Option<Location>,
    },
    For {
        pattern: Pattern,
        iterable: Expr,
        body: Vec<Stmt>,
        location: Option<Location>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
        location: Option<Location>,
    },
    Block(Vec<Stmt>),
    Assignment {
        name: String,
        value: Expr,
        location: Option<Location>,
    },
    AssignmentField {
        object: Box<Expr>,
        field: String,
        value: Expr,
        location: Option<Location>,
    },
    Import {
        module: String,
        alias: Option<String>,
        items: Option<Vec<ImportItem>>,
        location: Option<Location>,
    },
}

#[derive(Debug, Clone)]
pub enum ImportItem {
    Simple(String),
    Aliased { name: String, alias: String },
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: MatchPattern,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum MatchPattern {
    Literal(Literal),
    Ident(String),
    Underscore,
    Tuple(Vec<MatchPattern>),
}

/// Pattern for destructuring in variable declarations and for loops
#[derive(Debug, Clone)]
pub enum Pattern {
    Ident(String),
    Underscore,
    Tuple(Vec<Pattern>),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn new() -> Self {
        Program { statements: Vec::new() }
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

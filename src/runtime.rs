//! ForgeLang Runtime - Value types and environment

use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use crate::ast::*;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Void,
    List(Rc<RefCell<Vec<Value>>>),
    Object(Rc<Object>),
    Class(Rc<ClassDef>),
    Function(Rc<Function>),
    NativeFunction(NativeFn),
    Interface(Rc<InterfaceDef>),
    Module(String),
    ModuleEnv(Rc<Environment>),
}

impl Value {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => (a - b).abs() < f64::EPSILON,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Void, Value::Void) => true,
            (Value::List(a), Value::List(b)) => Rc::ptr_eq(a, b) || *a.borrow() == *b.borrow(),
            (Value::Object(a), Value::Object(b)) => Rc::ptr_eq(a, b),
            (Value::Class(a), Value::Class(b)) => Rc::ptr_eq(a, b),
            (Value::Function(a), Value::Function(b)) => Rc::ptr_eq(a, b),
            (Value::NativeFunction(_), Value::NativeFunction(_)) => false,
            (Value::Interface(a), Value::Interface(b)) => Rc::ptr_eq(a, b),
            (Value::Module(a), Value::Module(b)) => a == b,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Str(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Void => write!(f, "void"),
            Value::List(list) => {
                let items: Vec<String> = list.borrow().iter().map(|v| format!("{}", v)).collect();
                write!(f, "[{}]", items.join(", "))
            }
            Value::Object(obj) => write!(f, "{}", obj),
            Value::Class(cls) => write!(f, "class {}", cls.name),
            Value::Function(_) => write!(f, "fn"),
            Value::NativeFunction(_) => write!(f, "native fn"),
            Value::Interface(iface) => write!(f, "interface {}", iface.name),
            Value::Module(name) => write!(f, "module({})", name),
            Value::ModuleEnv(_) => write!(f, "module"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    pub class_name: String,
    pub fields: HashMap<String, Value>,
    pub methods: HashMap<String, Rc<Function>>,
    pub interface_impls: Vec<String>,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class_name)
    }
}

impl Object {
    pub fn new(class_name: String) -> Self {
        Object {
            class_name,
            fields: HashMap::new(),
            methods: HashMap::new(),
            interface_impls: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClassDef {
    pub name: String,
    pub fields: Vec<Field>,
    pub methods: HashMap<String, Rc<Function>>,
    pub static_methods: HashMap<String, Rc<Function>>,
    pub implements: Vec<String>,
}

impl ClassDef {
    pub fn new(name: String) -> Self {
        ClassDef {
            name,
            fields: Vec::new(),
            methods: HashMap::new(),
            static_methods: HashMap::new(),
            implements: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InterfaceDef {
    pub name: String,
    pub methods: Vec<InterfaceMethod>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Vec<Stmt>,
    pub closure_env: Option<Rc<Environment>>,
    pub is_method: bool,
}

impl Function {
    pub fn new(name: String, params: Vec<Param>, return_type: Option<TypeAnnotation>, 
               body: Vec<Stmt>, is_method: bool) -> Self {
        Function {
            name,
            params,
            return_type,
            body,
            closure_env: None,
            is_method,
        }
    }
}

pub type NativeFn = fn(&[Value]) -> Result<Value, String>;

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: Rc<RefCell<HashMap<String, Value>>>,
    pub enclosing: Option<Rc<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: Rc::new(RefCell::new(HashMap::new())),
            enclosing: None,
        }
    }

    pub fn with_enclosing(enclosing: Rc<Environment>) -> Self {
        Environment {
            values: Rc::new(RefCell::new(HashMap::new())),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.borrow_mut().insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<Value, String> {
        if let Some(value) = self.values.borrow().get(name) {
            return Ok(value.clone());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
        }

        Err(format!("Undefined variable '{}'", name))
    }

    pub fn assign(&self, name: &str, value: Value) -> Result<(), String> {
        if self.values.borrow().contains_key(name) {
            self.values.borrow_mut().insert(name.to_string(), value);
            return Ok(());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.assign(name, value);
        }

        Err(format!("Undefined variable '{}'", name))
    }

    pub fn assign_or_define(&mut self, name: &str, value: Value) {
        self.values.borrow_mut().insert(name.to_string(), value);
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

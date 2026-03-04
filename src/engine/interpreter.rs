//! ForgeLang Interpreter - Executes AST

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs;
use std::path::Path;
use crate::ast::*;
use crate::runtime::*;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub struct Interpreter {
    pub env: Rc<Environment>,
    pub classes: HashMap<String, Rc<ClassDef>>,
    pub interfaces: HashMap<String, Rc<InterfaceDef>>,
    pub interface_impls: HashMap<String, HashMap<String, Rc<Function>>>,
    return_value: Option<Value>,
    stdlib_path: String,
    loaded_modules: HashMap<String, Rc<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        // Determine stdlib path:
        // 1. Check FORGELANG_STDLIB_PATH env var (set by maul or installer)
        // 2. Check relative to current directory
        // 3. Check relative to executable
        // 4. Default to "stdlib"
        let stdlib_path = if let Ok(path) = std::env::var("FORGELANG_STDLIB_PATH") {
            path
        } else if Path::new("stdlib").exists() {
            "stdlib".to_string()
        } else if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                let candidate = parent.join("stdlib");
                if candidate.exists() {
                    candidate.to_string_lossy().to_string()
                } else {
                    "stdlib".to_string()
                }
            } else {
                "stdlib".to_string()
            }
        } else {
            "stdlib".to_string()
        };

        Interpreter {
            env: Rc::new(Environment::new()),
            classes: HashMap::new(),
            interfaces: HashMap::new(),
            interface_impls: HashMap::new(),
            return_value: None,
            stdlib_path,
            loaded_modules: HashMap::new(),
        }
    }
    
    pub fn interpret(&mut self, program: &Program) -> Result<Value, String> {
        self.setup_builtins();
        // Load primitive type classes from stdlib
        let _ = self.load_stdlib_module("str");
        let _ = self.load_stdlib_module("int");
        let _ = self.load_stdlib_module("float");
        let _ = self.load_stdlib_module("bool");
        let mut result = Value::Void;

        for stmt in &program.statements {
            result = self.execute(stmt)?;
        }

        Ok(result)
    }
    
    fn setup_builtins(&mut self) {
        let mut values = self.env.values.borrow_mut();

        // Core I/O builtins
        values.insert("builtin_print".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Ok(Value::Void);
            }
            print!("{}", args[0]);
            Ok(Value::Void)
        }));

        values.insert("builtin_println".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                println!();
                return Ok(Value::Void);
            }
            println!("{}", args[0]);
            Ok(Value::Void)
        }));

        values.insert("builtin_eprint".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Ok(Value::Void);
            }
            eprint!("{}", args[0]);
            Ok(Value::Void)
        }));

        values.insert("builtin_read_line".to_string(), Value::NativeFunction(|args| {
            use std::io::{self, Write};
            let _ = io::stdout().flush();
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => Ok(Value::Str(input.trim_end().to_string())),
                Err(_) => Ok(Value::Str("".to_string())),
            }
        }));

        values.insert("builtin_read_all".to_string(), Value::NativeFunction(|args| {
            use std::io::{self, Read};
            let mut input = String::new();
            match io::stdin().read_to_string(&mut input) {
                Ok(_) => Ok(Value::Str(input)),
                Err(_) => Ok(Value::Str("".to_string())),
            }
        }));

        values.insert("builtin_format".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Ok(Value::Str("".to_string()));
            }
            Ok(Value::Str(format!("{}", args[0])))
        }));

        // String builtins
        values.insert("builtin_str_length".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("length() requires 1 argument".to_string());
            }
            if let Value::Str(s) = &args[0] {
                Ok(Value::Int(s.chars().count() as i64))
            } else {
                Err("length() requires string argument".to_string())
            }
        }));

        values.insert("builtin_str_upper".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("upper() requires 1 argument".to_string());
            }
            if let Value::Str(s) = &args[0] {
                Ok(Value::Str(s.to_uppercase()))
            } else {
                Err("upper() requires string argument".to_string())
            }
        }));

        values.insert("builtin_str_lower".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("lower() requires 1 argument".to_string());
            }
            if let Value::Str(s) = &args[0] {
                Ok(Value::Str(s.to_lowercase()))
            } else {
                Err("lower() requires string argument".to_string())
            }
        }));

        values.insert("builtin_str_reverse".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("reverse() requires 1 argument".to_string());
            }
            if let Value::Str(s) = &args[0] {
                Ok(Value::Str(s.chars().rev().collect()))
            } else {
                Err("reverse() requires string argument".to_string())
            }
        }));

        values.insert("builtin_str_trim".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("trim() requires 1 argument".to_string());
            }
            if let Value::Str(s) = &args[0] {
                Ok(Value::Str(s.trim().to_string()))
            } else {
                Err("trim() requires string argument".to_string())
            }
        }));

        values.insert("builtin_str_contains".to_string(), Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err("contains() requires 2 arguments".to_string());
            }
            if let (Value::Str(s), Value::Str(substr)) = (&args[0], &args[1]) {
                Ok(Value::Bool(s.contains(substr)))
            } else {
                Err("contains() requires string arguments".to_string())
            }
        }));

        values.insert("builtin_str_split".to_string(), Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err("split() requires 2 arguments".to_string());
            }
            if let (Value::Str(s), Value::Str(delimiter)) = (&args[0], &args[1]) {
                let parts: Vec<Value> = s.split(delimiter)
                    .map(|p| Value::Str(p.to_string()))
                    .collect();
                Ok(Value::List(Rc::new(RefCell::new(parts))))
            } else {
                Err("split() requires string arguments".to_string())
            }
        }));

        // Module placeholders (will be loaded from stdlib)
        values.insert("math".to_string(), Value::Module("math".to_string()));
        values.insert("list".to_string(), Value::Module("list".to_string()));
        values.insert("fs".to_string(), Value::Module("fs".to_string()));
        values.insert("env".to_string(), Value::Module("env".to_string()));
        values.insert("time".to_string(), Value::Module("time".to_string()));

        // FS builtins
        values.insert("builtin_fs_read".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("read() requires 1 argument".to_string());
            }
            if let Value::Str(path) = &args[0] {
                match fs::read_to_string(path) {
                    Ok(content) => Ok(Value::Str(content)),
                    Err(e) => Err(format!("Failed to read file: {}", e)),
                }
            } else {
                Err("read() requires string argument".to_string())
            }
        }));

        values.insert("builtin_fs_write".to_string(), Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err("write() requires 2 arguments".to_string());
            }
            if let (Value::Str(path), Value::Str(content)) = (&args[0], &args[1]) {
                match fs::write(path, content) {
                    Ok(_) => Ok(Value::Void),
                    Err(e) => Err(format!("Failed to write file: {}", e)),
                }
            } else {
                Err("write() requires string arguments".to_string())
            }
        }));

        values.insert("builtin_fs_append".to_string(), Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err("append() requires 2 arguments".to_string());
            }
            if let (Value::Str(path), Value::Str(content)) = (&args[0], &args[1]) {
                use std::io::Write;
                match fs::OpenOptions::new().create(true).append(true).open(path) {
                    Ok(mut file) => {
                        match file.write_all(content.as_bytes()) {
                            Ok(_) => Ok(Value::Void),
                            Err(e) => Err(format!("Failed to append to file: {}", e)),
                        }
                    }
                    Err(e) => Err(format!("Failed to open file: {}", e)),
                }
            } else {
                Err("append() requires string arguments".to_string())
            }
        }));

        values.insert("builtin_fs_exists".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("exists() requires 1 argument".to_string());
            }
            if let Value::Str(path) = &args[0] {
                Ok(Value::Bool(Path::new(path).exists()))
            } else {
                Err("exists() requires string argument".to_string())
            }
        }));

        values.insert("builtin_fs_remove".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("remove() requires 1 argument".to_string());
            }
            if let Value::Str(path) = &args[0] {
                match fs::remove_file(path) {
                    Ok(_) => Ok(Value::Void),
                    Err(e) => Err(format!("Failed to remove file: {}", e)),
                }
            } else {
                Err("remove() requires string argument".to_string())
            }
        }));

        values.insert("builtin_fs_create_dir".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("create_dir() requires 1 argument".to_string());
            }
            if let Value::Str(path) = &args[0] {
                match fs::create_dir_all(path) {
                    Ok(_) => Ok(Value::Void),
                    Err(e) => Err(format!("Failed to create directory: {}", e)),
                }
            } else {
                Err("create_dir() requires string argument".to_string())
            }
        }));

        values.insert("builtin_fs_read_dir".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("read_dir() requires 1 argument".to_string());
            }
            if let Value::Str(path) = &args[0] {
                match fs::read_dir(path) {
                    Ok(entries) => {
                        let mut result = Vec::new();
                        for entry in entries.flatten() {
                            if let Some(name) = entry.file_name().to_str() {
                                result.push(Value::Str(name.to_string()));
                            }
                        }
                        Ok(Value::List(Rc::new(RefCell::new(result))))
                    }
                    Err(e) => Err(format!("Failed to read directory: {}", e)),
                }
            } else {
                Err("read_dir() requires string argument".to_string())
            }
        }));

        // Env builtins
        values.insert("builtin_env_cwd".to_string(), Value::NativeFunction(|_args| {
            match std::env::current_dir() {
                Ok(path) => Ok(Value::Str(path.to_string_lossy().to_string())),
                Err(_) => Err("Failed to get current directory".to_string()),
            }
        }));

        values.insert("builtin_env_home".to_string(), Value::NativeFunction(|_args| {
            match std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
                Ok(path) => Ok(Value::Str(path)),
                Err(_) => Ok(Value::Str("".to_string())),
            }
        }));

        values.insert("builtin_env_hostname".to_string(), Value::NativeFunction(|_args| {
            match std::env::var("HOSTNAME").or_else(|_| std::env::var("COMPUTERNAME")) {
                Ok(name) => Ok(Value::Str(name)),
                Err(_) => Ok(Value::Str("unknown".to_string())),
            }
        }));

        values.insert("builtin_env_os".to_string(), Value::NativeFunction(|_args| {
            Ok(Value::Str(std::env::consts::OS.to_string()))
        }));

        values.insert("builtin_env_get".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("get() requires 1 argument".to_string());
            }
            if let Value::Str(key) = &args[0] {
                match std::env::var(key) {
                    Ok(val) => Ok(Value::Str(val)),
                    Err(_) => Ok(Value::Void),
                }
            } else {
                Err("get() requires string argument".to_string())
            }
        }));

        values.insert("builtin_env_set".to_string(), Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err("set() requires 2 arguments".to_string());
            }
            if let (Value::Str(key), Value::Str(val)) = (&args[0], &args[1]) {
                unsafe { std::env::set_var(key, val); }
                Ok(Value::Void)
            } else {
                Err("set() requires string arguments".to_string())
            }
        }));

        values.insert("builtin_env_has".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("has() requires 1 argument".to_string());
            }
            if let Value::Str(key) = &args[0] {
                Ok(Value::Bool(std::env::var(key).is_ok()))
            } else {
                Err("has() requires string argument".to_string())
            }
        }));

        values.insert("builtin_env_remove".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("remove() requires 1 argument".to_string());
            }
            if let Value::Str(key) = &args[0] {
                unsafe { std::env::remove_var(key); }
                Ok(Value::Void)
            } else {
                Err("remove() requires string argument".to_string())
            }
        }));

        values.insert("builtin_env_vars".to_string(), Value::NativeFunction(|_args| {
            let mut result = Vec::new();
            for (key, val) in std::env::vars() {
                // Store as a simple string "key=value"
                result.push(Value::Str(format!("{}={}", key, val)));
            }
            Ok(Value::List(Rc::new(RefCell::new(result))))
        }));

        // Time builtins
        use std::time::{SystemTime, UNIX_EPOCH};
        values.insert("builtin_time_now".to_string(), Value::NativeFunction(|_args| {
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration) => Ok(Value::Int(duration.as_millis() as i64)),
                Err(_) => Ok(Value::Int(0)),
            }
        }));

        values.insert("builtin_time_sleep".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("sleep() requires 1 argument".to_string());
            }
            if let Value::Int(ms) = &args[0] {
                std::thread::sleep(std::time::Duration::from_millis(*ms as u64));
                Ok(Value::Void)
            } else {
                Err("sleep() requires integer argument".to_string())
            }
        }));

        // Math builtins
        values.insert("builtin_math_abs".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("abs() requires 1 argument".to_string());
            }
            match &args[0] {
                Value::Int(i) => Ok(Value::Int(i.abs())),
                Value::Float(f) => Ok(Value::Float(f.abs())),
                _ => Err("abs() requires numeric argument".to_string())
            }
        }));

        values.insert("builtin_math_min".to_string(), Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err("min() requires 2 arguments".to_string());
            }
            match (&args[0], &args[1]) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(std::cmp::min(*a, *b))),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.min(*b))),
                _ => Err("min() requires numeric arguments".to_string())
            }
        }));

        values.insert("builtin_math_max".to_string(), Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err("max() requires 2 arguments".to_string());
            }
            match (&args[0], &args[1]) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(std::cmp::max(*a, *b))),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.max(*b))),
                _ => Err("max() requires numeric arguments".to_string())
            }
        }));

        values.insert("builtin_math_pow".to_string(), Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err("pow() requires 2 arguments".to_string());
            }
            match (&args[0], &args[1]) {
                (Value::Int(base), Value::Int(exp)) => {
                    Ok(Value::Int(base.pow(*exp as u32)))
                }
                (Value::Float(base), Value::Float(exp)) => {
                    Ok(Value::Float(base.powf(*exp)))
                }
                _ => Err("pow() requires numeric arguments".to_string())
            }
        }));

        values.insert("builtin_math_sqrt".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("sqrt() requires 1 argument".to_string());
            }
            match &args[0] {
                Value::Int(i) => {
                    let result = (*i as f64).sqrt();
                    if result.fract() == 0.0 {
                        Ok(Value::Int(result as i64))
                    } else {
                        Ok(Value::Float(result))
                    }
                }
                Value::Float(f) => {
                    let result = f.sqrt();
                    if result.fract() == 0.0 {
                        Ok(Value::Int(result as i64))
                    } else {
                        Ok(Value::Float(result))
                    }
                }
                _ => Err("sqrt() requires numeric argument".to_string())
            }
        }));

        // Primitive type constructor builtins
        values.insert("builtin_str_ctor".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Ok(Value::Str("".to_string()));
            }
            Ok(Value::Str(format!("{}", args[0])))
        }));

        values.insert("builtin_str_find".to_string(), Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err("find() requires 2 arguments".to_string());
            }
            if let (Value::Str(s), Value::Str(substr)) = (&args[0], &args[1]) {
                match s.find(substr) {
                    Some(idx) => Ok(Value::Int(idx as i64)),
                    None => Ok(Value::Int(-1)),
                }
            } else {
                Err("find() requires string arguments".to_string())
            }
        }));

        values.insert("builtin_str_replace".to_string(), Value::NativeFunction(|args| {
            if args.len() < 3 {
                return Err("replace() requires 3 arguments".to_string());
            }
            if let (Value::Str(s), Value::Str(old), Value::Str(new)) = (&args[0], &args[1], &args[2]) {
                Ok(Value::Str(s.replace(old, new)))
            } else {
                Err("replace() requires string arguments".to_string())
            }
        }));

        values.insert("builtin_str_starts_with".to_string(), Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err("starts_with() requires 2 arguments".to_string());
            }
            if let (Value::Str(s), Value::Str(prefix)) = (&args[0], &args[1]) {
                Ok(Value::Bool(s.starts_with(prefix)))
            } else {
                Err("starts_with() requires string arguments".to_string())
            }
        }));

        values.insert("builtin_str_ends_with".to_string(), Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err("ends_with() requires 2 arguments".to_string());
            }
            if let (Value::Str(s), Value::Str(suffix)) = (&args[0], &args[1]) {
                Ok(Value::Bool(s.ends_with(suffix)))
            } else {
                Err("ends_with() requires string arguments".to_string())
            }
        }));

        values.insert("builtin_str_to_int".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("to_int() requires 1 argument".to_string());
            }
            if let Value::Str(s) = &args[0] {
                match s.parse::<i64>() {
                    Ok(n) => Ok(Value::Int(n)),
                    Err(_) => Err(format!("Cannot convert '{}' to int", s)),
                }
            } else {
                Err("to_int() requires string argument".to_string())
            }
        }));

        values.insert("builtin_str_to_float".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("to_float() requires 1 argument".to_string());
            }
            if let Value::Str(s) = &args[0] {
                match s.parse::<f64>() {
                    Ok(n) => Ok(Value::Float(n)),
                    Err(_) => Err(format!("Cannot convert '{}' to float", s)),
                }
            } else {
                Err("to_float() requires string argument".to_string())
            }
        }));

        values.insert("builtin_int".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Ok(Value::Int(0));
            }
            match &args[0] {
                Value::Int(i) => Ok(Value::Int(*i)),
                Value::Float(f) => Ok(Value::Int(*f as i64)),
                Value::Str(s) => {
                    match s.parse::<i64>() {
                        Ok(n) => Ok(Value::Int(n)),
                        Err(_) => Err(format!("Cannot convert '{}' to int", s)),
                    }
                }
                Value::Bool(b) => Ok(Value::Int(if *b { 1 } else { 0 })),
                _ => Err("Cannot convert to int".to_string())
            }
        }));

        values.insert("builtin_int_to_str".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("to_str() requires 1 argument".to_string());
            }
            match &args[0] {
                Value::Int(i) => Ok(Value::Str(format!("{}", i))),
                _ => Err("to_str() requires int argument".to_string())
            }
        }));

        values.insert("builtin_int_to_float".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("to_float() requires 1 argument".to_string());
            }
            match &args[0] {
                Value::Int(i) => Ok(Value::Float(*i as f64)),
                _ => Err("to_float() requires int argument".to_string())
            }
        }));

        values.insert("builtin_int_abs".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("abs() requires 1 argument".to_string());
            }
            match &args[0] {
                Value::Int(i) => Ok(Value::Int(i.abs())),
                _ => Err("abs() requires int argument".to_string())
            }
        }));

        values.insert("builtin_float".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Ok(Value::Float(0.0));
            }
            match &args[0] {
                Value::Int(i) => Ok(Value::Float(*i as f64)),
                Value::Float(f) => Ok(Value::Float(*f)),
                Value::Str(s) => {
                    match s.parse::<f64>() {
                        Ok(n) => Ok(Value::Float(n)),
                        Err(_) => Err(format!("Cannot convert '{}' to float", s)),
                    }
                }
                Value::Bool(b) => Ok(Value::Float(if *b { 1.0 } else { 0.0 })),
                _ => Err("Cannot convert to float".to_string())
            }
        }));

        values.insert("builtin_float_to_str".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("to_str() requires 1 argument".to_string());
            }
            match &args[0] {
                Value::Float(f) => Ok(Value::Str(format!("{}", f))),
                _ => Err("to_str() requires float argument".to_string())
            }
        }));

        values.insert("builtin_float_to_int".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("to_int() requires 1 argument".to_string());
            }
            match &args[0] {
                Value::Float(f) => Ok(Value::Int(*f as i64)),
                _ => Err("to_int() requires float argument".to_string())
            }
        }));

        values.insert("builtin_float_abs".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("abs() requires 1 argument".to_string());
            }
            match &args[0] {
                Value::Float(f) => Ok(Value::Float(f.abs())),
                _ => Err("abs() requires float argument".to_string())
            }
        }));

        values.insert("builtin_float_floor".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("floor() requires 1 argument".to_string());
            }
            match &args[0] {
                Value::Float(f) => Ok(Value::Float(f.floor())),
                _ => Err("floor() requires float argument".to_string())
            }
        }));

        values.insert("builtin_float_ceil".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("ceil() requires 1 argument".to_string());
            }
            match &args[0] {
                Value::Float(f) => Ok(Value::Float(f.ceil())),
                _ => Err("ceil() requires float argument".to_string())
            }
        }));

        values.insert("builtin_float_round".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("round() requires 1 argument".to_string());
            }
            match &args[0] {
                Value::Float(f) => Ok(Value::Float(f.round())),
                _ => Err("round() requires float argument".to_string())
            }
        }));

        values.insert("builtin_float_is_finite".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("is_finite() requires 1 argument".to_string());
            }
            match &args[0] {
                Value::Float(f) => Ok(Value::Bool(f.is_finite())),
                _ => Err("is_finite() requires float argument".to_string())
            }
        }));

        values.insert("builtin_float_is_nan".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err("is_nan() requires 1 argument".to_string());
            }
            match &args[0] {
                Value::Float(f) => Ok(Value::Bool(f.is_nan())),
                _ => Err("is_nan() requires float argument".to_string())
            }
        }));

        values.insert("builtin_bool".to_string(), Value::NativeFunction(|args| {
            if args.is_empty() {
                return Ok(Value::Bool(false));
            }
            match &args[0] {
                Value::Bool(b) => Ok(Value::Bool(*b)),
                Value::Int(i) => Ok(Value::Bool(*i != 0)),
                Value::Float(f) => Ok(Value::Bool(*f != 0.0)),
                Value::Str(s) => Ok(Value::Bool(!s.is_empty())),
                _ => Ok(Value::Bool(true))
            }
        }));
    }
    
    fn execute(&mut self, stmt: &Stmt) -> Result<Value, String> {
        match stmt {
            Stmt::Import { module, alias, items, .. } => {
                self.execute_import(module, alias.as_ref(), items.as_ref())?;
                Ok(Value::Void)
            }
            Stmt::ConstDecl { name, value, .. } => {
                let val = self.evaluate(value)?;
                self.define_variable(name.clone(), val);
                Ok(Value::Void)
            }
            Stmt::VarDecl { pattern, initializer, .. } => {
                let val = match initializer {
                    Some(expr) => self.evaluate(expr)?,
                    None => Value::Void,
                };
                self.bind_pattern(pattern, val)?;
                Ok(Value::Void)
            }
            Stmt::FnDecl { name, params, return_type, body, .. } => {
                let func = Rc::new(Function {
                    name: name.clone(),
                    params: params.clone(),
                    return_type: return_type.clone(),
                    body: body.clone(),
                    closure_env: Some(Rc::clone(&self.env)),
                    is_method: false,
                });
                self.define_variable(name.clone(), Value::Function(func));
                Ok(Value::Void)
            }
            Stmt::ClassDecl { name, implements, fields, methods, .. } => {
                let mut class_def = ClassDef::new(name.clone());
                class_def.fields = fields.clone();
                class_def.implements = implements.clone();

                for method in methods {
                    let func = Rc::new(Function {
                        name: method.name.clone(),
                        params: method.params.clone(),
                        return_type: method.return_type.clone(),
                        body: method.body.clone(),
                        closure_env: Some(Rc::clone(&self.env)),
                        is_method: true,
                    });

                    if method.name == "new" || method.name == "origin" ||
                       method.name == "create" || method.name == "from" {
                        class_def.static_methods.insert(method.name.clone(), func);
                    } else {
                        class_def.methods.insert(method.name.clone(), func);
                    }
                }

                let class_rc = Rc::new(class_def);
                
                // Register the class itself as a value (constructor)
                self.classes.insert(name.clone(), Rc::clone(&class_rc));
                self.define_variable(name.clone(), Value::Class(class_rc));
                Ok(Value::Void)
            }
            Stmt::InterfaceDecl { name, methods, .. } => {
                let iface = Rc::new(InterfaceDef {
                    name: name.clone(),
                    methods: methods.clone(),
                });
                self.interfaces.insert(name.clone(), iface);
                Ok(Value::Void)
            }
            Stmt::ImplementDecl { interface_name, class_name, methods, .. } => {
                let mut impl_methods = HashMap::new();

                for method in methods {
                    let func = Rc::new(Function {
                        name: method.name.clone(),
                        params: method.params.clone(),
                        return_type: method.return_type.clone(),
                        body: method.body.clone(),
                        closure_env: Some(Rc::clone(&self.env)),
                        is_method: true,
                    });
                    impl_methods.insert(method.name.clone(), func);
                }

                let key = format!("{}:{}", class_name, interface_name);
                self.interface_impls.insert(key, impl_methods);
                
                // Update the class to know it implements this interface
                if let Some(class_def) = self.classes.get(class_name) {
                    let mut updated = (**class_def).clone();
                    if !updated.implements.contains(interface_name) {
                        updated.implements.push(interface_name.clone());
                    }
                    self.classes.insert(class_name.clone(), Rc::new(updated));
                }
                
                Ok(Value::Void)
            }
            Stmt::ExprStmt(expr) => {
                self.evaluate(expr)?;
                Ok(Value::Void)
            }
            Stmt::Return(expr) => {
                let val = match expr {
                    Some(e) => self.evaluate(e)?,
                    None => Value::Void,
                };
                self.return_value = Some(val);
                return Err("RETURN".to_string());
            }
            Stmt::If { condition, then_branch, else_if_branches, else_branch, .. } => {
                let cond = self.evaluate(condition)?;
                
                if self.is_truthy(&cond) {
                    for s in then_branch {
                        self.execute(s)?;
                    }
                } else {
                    let mut executed = false;
                    
                    for (ei_cond, ei_body) in else_if_branches {
                        let ei_val = self.evaluate(ei_cond)?;
                        if self.is_truthy(&ei_val) {
                            for s in ei_body {
                                self.execute(s)?;
                            }
                            executed = true;
                            break;
                        }
                    }
                    
                    if !executed {
                        if let Some(else_body) = else_branch {
                            for s in else_body {
                                self.execute(s)?;
                            }
                        }
                    }
                }
                
                Ok(Value::Void)
            }
            Stmt::Match { expr, arms, .. } => {
                let match_value = self.evaluate(expr)?;

                for arm in arms {
                    let (matches, bindings) = self.match_pattern(&arm.pattern, &match_value);

                    if matches {
                        for (name, value) in bindings {
                            self.define_variable(name, value);
                        }
                        for s in &arm.body {
                            let _ = self.execute(s);
                        }
                        break;
                    }
                }

                Ok(Value::Void)
            }
            Stmt::For { pattern, iterable, body, .. } => {
                let iter_value = self.evaluate(iterable)?;

                match iter_value {
                    Value::List(list) => {
                        let list_clone = list.borrow().clone();
                        for item in list_clone.iter() {
                            self.bind_pattern(pattern, item.clone())?;

                            for s in body {
                                let _ = self.execute(s);
                            }
                        }
                    }
                    Value::Tuple(tuple_items) => {
                        for item in tuple_items.iter() {
                            self.bind_pattern(pattern, item.clone())?;

                            for s in body {
                                let _ = self.execute(s);
                            }
                        }
                    }
                    _ => return Err(format!("Cannot iterate over non-list value")),
                }

                Ok(Value::Void)
            }
            Stmt::While { condition, body, .. } => {
                loop {
                    let cond = self.evaluate(condition)?;
                    if !self.is_truthy(&cond) {
                        break;
                    }

                    for s in body {
                        self.execute(s)?;
                    }
                }

                Ok(Value::Void)
            }
            Stmt::Block(statements) => {
                let mut result = Value::Void;
                for s in statements {
                    result = self.execute(s)?;
                }
                Ok(result)
            }
            Stmt::Assignment { name, value, .. } => {
                let val = self.evaluate(value)?;
                self.assign_variable(name, val)?;
                Ok(Value::Void)
            }
            Stmt::AssignmentField { object, field, value, .. } => {
                let _obj_val = self.evaluate(object)?;
                let _val = self.evaluate(value)?;
                // Field assignment is tricky with Rc - skip for now
                Ok(Value::Void)
            }
        }
    }
    
    fn define_variable(&mut self, name: String, value: Value) {
        let mut env_values = self.env.values.borrow_mut();
        env_values.insert(name, value);
    }

    fn bind_pattern(&mut self, pattern: &Pattern, value: Value) -> Result<(), String> {
        match pattern {
            Pattern::Ident(name) => {
                self.define_variable(name.clone(), value);
            }
            Pattern::Underscore => {}
            Pattern::Tuple(patterns) => {
                match value {
                    Value::Tuple(values) => {
                        if patterns.len() != values.len() {
                            return Err(format!(
                                "Tuple pattern mismatch: expected {} elements, got {}",
                                patterns.len(),
                                values.len()
                            ));
                        }
                        for (i, pattern) in patterns.iter().enumerate() {
                            self.bind_pattern(pattern, values[i].clone())?;
                        }
                    }
                    _ => {
                        return Err(format!(
                            "Cannot destructure non-tuple value with tuple pattern"
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn match_pattern(&self, pattern: &MatchPattern, value: &Value) -> (bool, Vec<(String, Value)>) {
        match pattern {
            MatchPattern::Literal(lit) => {
                match lit {
                    Literal::Int(i) => {
                        if let Value::Int(v) = value {
                            (v == i, vec![])
                        } else {
                            (false, vec![])
                        }
                    }
                    Literal::Float(f) => {
                        if let Value::Float(v) = value {
                            (v == f, vec![])
                        } else {
                            (false, vec![])
                        }
                    }
                    Literal::Str(s) => {
                        if let Value::Str(v) = value {
                            (v == s, vec![])
                        } else {
                            (false, vec![])
                        }
                    }
                    Literal::Bool(b) => {
                        if let Value::Bool(v) = value {
                            (v == b, vec![])
                        } else {
                            (false, vec![])
                        }
                    }
                    Literal::Void => {
                        (matches!(value, Value::Void), vec![])
                    }
                }
            }
            MatchPattern::Ident(name) => {
                (true, vec![(name.clone(), value.clone())])
            }
            MatchPattern::Underscore => {
                (true, vec![])
            }
            MatchPattern::Tuple(patterns) => {
                if let Value::Tuple(values) = value {
                    if patterns.len() != values.len() {
                        return (false, vec![]);
                    }
                    let mut all_bindings = vec![];
                    for (i, pattern) in patterns.iter().enumerate() {
                        let (matches, bindings) = self.match_pattern(pattern, &values[i]);
                        if !matches {
                            return (false, vec![]);
                        }
                        all_bindings.extend(bindings);
                    }
                    (true, all_bindings)
                } else {
                    (false, vec![])
                }
            }
        }
    }

    fn assign_variable(&mut self, name: &str, value: Value) -> Result<(), String> {
        self.env.assign(name, value)
    }
    
    fn execute_import(&mut self, module: &str, alias: Option<&String>, items: Option<&Vec<ImportItem>>) -> Result<(), String> {
        if module.starts_with("std.") {
            let module_name = module.strip_prefix("std.").unwrap();
            
            // Load the module from stdlib
            let module_env = self.load_stdlib_module(module_name)?;
            
            if let Some(item_list) = items {
                // Import specific items from the module
                for item in item_list {
                    match item {
                        ImportItem::Simple(name) => {
                            let val = module_env.get(name)
                                .map_err(|_| format!("'{}' is not exported by module '{}'", name, module))?;
                            self.define_variable(name.clone(), val);
                        }
                        ImportItem::Aliased { name, alias: a } => {
                            let val = module_env.get(name)
                                .map_err(|_| format!("'{}' is not exported by module '{}'", name, module))?;
                            self.define_variable(a.clone(), val);
                        }
                    }
                }
            } else {
                // Import the module itself
                let name = alias.cloned().unwrap_or_else(|| module_name.to_string());
                
                // Check if we already have this module loaded
                if let Some(cached) = self.loaded_modules.get(module_name) {
                    self.define_variable(name, Value::ModuleEnv(Rc::clone(cached)));
                } else {
                    self.define_variable(name, Value::ModuleEnv(Rc::clone(&module_env)));
                }
            }
        }

        Ok(())
    }

    fn load_stdlib_module(&mut self, module_name: &str) -> Result<Rc<Environment>, String> {
        // Check if already loaded
        if let Some(cached) = self.loaded_modules.get(module_name) {
            return Ok(Rc::clone(cached));
        }

        // Construct path to module's package.fl
        let module_path = format!("{}/{}/package.fl", self.stdlib_path, module_name);

        // Read the source file
        let source = fs::read_to_string(&module_path)
            .map_err(|e| format!("Failed to load module '{}': {}\n  Expected file at: {}", module_name, e, module_path))?;

        let source_rc = Rc::new(source.clone());

        // Lex the source
        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize()
            .map_err(|e| format!("Lexer error in module '{}': {:?}", module_name, e))?;

        // Parse the source
        let mut parser = Parser::new(tokens, Rc::clone(&source_rc));
        let program = parser.parse()
            .map_err(|e| format!("Parser error in module '{}': {:?}", module_name, e))?;

        // Create a new environment for the module with enclosing pointing to builtins
        let module_env = Rc::new(Environment::with_enclosing(Rc::clone(&self.env)));

        // Execute the module in its own environment
        let old_env = std::mem::replace(&mut self.env, Rc::clone(&module_env));
        for stmt in &program.statements {
            let _ = self.execute(stmt);
        }
        self.env = old_env;

        // Cache and return the module environment
        self.loaded_modules.insert(module_name.to_string(), Rc::clone(&module_env));

        Ok(module_env)
    }
    
    fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(lit) => {
                let val = self.literal_to_value(lit)?;
                // Handle string interpolation for Str literals
                if let Value::Str(s) = &val {
                    return self.interpolate_string(s);
                }
                Ok(val)
            }
            Expr::Ident(name) => {
                self.env.get(name)
            }
            Expr::Binary { left, op, right } => {
                let left_val = self.evaluate(left)?;
                
                match op {
                    BinaryOp::And => {
                        if !self.is_truthy(&left_val) {
                            return Ok(Value::Bool(false));
                        }
                        let right_val = self.evaluate(right)?;
                        return Ok(Value::Bool(self.is_truthy(&right_val)));
                    }
                    BinaryOp::Or => {
                        if self.is_truthy(&left_val) {
                            return Ok(Value::Bool(true));
                        }
                        let right_val = self.evaluate(right)?;
                        return Ok(Value::Bool(self.is_truthy(&right_val)));
                    }
                    _ => {}
                }
                
                let right_val = self.evaluate(right)?;
                
                match op {
                    BinaryOp::Add => self.add(&left_val, &right_val),
                    BinaryOp::Sub => self.subtract(&left_val, &right_val),
                    BinaryOp::Mul => self.multiply(&left_val, &right_val),
                    BinaryOp::Div => self.divide(&left_val, &right_val),
                    BinaryOp::Mod => self.modulo(&left_val, &right_val),
                    BinaryOp::Eq => Ok(Value::Bool(left_val == right_val)),
                    BinaryOp::NotEq => Ok(Value::Bool(left_val != right_val)),
                    BinaryOp::Lt => self.less_than(&left_val, &right_val),
                    BinaryOp::LtEq => {
                        let lt = self.less_than(&left_val, &right_val)?;
                        let eq = left_val == right_val;
                        Ok(Value::Bool(lt.as_bool().unwrap() || eq))
                    }
                    BinaryOp::Gt => self.greater_than(&left_val, &right_val),
                    BinaryOp::GtEq => {
                        let gt = self.greater_than(&left_val, &right_val)?;
                        let eq = left_val == right_val;
                        Ok(Value::Bool(gt.as_bool().unwrap() || eq))
                    }
                    _ => Err(format!("Unknown binary operator: {:?}", op)),
                }
            }
            Expr::Unary { op, expr } => {
                let val = self.evaluate(expr)?;
                
                match op {
                    UnaryOp::Neg => {
                        match val {
                            Value::Int(i) => Ok(Value::Int(-i)),
                            Value::Float(f) => Ok(Value::Float(-f)),
                            _ => Err(format!("Cannot negate non-numeric value")),
                        }
                    }
                    UnaryOp::Not => Ok(Value::Bool(!self.is_truthy(&val))),
                }
            }
            Expr::Call { callee, args } => {
                let func_val = self.evaluate(callee)?;
                let arg_values: Vec<Value> = args.iter().map(|a| self.evaluate(a)).collect::<Result<_, _>>()?;

                match func_val {
                    Value::Function(func) => {
                        self.call_function(&func, &arg_values, None)
                    }
                    Value::NativeFunction(native) => {
                        native(&arg_values)
                    }
                    Value::Class(class_def) => {
                        // Create a new instance of the class
                        let mut methods = HashMap::new();
                        for (method_name, method_fn) in &class_def.methods {
                            methods.insert(method_name.clone(), Rc::clone(method_fn));
                        }
                        
                        let obj = Rc::new(Object {
                            class_name: class_def.name.clone(),
                            fields: HashMap::new(),
                            methods: methods,
                            interface_impls: class_def.implements.clone(),
                        });
                        
                        Ok(Value::Object(obj))
                    }
                    _ => Err(format!("Cannot call non-function value")),
                }
            }
            Expr::MethodCall { object, method, args } => {
                let obj_val = self.evaluate(object);
                let arg_values: Vec<Value> = args.iter().map(|a| self.evaluate(a)).collect::<Result<_, _>>()?;

                // If object evaluation failed, check if it's a class name
                let obj_val = if obj_val.is_err() {
                    if let Expr::Ident(class_name) = object.as_ref() {
                        if self.classes.contains_key(class_name) {
                            // It's a class static method call
                            if let Some(class_def) = self.classes.get(class_name).cloned() {
                                if let Some(func) = class_def.static_methods.get(method) {
                                    return self.call_function(func, &arg_values, None);
                                }
                            }
                            return Err(format!("Static method '{}' not found on class {}", method, class_name));
                        }
                    }
                    return obj_val;
                } else {
                    obj_val?
                };

                match obj_val {
                    Value::Object(obj_rc) => {
                        // First check object's own methods
                        if let Some(func) = obj_rc.methods.get(method).cloned() {
                            return self.call_function(&func, &arg_values, Some(Rc::clone(&obj_rc)));
                        }

                        // Check interface implementations
                        let key_format = format!("{}:{}", obj_rc.class_name, method);
                        for iface_name in &obj_rc.interface_impls {
                            let key = format!("{}:{}", obj_rc.class_name, iface_name);
                            if let Some(impl_methods) = self.interface_impls.get(&key).cloned() {
                                if let Some(func) = impl_methods.get(method).cloned() {
                                    return self.call_function(&func, &arg_values, Some(Rc::clone(&obj_rc)));
                                }
                            }
                        }

                        // Check class methods
                        if let Some(class_def) = self.classes.get(&obj_rc.class_name).cloned() {
                            if let Some(func) = class_def.methods.get(method).cloned() {
                                return self.call_function(&func, &arg_values, Some(Rc::clone(&obj_rc)));
                            }
                        }

                        Err(format!("Method '{}' not found on {}", method, obj_rc.class_name))
                    }
                    Value::Str(s) => {
                        // String primitive - call str class method
                        return self.call_primitive_method("str", method, &arg_values, Some(Value::Str(s)));
                    }
                    Value::Int(i) => {
                        // Int primitive - call int class method
                        return self.call_primitive_method("int", method, &arg_values, Some(Value::Int(i)));
                    }
                    Value::Float(f) => {
                        // Float primitive - call float class method
                        return self.call_primitive_method("float", method, &arg_values, Some(Value::Float(f)));
                    }
                    Value::Bool(b) => {
                        // Bool primitive - call bool class method
                        return self.call_primitive_method("bool", method, &arg_values, Some(Value::Bool(b)));
                    }
                    Value::List(list) => {
                        self.call_list_method(&list, method, &arg_values)
                    }
                    Value::Module(module_name) => {
                        self.call_module_method(&module_name, method, &arg_values)
                    }
                    Value::ModuleEnv(module_env) => {
                        // Call function from module environment
                        let func_val = module_env.get(method)
                            .map_err(|_| format!("Function '{}' not found in module", method))?;
                        match func_val {
                            Value::Function(func) => {
                                return self.call_function(&func, &arg_values, None);
                            }
                            Value::NativeFunction(native) => {
                                return native(&arg_values);
                            }
                            _ => Err(format!("'{}' is not a function in module", method)),
                        }
                    }
                    _ => Err(format!("Cannot call method '{}' on this value", method)),
                }
            }
            Expr::PropertyAccess { object, property } => {
                let obj_val = self.evaluate(object)?;
                
                match obj_val {
                    Value::Object(obj_rc) => {
                        if let Some(val) = obj_rc.fields.get(property) {
                            return Ok(val.clone());
                        }
                        Err(format!("Property '{}' not found on {}", property, obj_rc.class_name))
                    }
                    _ => Err(format!("Cannot access property on non-object")),
                }
            }
            Expr::Index { object, index } => {
                let obj_val = self.evaluate(object)?;
                let idx_val = self.evaluate(index)?;
                
                match obj_val {
                    Value::List(list) => {
                        match idx_val {
                            Value::Int(i) => {
                                let list_ref = list.borrow();
                                let idx = if i < 0 { (list_ref.len() as i64 + i) as usize } else { i as usize };
                                list_ref.get(idx).cloned().ok_or(format!("Index out of bounds: {}", i))
                            }
                            _ => Err(format!("List index must be an integer")),
                        }
                    }
                    Value::Str(s) => {
                        match idx_val {
                            Value::Int(i) => {
                                let chars: Vec<char> = s.chars().collect();
                                let idx = if i < 0 { (chars.len() as i64 + i) as usize } else { i as usize };
                                chars.get(idx).map(|c| Value::Str(c.to_string()))
                                    .ok_or(format!("Index out of bounds: {}", i))
                            }
                            _ => Err(format!("String index must be an integer")),
                        }
                    }
                    _ => Err(format!("Cannot index non-indexable value")),
                }
            }
            Expr::Lambda { params, return_type, body } => {
                let func = Rc::new(Function {
                    name: "<lambda>".to_string(),
                    params: params.clone(),
                    return_type: return_type.clone(),
                    body: body.clone(),
                    closure_env: Some(Rc::clone(&self.env)),
                    is_method: false,
                });
                Ok(Value::Function(func))
            }
            Expr::ListLiteral(elements) => {
                let values: Vec<Value> = elements.iter()
                    .map(|e| self.evaluate(e))
                    .collect::<Result<_, _>>()?;
                Ok(Value::List(Rc::new(RefCell::new(values))))
            }
            Expr::TupleLiteral(elements) => {
                let values: Vec<Value> = elements.iter()
                    .map(|e| self.evaluate(e))
                    .collect::<Result<_, _>>()?;
                Ok(Value::Tuple(values))
            }
            Expr::ClassLiteral { class_name, fields } => {
                if let Some(class_def) = self.classes.get(class_name).cloned() {
                    let mut obj = Object::new(class_name.clone());
                    obj.interface_impls = class_def.implements.clone();
                    
                    for (field_name, field_expr) in fields {
                        let val = self.evaluate(field_expr)?;
                        obj.fields.insert(field_name.clone(), val);
                    }
                    
                    for (name, func) in &class_def.methods {
                        obj.methods.insert(name.clone(), Rc::clone(func));
                    }
                    
                    Ok(Value::Object(Rc::new(obj)))
                } else {
                    Err(format!("Unknown class: {}", class_name))
                }
            }
            Expr::InterpolatedString { parts } => {
                let mut result = String::new();
                
                for part in parts {
                    match part {
                        StringInterpPart::Text(text) => {
                            result.push_str(text);
                        }
                        StringInterpPart::Expr(expr) => {
                            let val = self.evaluate(expr)?;
                            result.push_str(&format!("{}", val));
                        }
                    }
                }
                
                Ok(Value::Str(result))
            }
            Expr::Self_ => {
                self.env.get("__self__")
            }
        }
    }
    
    fn interpolate_string(&mut self, s: &str) -> Result<Value, String> {
        // Simple string interpolation: replace {expr} with evaluated expression
        // Supports: {variable}, {self.property}, {expr.method()}
        let mut result = String::new();
        let mut chars = s.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '{' {
                // Find matching }
                let mut expr_str = String::new();
                let mut depth = 1;
                
                while let Some(&inner_ch) = chars.peek() {
                    chars.next();
                    if inner_ch == '{' {
                        depth += 1;
                        expr_str.push(inner_ch);
                    } else if inner_ch == '}' {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                        expr_str.push(inner_ch);
                    } else {
                        expr_str.push(inner_ch);
                    }
                }
                
                // Parse and evaluate the expression
                let expr_val = self.evaluate_interpolation_expr(&expr_str)?;
                result.push_str(&format!("{}", expr_val));
            } else {
                result.push(ch);
            }
        }
        
        Ok(Value::Str(result))
    }
    
    fn parse_interpolation_args(&mut self, args_str: &str) -> Result<Vec<Value>, String> {
        if args_str.trim().is_empty() {
            return Ok(Vec::new());
        }

        let mut parsed_args = Vec::new();
        for arg in args_str.split(',') {
            let arg = arg.trim();
            // Try to parse as int
            if let Ok(i) = arg.parse::<i64>() {
                parsed_args.push(Value::Int(i));
            } else if let Ok(f) = arg.parse::<f64>() {
                parsed_args.push(Value::Float(f));
            } else if arg == "true" {
                parsed_args.push(Value::Bool(true));
            } else if arg == "false" {
                parsed_args.push(Value::Bool(false));
            } else if arg.starts_with('"') && arg.ends_with('"') {
                parsed_args.push(Value::Str(arg[1..arg.len()-1].to_string()));
            } else if arg.starts_with("self.") {
                // Handle self.property access
                let prop_name = &arg[5..];
                if let Ok(self_val) = self.env.get("__self__") {
                    if let Value::Object(obj) = self_val {
                        if let Some(val) = obj.fields.get(prop_name) {
                            parsed_args.push(val.clone());
                            continue;
                        }
                    }
                }
                return Err(format!("Property '{}' not found on self", prop_name));
            } else {
                // Try as variable
                if let Ok(val) = self.env.get(arg) {
                    parsed_args.push(val);
                }
            }
        }
        Ok(parsed_args)
    }
    
    /// Find the matching closing parenthesis, handling nesting
    fn find_matching_paren(s: &str, start: usize) -> Option<usize> {
        let mut depth = 1;
        let chars: Vec<char> = s.chars().collect();
        let mut i = start;
        while i < chars.len() {
            match chars[i] {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i);
                    }
                }
                _ => {}
            }
            i += 1;
        }
        None
    }

    fn evaluate_interpolation_expr(&mut self, expr_str: &str) -> Result<Value, String> {
        // Simple expression evaluator for interpolation
        // Supports: identifier, self.identifier, identifier.method(), identifier.property
        // Also supports chained calls like s.trim().upper()
        let expr_str = expr_str.trim();

        // Check for self.property or self.method()
        if expr_str.starts_with("self.") {
            let rest = &expr_str[5..];
            if let Some(dot_pos) = rest.find('(') {
                // Method call: self.method()
                let method_name = &rest[..dot_pos];
                // Get self
                let self_val = self.env.get("__self__")?;
                if let Value::Object(obj) = self_val {
                    // Call method on self
                    if let Some(func) = obj.methods.get(method_name) {
                        return self.call_function(func, &[], Some(Rc::clone(&obj)));
                    }
                }
                return Err(format!("Method '{}' not found on self", method_name));
            } else {
                // Property access: self.property
                let prop_name = rest;
                let self_val = self.env.get("__self__")?;
                if let Value::Object(obj) = self_val {
                    if let Some(val) = obj.fields.get(prop_name) {
                        return Ok(val.clone());
                    }
                }
                return Err(format!("Property '{}' not found on self", prop_name));
            }
        }

        // For method chains like s.trim().upper(), we need to evaluate left-to-right
        // Find the LAST method call (rightmost pattern of ".method(" or just "method(")
        // We need to find the rightmost '(' that has a '.' or start before it
        let mut last_method_start = None;
        let mut last_method_end = None;
        let chars: Vec<char> = expr_str.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '(' {
                // Find the start of this method name (go back to '.' or start)
                let mut j = i;
                while j > 0 && chars[j-1] != '.' && chars[j-1] != '(' && j > 0 {
                    j -= 1;
                }
                last_method_start = Some(j);
                last_method_end = Some(i);
            }
            i += 1;
        }

        if let Some(method_start) = last_method_start {
            if let Some(paren_pos) = last_method_end {
                let before_paren = &expr_str[..paren_pos];
                
                // Check if there's a '.' before this method (indicating object.method)
                if method_start > 0 && chars[method_start - 1] == '.' {
                    let obj_expr = &expr_str[..method_start - 1];
                    let method_name = &expr_str[method_start..paren_pos];

                    // Extract arguments between parentheses
                    let args_start = paren_pos + 1;
                    let args_end = Self::find_matching_paren(expr_str, args_start).unwrap_or(expr_str.len());
                    let args_str = &expr_str[args_start..args_end];

                    // Evaluate the object expression (could be a variable or another method call)
                    let obj_val = self.evaluate_interpolation_expr(obj_expr)?;

                    // Parse arguments
                    let args = self.parse_interpolation_args(args_str)?;

                    // Call method on object
                    return self.call_method_on_value(obj_val, method_name, &args);
                } else {
                    // Simple function call: function()
                    let func_name = before_paren;
                    let args_start = paren_pos + 1;
                    let args_end = Self::find_matching_paren(expr_str, args_start).unwrap_or(expr_str.len());
                    let args_str = &expr_str[args_start..args_end];
                    let args = self.parse_interpolation_args(args_str)?;

                    let func_val = self.env.get(func_name)?;
                    match func_val {
                        Value::Function(func) => {
                            return self.call_function(&func, &args, None);
                        }
                        Value::NativeFunction(native) => {
                            return native(&args);
                        }
                        _ => return Err(format!("'{}' is not a callable function", func_name)),
                    }
                }
            }
        }

        // Check for property access: identifier.property
        if let Some(dot_pos) = expr_str.rfind('.') {
            let obj_name = &expr_str[..dot_pos];
            let prop_name = &expr_str[dot_pos + 1..];

            let obj_val = self.evaluate_interpolation_expr(obj_name)?;
            if let Value::Object(obj) = obj_val {
                if let Some(val) = obj.fields.get(prop_name) {
                    return Ok(val.clone());
                }
            }
            return Err(format!("Property '{}' not found", prop_name));
        }

        // Simple variable reference
        self.env.get(expr_str)
    }

    fn call_method_on_value(&mut self, obj_val: Value, method_name: &str, args: &[Value]) -> Result<Value, String> {
        match obj_val {
            Value::Object(obj_rc) => {
                if let Some(func) = obj_rc.methods.get(method_name).cloned() {
                    return self.call_function(&func, args, Some(Rc::clone(&obj_rc)));
                }
                if let Some(class_def) = self.classes.get(&obj_rc.class_name).cloned() {
                    if let Some(func) = class_def.methods.get(method_name).cloned() {
                        return self.call_function(&func, args, Some(Rc::clone(&obj_rc)));
                    }
                }
                Err(format!("Method '{}' not found on {}", method_name, obj_rc.class_name))
            }
            Value::Str(s) => {
                return self.call_primitive_method("str", method_name, args, Some(Value::Str(s)));
            }
            Value::Int(i) => {
                return self.call_primitive_method("int", method_name, args, Some(Value::Int(i)));
            }
            Value::Float(f) => {
                return self.call_primitive_method("float", method_name, args, Some(Value::Float(f)));
            }
            Value::Bool(b) => {
                return self.call_primitive_method("bool", method_name, args, Some(Value::Bool(b)));
            }
            Value::Module(module_name) => {
                if module_name.starts_with("class:") {
                    let class_name = &module_name[6..];
                    if let Some(class_def) = self.classes.get(class_name).cloned() {
                        if let Some(func) = class_def.static_methods.get(method_name) {
                            return self.call_function(func, args, None);
                        }
                    }
                    return Err(format!("Static method '{}' not found on class {}", method_name, class_name));
                }
                return self.call_module_method(&module_name, method_name, args);
            }
            Value::ModuleEnv(module_env) => {
                let func_val = module_env.get(method_name)
                    .map_err(|_| format!("Function '{}' not found in module", method_name))?;
                match func_val {
                    Value::Function(func) => {
                        return self.call_function(&func, args, None);
                    }
                    Value::NativeFunction(native) => {
                        return native(args);
                    }
                    _ => return Err(format!("'{}' is not a function in module", method_name)),
                }
            }
            _ => Err(format!("Cannot call method '{}' on {:?}", method_name, obj_val)),
        }
    }
    
    fn literal_to_value(&self, lit: &Literal) -> Result<Value, String> {
        match lit {
            Literal::Int(i) => Ok(Value::Int(*i)),
            Literal::Float(f) => Ok(Value::Float(*f)),
            Literal::Str(s) => Ok(Value::Str(s.clone())),
            Literal::Bool(b) => Ok(Value::Bool(*b)),
            Literal::Void => Ok(Value::Void),
        }
    }
    
    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::Str(s) => !s.is_empty(),
            Value::List(l) => !l.borrow().is_empty(),
            Value::Void => false,
            _ => true,
        }
    }
    
    fn add(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::Str(a), Value::Str(b)) => Ok(Value::Str(format!("{}{}", a, b))),
            (Value::List(a), Value::List(b)) => {
                let mut new_vec = a.borrow().clone();
                new_vec.extend(b.borrow().iter().cloned());
                Ok(Value::List(Rc::new(RefCell::new(new_vec))))
            }
            _ => Err(format!("Cannot add {:?} and {:?}", left, right)),
        }
    }
    
    fn subtract(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),
            _ => Err(format!("Cannot subtract {:?} from {:?}", right, left)),
        }
    }
    
    fn multiply(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * *b as f64)),
            _ => Err(format!("Cannot multiply {:?} and {:?}", left, right)),
        }
    }
    
    fn divide(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(format!("Division by zero"))
                } else {
                    Ok(Value::Int(a / b))
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if b.abs() < f64::EPSILON {
                    Err(format!("Division by zero"))
                } else {
                    Ok(Value::Float(a / b))
                }
            }
            (Value::Int(a), Value::Float(b)) => {
                if b.abs() < f64::EPSILON {
                    Err(format!("Division by zero"))
                } else {
                    Ok(Value::Float(*a as f64 / b))
                }
            }
            (Value::Float(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(format!("Division by zero"))
                } else {
                    Ok(Value::Float(a / *b as f64))
                }
            }
            _ => Err(format!("Cannot divide {:?} by {:?}", left, right)),
        }
    }
    
    fn modulo(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(format!("Modulo by zero"))
                } else {
                    Ok(Value::Int(a % b))
                }
            }
            _ => Err(format!("Cannot modulo {:?} by {:?}", left, right)),
        }
    }
    
    fn less_than(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((*a as f64) < *b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(*a < *b as f64)),
            (Value::Str(a), Value::Str(b)) => Ok(Value::Bool(a < b)),
            _ => Err(format!("Cannot compare {:?} < {:?}", left, right)),
        }
    }
    
    fn greater_than(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((*a as f64) > *b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(*a > *b as f64)),
            (Value::Str(a), Value::Str(b)) => Ok(Value::Bool(a > b)),
            _ => Err(format!("Cannot compare {:?} > {:?}", left, right)),
        }
    }
    
    pub fn call_function(&mut self, func: &Function, args: &[Value], instance: Option<Rc<Object>>) -> Result<Value, String> {
        // Use the function's closure environment as the enclosing, or current env if not available
        let enclosing = func.closure_env.clone().unwrap_or_else(|| Rc::clone(&self.env));
        let new_env = Environment::with_enclosing(enclosing);
        let old_env = std::mem::replace(&mut self.env, Rc::new(new_env));

        if let Some(obj) = &instance {
            self.define_variable("__self__".to_string(), Value::Object(Rc::clone(obj)));
        }

        let mut arg_idx = 0;
        for (i, param) in func.params.iter().enumerate() {
            if func.is_method && i == 0 && param.name == "self" {
                continue;
            }

            if let Some(arg) = args.get(arg_idx) {
                self.define_variable(param.name.clone(), arg.clone());
                arg_idx += 1;
            }
        }

        let mut result = Value::Void;
        for stmt in &func.body {
            match self.execute(stmt) {
                Ok(val) => result = val,
                Err(e) => {
                    self.env = old_env;
                    if e == "RETURN" {
                        return Ok(self.return_value.take().unwrap_or(Value::Void));
                    }
                    return Err(e);
                }
            }
        }

        self.env = old_env;
        Ok(result)
    }

    fn call_primitive_method(&mut self, class_name: &str, method: &str, args: &[Value], self_val: Option<Value>) -> Result<Value, String> {
        // Check loaded module environments for the class and method
        if let Some(module_env) = self.loaded_modules.get(class_name) {
            // Try to get the class from the module
            if let Ok(Value::Class(class_def)) = module_env.get(class_name) {
                if let Some(func) = class_def.methods.get(method) {
                    if let Some(self_value) = self_val {
                        let mut obj = Object::new(class_name.to_string());
                        obj.fields.insert("_value".to_string(), self_value);
                        let obj_rc = Rc::new(obj);
                        return self.call_function(func, args, Some(obj_rc));
                    } else {
                        return self.call_function(func, args, None);
                    }
                }
            }
        }
        
        // Also check self.classes in case the class was registered there
        if let Some(class_def) = self.classes.get(class_name).cloned() {
            if let Some(func) = class_def.methods.get(method) {
                if let Some(self_value) = self_val {
                    let mut obj = Object::new(class_name.to_string());
                    obj.fields.insert("_value".to_string(), self_value);
                    let obj_rc = Rc::new(obj);
                    return self.call_function(&func, args, Some(obj_rc));
                } else {
                    return self.call_function(&func, args, None);
                }
            }
        }
        
        Err(format!("Cannot call method '{}' on {}", method, self_val.map_or("unknown".to_string(), |v| format!("{}", v))))
    }

    fn call_string_method(&mut self, s: &str, method: &str, args: &[Value]) -> Result<Value, String> {
        match method {
            "trim" => Ok(Value::Str(s.trim().to_string())),
            "upper" => Ok(Value::Str(s.to_uppercase())),
            "lower" => Ok(Value::Str(s.to_lowercase())),
            "reverse" => Ok(Value::Str(s.chars().rev().collect())),
            "length" => Ok(Value::Int(s.chars().count() as i64)),
            "contains" => {
                if args.is_empty() {
                    return Err(format!("contains() requires 1 argument"));
                }
                if let Value::Str(sub) = &args[0] {
                    Ok(Value::Bool(s.contains(sub)))
                } else {
                    Err(format!("contains() requires string argument"))
                }
            }
            "split" => {
                if args.is_empty() {
                    return Err(format!("split() requires 1 argument"));
                }
                if let Value::Str(delimiter) = &args[0] {
                    let parts: Vec<Value> = s.split(delimiter)
                        .map(|p| Value::Str(p.to_string()))
                        .collect();
                    Ok(Value::List(Rc::new(RefCell::new(parts))))
                } else {
                    Err(format!("split() requires string argument"))
                }
            }
            _ => Err(format!("String has no method '{}'", method)),
        }
    }

    fn call_list_method(&mut self, list: &Rc<RefCell<Vec<Value>>>, method: &str, args: &[Value]) -> Result<Value, String> {
        match method {
            "length" => Ok(Value::Int(list.borrow().len() as i64)),
            "first" => {
                list.borrow().first().cloned().ok_or(format!("Cannot get first element of empty list"))
            }
            "last" => {
                list.borrow().last().cloned().ok_or(format!("Cannot get last element of empty list"))
            }
            "contains" => {
                if args.is_empty() {
                    return Err(format!("contains() requires 1 argument"));
                }
                Ok(Value::Bool(list.borrow().contains(&args[0])))
            }
            "join" => {
                if args.is_empty() {
                    return Err(format!("join() requires 1 argument"));
                }
                if let Value::Str(delimiter) = &args[0] {
                    let strs: Vec<String> = list.borrow().iter().map(|v| format!("{}", v)).collect();
                    Ok(Value::Str(strs.join(delimiter)))
                } else {
                    Err(format!("join() requires string argument"))
                }
            }
            "push" => {
                if args.is_empty() {
                    return Err(format!("push() requires 1 argument"));
                }
                list.borrow_mut().push(args[0].clone());
                Ok(Value::Void)
            }
            "filter" => {
                if args.is_empty() {
                    return Err(format!("filter() requires a function argument"));
                }
                if let Value::Function(func) = &args[0] {
                    let mut result = Vec::new();
                    for item in list.borrow().iter() {
                        let filtered = self.call_function(func, &[item.clone()], None)?;
                        if self.is_truthy(&filtered) {
                            result.push(item.clone());
                        }
                    }
                    Ok(Value::List(Rc::new(RefCell::new(result))))
                } else {
                    Err(format!("filter() requires a function argument"))
                }
            }
            "map" => {
                if args.is_empty() {
                    return Err(format!("map() requires a function argument"));
                }
                if let Value::Function(func) = &args[0] {
                    let mut result = Vec::new();
                    for item in list.borrow().iter() {
                        let mapped = self.call_function(func, &[item.clone()], None)?;
                        result.push(mapped);
                    }
                    Ok(Value::List(Rc::new(RefCell::new(result))))
                } else {
                    Err(format!("map() requires a function argument"))
                }
            }
            "reduce" => {
                if args.len() < 2 {
                    return Err(format!("reduce() requires initial value and function"));
                }
                if let Value::Function(func) = &args[1] {
                    let mut acc = args[0].clone();
                    for item in list.borrow().iter() {
                        acc = self.call_function(func, &[acc.clone(), item.clone()], None)?;
                    }
                    Ok(acc)
                } else {
                    Err(format!("reduce() requires a function argument"))
                }
            }
            _ => Err(format!("List has no method '{}'", method)),
        }
    }
    
    fn call_module_method(&mut self, module: &str, method: &str, args: &[Value]) -> Result<Value, String> {
        match module {
            "math" => self.call_math_method(method, args),
            "list" => self.call_list_module_method(method, args),
            "fs" => self.call_fs_method(method, args),
            "env" => self.call_env_method(method, args),
            "time" => self.call_time_method(method, args),
            _ => Err(format!("Unknown module: {}", module)),
        }
    }
    
    fn call_math_method(&self, method: &str, args: &[Value]) -> Result<Value, String> {
        match method {
            "abs" => {
                if args.is_empty() { return Err(format!("abs() requires 1 argument")); }
                match &args[0] {
                    Value::Int(i) => Ok(Value::Int(i.abs())),
                    Value::Float(f) => Ok(Value::Float(f.abs())),
                    _ => Err(format!("abs() requires numeric argument")),
                }
            }
            "min" => {
                if args.len() < 2 { return Err(format!("min() requires 2 arguments")); }
                match (&args[0], &args[1]) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(*std::cmp::min(a, b))),
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.min(*b))),
                    _ => Err(format!("min() requires numeric arguments")),
                }
            }
            "max" => {
                if args.len() < 2 { return Err(format!("max() requires 2 arguments")); }
                match (&args[0], &args[1]) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(*std::cmp::max(a, b))),
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.max(*b))),
                    _ => Err(format!("max() requires numeric arguments")),
                }
            }
            "pow" => {
                if args.len() < 2 { return Err(format!("pow() requires 2 arguments")); }
                match (&args[0], &args[1]) {
                    (Value::Int(base), Value::Int(exp)) => {
                        Ok(Value::Int(base.pow(*exp as u32)))
                    }
                    (Value::Float(base), Value::Float(exp)) => {
                        Ok(Value::Float(base.powf(*exp)))
                    }
                    _ => Err(format!("pow() requires numeric arguments")),
                }
            }
            "sqrt" => {
                if args.is_empty() { return Err(format!("sqrt() requires 1 argument")); }
                match &args[0] {
                    Value::Int(i) => {
                        let result = (*i as f64).sqrt();
                        if result.fract() == 0.0 {
                            Ok(Value::Int(result as i64))
                        } else {
                            Ok(Value::Float(result))
                        }
                    }
                    Value::Float(f) => {
                        let result = f.sqrt();
                        if result.fract() == 0.0 {
                            Ok(Value::Int(result as i64))
                        } else {
                            Ok(Value::Float(result))
                        }
                    }
                    _ => Err(format!("sqrt() requires numeric argument")),
                }
            }
            "gcd" => {
                if args.len() < 2 { return Err(format!("gcd() requires 2 arguments")); }
                match (&args[0], &args[1]) {
                    (Value::Int(a), Value::Int(b)) => {
                        fn gcd(mut a: i64, mut b: i64) -> i64 {
                            a = a.abs();
                            b = b.abs();
                            while b != 0 {
                                let t = b;
                                b = a % b;
                                a = t;
                            }
                            a
                        }
                        Ok(Value::Int(gcd(*a, *b)))
                    }
                    _ => Err(format!("gcd() requires integer arguments")),
                }
            }
            "lcm" => {
                if args.len() < 2 { return Err(format!("lcm() requires 2 arguments")); }
                match (&args[0], &args[1]) {
                    (Value::Int(a), Value::Int(b)) => {
                        fn gcd(mut a: i64, mut b: i64) -> i64 {
                            a = a.abs();
                            b = b.abs();
                            while b != 0 {
                                let t = b;
                                b = a % b;
                                a = t;
                            }
                            a
                        }
                        if *a == 0 || *b == 0 {
                            Ok(Value::Int(0))
                        } else {
                            Ok(Value::Int((a * b).abs() / gcd(*a, *b)))
                        }
                    }
                    _ => Err(format!("lcm() requires integer arguments")),
                }
            }
            "is_prime" => {
                if args.is_empty() { return Err(format!("is_prime() requires 1 argument")); }
                match &args[0] {
                    Value::Int(n) => {
                        if *n < 2 {
                            Ok(Value::Bool(false))
                        } else if *n == 2 {
                            Ok(Value::Bool(true))
                        } else if *n % 2 == 0 {
                            Ok(Value::Bool(false))
                        } else {
                            let mut i = 3;
                            while i * i <= *n {
                                if *n % i == 0 {
                                    return Ok(Value::Bool(false));
                                }
                                i += 2;
                            }
                            Ok(Value::Bool(true))
                        }
                    }
                    _ => Err(format!("is_prime() requires integer argument")),
                }
            }
            "is_even" => {
                if args.is_empty() { return Err(format!("is_even() requires 1 argument")); }
                match &args[0] {
                    Value::Int(n) => Ok(Value::Bool(n % 2 == 0)),
                    _ => Err(format!("is_even() requires integer argument")),
                }
            }
            _ => Err(format!("math.{} is not defined", method)),
        }
    }
    
    fn call_list_module_method(&self, method: &str, args: &[Value]) -> Result<Value, String> {
        match method {
            "range" => {
                if args.len() < 2 { return Err(format!("range() requires 2 arguments")); }
                match (&args[0], &args[1]) {
                    (Value::Int(start), Value::Int(end)) => {
                        let mut result = Vec::new();
                        let mut i = *start;
                        while i < *end {
                            result.push(Value::Int(i));
                            i += 1;
                        }
                        Ok(Value::List(Rc::new(RefCell::new(result))))
                    }
                    _ => Err(format!("range() requires integer arguments")),
                }
            }
            _ => Err(format!("list.{} is not defined", method)),
        }
    }
    
    fn call_fs_method(&self, method: &str, args: &[Value]) -> Result<Value, String> {
        use std::fs;
        use std::path::Path;
        
        match method {
            "read" => {
                if args.is_empty() { return Err(format!("read() requires 1 argument")); }
                if let Value::Str(path) = &args[0] {
                    match fs::read_to_string(path) {
                        Ok(content) => Ok(Value::Str(content)),
                        Err(e) => Err(format!("Failed to read file: {}", e)),
                    }
                } else {
                    Err(format!("read() requires string argument"))
                }
            }
            "write" => {
                if args.len() < 2 { return Err(format!("write() requires 2 arguments")); }
                if let (Value::Str(path), Value::Str(content)) = (&args[0], &args[1]) {
                    match fs::write(path, content) {
                        Ok(_) => Ok(Value::Void),
                        Err(e) => Err(format!("Failed to write file: {}", e)),
                    }
                } else {
                    Err(format!("write() requires string arguments"))
                }
            }
            "exists" => {
                if args.is_empty() { return Err(format!("exists() requires 1 argument")); }
                if let Value::Str(path) = &args[0] {
                    Ok(Value::Bool(Path::new(path).exists()))
                } else {
                    Err(format!("exists() requires string argument"))
                }
            }
            "remove" => {
                if args.is_empty() { return Err(format!("remove() requires 1 argument")); }
                if let Value::Str(path) = &args[0] {
                    match fs::remove_file(path) {
                        Ok(_) => Ok(Value::Void),
                        Err(e) => Err(format!("Failed to remove file: {}", e)),
                    }
                } else {
                    Err(format!("remove() requires string argument"))
                }
            }
            "dirname" => {
                if args.is_empty() { return Err(format!("dirname() requires 1 argument")); }
                if let Value::Str(path) = &args[0] {
                    let parent = Path::new(path).parent()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_default();
                    Ok(Value::Str(parent))
                } else {
                    Err(format!("dirname() requires string argument"))
                }
            }
            "basename" => {
                if args.is_empty() { return Err(format!("basename() requires 1 argument")); }
                if let Value::Str(path) = &args[0] {
                    let name = Path::new(path).file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();
                    Ok(Value::Str(name))
                } else {
                    Err(format!("basename() requires string argument"))
                }
            }
            "extension" => {
                if args.is_empty() { return Err(format!("extension() requires 1 argument")); }
                if let Value::Str(path) = &args[0] {
                    let ext = Path::new(path).extension()
                        .map(|e| e.to_string_lossy().to_string())
                        .unwrap_or_default();
                    Ok(Value::Str(ext))
                } else {
                    Err(format!("extension() requires string argument"))
                }
            }
            _ => Err(format!("fs.{} is not defined", method)),
        }
    }
    
    fn call_env_method(&self, method: &str, args: &[Value]) -> Result<Value, String> {
        use std::env;
        
        match method {
            "cwd" => {
                match env::current_dir() {
                    Ok(path) => Ok(Value::Str(path.to_string_lossy().to_string())),
                    Err(_) => Err(format!("Failed to get current directory")),
                }
            }
            "home" => {
                match env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
                    Ok(path) => Ok(Value::Str(path)),
                    Err(_) => Ok(Value::Str("".to_string())),
                }
            }
            "hostname" => {
                match env::var("HOSTNAME").or_else(|_| env::var("COMPUTERNAME")) {
                    Ok(name) => Ok(Value::Str(name)),
                    Err(_) => Ok(Value::Str("unknown".to_string())),
                }
            }
            "os" => {
                Ok(Value::Str(env::consts::OS.to_string()))
            }
            "get" => {
                if args.is_empty() { return Err(format!("get() requires 1 argument")); }
                if let Value::Str(key) = &args[0] {
                    match env::var(key) {
                        Ok(val) => Ok(Value::Str(val)),
                        Err(_) => Ok(Value::Void),
                    }
                } else {
                    Err(format!("get() requires string argument"))
                }
            }
            _ => Err(format!("env.{} is not defined", method)),
        }
    }
    
    fn call_time_method(&self, method: &str, args: &[Value]) -> Result<Value, String> {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        match method {
            "now" => {
                match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(duration) => Ok(Value::Int(duration.as_millis() as i64)),
                    Err(_) => Ok(Value::Int(0)),
                }
            }
            "sleep" => {
                if args.is_empty() { return Err(format!("sleep() requires 1 argument")); }
                if let Value::Int(ms) = &args[0] {
                    std::thread::sleep(std::time::Duration::from_millis(*ms as u64));
                    Ok(Value::Void)
                } else {
                    Err(format!("sleep() requires integer argument"))
                }
            }
            _ => Err(format!("time.{} is not defined", method)),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

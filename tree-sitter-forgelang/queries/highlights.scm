; ForgeLang Tree-sitter Query for Syntax Highlighting
; Based on Rust/TypeScript highlighting conventions

; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
; COMMENTS
; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
(comment) @comment @spell
(box_comment) @comment @spell

; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
; LITERALS
; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
(integer_literal) @number
(float_literal) @number.float
(string_literal) @string
(string_content) @string
(escape_sequence) @string.escape
(string_interpolation) @string.special
(boolean_literal) @boolean
(char_literal) @character

; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
; KEYWORDS
; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[
  "module"
  "import"
  "from"
  "as"
] @keyword.import

[
  "const"
  "var"
  "type"
] @keyword.storage

[
  "fn"
  "return"
] @keyword.function

[
  "if"
  "else"
  "match"
  "for"
  "in"
  "while"
  "break"
  "continue"
] @keyword.conditional

[
  "class"
  "interface"
  "implement"
  "implements"
  "enum"
] @keyword.type

[
  "self"
  "Self"
] @keyword.self

; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
; OPERATORS
; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[
  "=>"
  "->"
] @keyword.operator

[
  "+"
  "-"
  "*"
  "/"
  "%"
  "&&"
  "||"
  "!"
  "=="
  "!="
  "<"
  "<="
  ">"
  ">="
  "="
  "+="
  "-="
  "*="
  "/="
  "%="
] @operator

; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
; PUNCTUATION
; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  "."
  ","
  ":"
  ";"
  "_"
] @punctuation.delimiter

; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
; TYPES
; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
(type_annotation) @type
(generic_type) @type
(function_type) @type
(tuple_type) @type
(list_type) @type
(map_type) @type
(set_type) @type
(option_type) @type
(result_type) @type

(primitive_type) @type.builtin

; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
; IDENTIFIERS
; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
(identifier) @variable

; Function definitions
(function_declaration
  name: (identifier) @function)

; Method definitions
(method_declaration
  name: (identifier) @method)

; Function calls
(call_expression
  function: (identifier) @function.call)

; Method calls
(method_call_expression
  method: (identifier) @method.call)

; Field access
(field_expression
  field: (identifier) @property)

; Parameters
(parameter
  name: (identifier) @variable.parameter)

; Enum variants
(enum_variant
  name: (identifier) @type.enum)

; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
; SPECIAL PATTERNS
; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
; Match patterns
(match_pattern
  (identifier) @variable)

; Wildcard
"_" @keyword

; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
; BUILTIN TYPES
; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[
  "int"
  "f64"
  "float"
  "str"
  "bool"
  "void"
] @type.builtin

; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
; BUILTIN FUNCTIONS
; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
((identifier) @function.builtin
 (#match? @function.builtin "^(print|println|eprint|eprintln)$"))

; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
; CONSTANTS
; ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
(const_declaration
  name: (identifier) @constant)

((identifier) @constant
 (#match? @constant "^[A-Z][A-Z0-9_]*$"))

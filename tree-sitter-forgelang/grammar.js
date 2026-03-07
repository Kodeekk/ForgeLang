/**
 * Tree-sitter grammar for ForgeLang v2.0
 * A modern systems programming language with Rust-like syntax
 */

module.exports = grammar({
  name: 'forgelang',

  extras: $ => [
    /\s/,
    $.comment,
    $.box_comment,
  ],

  externals: $ => [],

  inline: $ => [],

  conflicts: $ => [
    [$.type_annotation, $.expression],
    [$.generic_type, $.binary_expression],
  ],

  word: $ => $.identifier,

  rules: {
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // SOURCE FILE
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    source_file: $ => seq(
      optional($.module_declaration),
      repeat($.statement)
    ),

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // MODULE DECLARATION
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    module_declaration: $ => seq(
      'module',
      $.module_path,
      ';'
    ),

    module_path: $ => seq(
      $.identifier,
      repeat(seq('.', $.identifier))
    ),

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // COMMENTS
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    comment: $ => token(seq('//', /.*/)),

    box_comment: $ => token(seq(
      '╔',
      repeat(seq(/[^\n╚]/, optional('\n'))),
      '╚',
      /[═╝]*/
    )),

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // STATEMENTS
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    statement: $ => choice(
      $.import_statement,
      $.const_declaration,
      $.var_declaration,
      $.type_alias,
      $.enum_declaration,
      $.interface_declaration,
      $.class_declaration,
      $.implement_declaration,
      $.function_declaration,
      $.expression_statement,
      $.return_statement,
      $.if_statement,
      $.match_statement,
      $.for_statement,
      $.while_statement,
      $.break_statement,
      $.continue_statement,
      $.block,
    ),

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // IMPORTS
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    import_statement: $ => choice(
      seq('import', $.module_path, ';'),
      seq('import', $.module_path, 'as', $.identifier, ';'),
      seq('import', $.import_list, 'from', $.module_path, ';')
    ),

    import_list: $ => seq(
      '[',
      commaSep($.import_item),
      ']'
    ),

    import_item: $ => choice(
      $.identifier,
      seq($.identifier, 'as', $.identifier)
    ),

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // DECLARATIONS
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    const_declaration: $ => seq(
      'const',
      $.identifier,
      optional($.type_annotation),
      '=',
      $.expression,
      ';'
    ),

    var_declaration: $ => seq(
      'var',
      $.identifier,
      optional($.type_annotation),
      optional(seq('=', $.expression)),
      ';'
    ),

    type_alias: $ => seq(
      'type',
      $.identifier,
      optional($.type_parameters),
      '=',
      $.type_annotation,
      ';'
    ),

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // ENUMS
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    enum_declaration: $ => seq(
      'enum',
      $.identifier,
      '{',
      repeat($.enum_variant),
      '}'
    ),

    enum_variant: $ => choice(
      seq($.identifier, optional($.variant_fields), ','),
      $.identifier
    ),

    variant_fields: $ => seq(
      '(',
      commaSep(seq($.identifier, ':', $.type_annotation)),
      ')'
    ),

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // INTERFACES
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    interface_declaration: $ => seq(
      'interface',
      $.identifier,
      '{',
      repeat($.interface_method),
      '}'
    ),

    interface_method: $ => seq(
      'fn',
      $.identifier,
      $.parameters,
      optional($.type_annotation),
      ';'
    ),

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // CLASSES
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    class_declaration: $ => seq(
      'class',
      $.identifier,
      optional($.type_parameters),
      optional($.implements_clause),
      '{',
      repeat($.class_member),
      '}'
    ),

    implements_clause: $ => seq(
      'implements',
      commaSep($.type_annotation)
    ),

    class_member: $ => choice(
      $.field_declaration,
      $.method_declaration
    ),

    field_declaration: $ => seq(
      'var',
      $.identifier,
      optional($.type_annotation),
      ';'
    ),

    method_declaration: $ => seq(
      'fn',
      $.identifier,
      $.parameters,
      optional($.type_annotation),
      $.block
    ),

    implement_declaration: $ => seq(
      'implement',
      $.type_annotation,
      'for',
      $.identifier,
      '{',
      repeat($.method_declaration),
      '}'
    ),

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // FUNCTIONS
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    function_declaration: $ => seq(
      'fn',
      $.identifier,
      optional($.type_parameters),
      $.parameters,
      optional($.type_annotation),
      $.block
    ),

    type_parameters: $ => seq(
      '<',
      commaSep($.identifier),
      '>'
    ),

    parameters: $ => seq(
      '(',
      optional(commaSep($.parameter)),
      ')'
    ),

    parameter: $ => seq(
      $.identifier,
      optional($.type_annotation)
    ),

    type_annotation: $ => choice(
      'int',
      'f64',
      'float',
      'str',
      'bool',
      'void',
      'Self',
      $.generic_type,
      $.function_type,
      $.tuple_type,
      $.list_type,
      $.map_type,
      $.set_type,
      $.option_type,
      $.result_type
    ),

    generic_type: $ => seq(
      $.identifier,
      '<',
      commaSep($.type_annotation),
      '>'
    ),

    function_type: $ => seq(
      'fn',
      $.parameters,
      '->',
      $.type_annotation
    ),

    tuple_type: $ => seq(
      '(',
      commaSep($.type_annotation),
      ')'
    ),

    list_type: $ => seq('list', '<', $.type_annotation, '>'),
    map_type: $ => seq('map', '<', $.type_annotation, ',', $.type_annotation, '>'),
    set_type: $ => seq('set', '<', $.type_annotation, '>'),
    option_type: $ => seq('Option', '<', $.type_annotation, '>'),
    result_type: $ => seq('Result', '<', $.type_annotation, ',', $.type_annotation, '>'),

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // EXPRESSIONS
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    expression_statement: $ => seq($.expression, ';'),

    expression: $ => choice(
      $.binary_expression,
      $.unary_expression,
      $.call_expression,
      $.method_call_expression,
      $.index_expression,
      $.field_expression,
      $.assignment_expression,
      $.literal,
      $.identifier,
      $.self_expression,
      $.lambda_expression,
      $.tuple_expression,
      $.list_literal,
      $.map_literal,
      $.set_literal,
      $.object_literal,
      $.parenthesized_expression
    ),

    binary_expression: $ => choice(
      ...[
        ['&&', 'left'],
        ['||', 'left'],
        ['==', 'left'],
        ['!=', 'left'],
        ['<', 'left'],
        ['<=', 'left'],
        ['>', 'left'],
        ['>=', 'left'],
        ['+', 'left'],
        ['-', 'left'],
        ['*', 'left'],
        ['/', 'left'],
        ['%', 'left'],
      ].map(([operator, associativity]) =>
        prec.left(associativity === 'left' ? 10 : 20, seq(
          $.expression,
          operator,
          $.expression
        ))
      )
    ),

    unary_expression: $ => prec.right(seq(
      choice('-', '!'),
      $.expression
    )),

    call_expression: $ => seq(
      $.expression,
      $.arguments
    ),

    method_call_expression: $ => prec.left(seq(
      $.expression,
      '.',
      $.identifier,
      $.arguments
    )),

    index_expression: $ => prec.left(seq(
      $.expression,
      '[',
      $.expression,
      ']'
    )),

    field_expression: $ => prec.left(seq(
      $.expression,
      '.',
      $.identifier
    )),

    assignment_expression: $ => prec.right(seq(
      $.expression,
      choice('=', '+=', '-=', '*=', '/=', '%='),
      $.expression
    )),

    lambda_expression: $ => seq(
      'fn',
      $.parameters,
      optional($.type_annotation),
      $.block
    ),

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // CONTROL FLOW
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    block: $ => seq(
      '{',
      repeat($.statement),
      '}'
    ),

    if_statement: $ => seq(
      'if',
      $.expression,
      $.block,
      repeat($.else_if_clause),
      optional($.else_clause)
    ),

    else_if_clause: $ => seq(
      'else',
      'if',
      $.expression,
      $.block
    ),

    else_clause: $ => seq(
      'else',
      $.block
    ),

    match_statement: $ => seq(
      'match',
      $.expression,
      '{',
      repeat($.match_arm),
      '}'
    ),

    match_arm: $ => seq(
      $.match_pattern,
      '=>',
      $.expression,
      optional(',')
    ),

    match_pattern: $ => choice(
      $.literal,
      $.identifier,
      $.enum_pattern,
      '_'
    ),

    enum_pattern: $ => seq(
      $.identifier,
      '(',
      optional(commaSep($.identifier)),
      ')'
    ),

    for_statement: $ => seq(
      'for',
      $.pattern,
      'in',
      $.expression,
      $.block
    ),

    while_statement: $ => seq(
      'while',
      $.expression,
      $.block
    ),

    break_statement: $ => seq('break', optional($.expression), ';'),
    continue_statement: $ => seq('continue', ';'),

    return_statement: $ => seq('return', optional($.expression), ';'),

    pattern: $ => choice(
      $.identifier,
      $.tuple_pattern,
      '_'
    ),

    tuple_pattern: $ => seq(
      '(',
      commaSep($.pattern),
      ')'
    ),

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // LITERALS
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    literal: $ => choice(
      $.integer_literal,
      $.float_literal,
      $.string_literal,
      $.boolean_literal,
      $.char_literal
    ),

    integer_literal: $ => token(seq(
      optional('-'),
      choice(
        /[0-9][0-9_]*/,
        /0x[0-9a-fA-F]+/,
        /0b[01]+/,
        /0o[0-7]+/
      )
    )),

    float_literal: $ => token(seq(
      optional('-'),
      /[0-9][0-9_]*\.[0-9][0-9_]*/,
      optional(/[eE][+-]?[0-9]+/)
    )),

    string_literal: $ => choice(
      seq('"', repeat($.string_content), '"'),
      seq('r"', /[^"]*/, '"')  // raw string
    ),

    string_content: $ => choice(
      $.string_interpolation,
      $.escape_sequence,
      /[^"{}\\]+/
    ),

    string_interpolation: $ => seq(
      '{',
      $.expression,
      '}'
    ),

    escape_sequence: $ => token(seq(
      '\\',
      choice(
        /["\\nrt0]/,
        /u\{[0-9a-fA-F]+\}/
      )
    )),

    boolean_literal: $ => choice('true', 'false'),

    char_literal: $ => seq(
      "'",
      choice(
        $.escape_sequence,
        /[^'\\]/
      ),
      "'"
    ),

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // COMPOSITE LITERALS
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    tuple_expression: $ => prec.right(seq(
      '(',
      $.expression,
      ',',
      commaSep1($.expression),
      ')'
    )),

    list_literal: $ => seq(
      '[',
      optional(commaSep($.expression)),
      ']'
    ),

    map_literal: $ => seq(
      'map.new',
      $.arguments
    ),

    set_literal: $ => seq(
      'set.new',
      $.arguments
    ),

    object_literal: $ => prec(-1, seq(
      $.identifier,
      '{',
      optional(commaSep($.object_field)),
      '}'
    )),

    object_field: $ => seq(
      $.identifier,
      ':',
      $.expression
    ),

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // SPECIAL EXPRESSIONS
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    self_expression: $ => 'self',

    parenthesized_expression: $ => seq(
      '(',
      $.expression,
      ')'
    ),

    arguments: $ => seq(
      '(',
      optional(commaSep($.expression)),
      ')'
    ),

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // IDENTIFIERS
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,
  }
});

/**
 * Helper for comma-separated lists (one or more)
 */
function commaSep1(rule) {
  return seq(rule, repeat(seq(',', rule)));
}

/**
 * Helper for comma-separated lists
 */
function commaSep(rule) {
  return seq(rule, repeat(seq(',', rule)), optional(','));
}

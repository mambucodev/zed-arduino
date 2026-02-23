; Arduino builtins
((identifier) @function.builtin
  (#any-of? @function.builtin
    ; Digital I/O
    "digitalRead" "digitalWrite" "pinMode"
    ; Analog I/O
    "analogRead" "analogReference" "analogWrite"
    ; Zero, Due & MKR Family
    "analogReadResolution" "analogWriteResolution"
    ; Advanced I/O
    "noTone" "pulseIn" "pulseInLong" "shiftIn" "shiftOut" "tone"
    ; Time
    "delay" "delayMicroseconds" "micros" "millis"
    ; Math
    "abs" "constrain" "map" "max" "min" "pow" "sq" "sqrt"
    ; Trigonometry
    "cos" "sin" "tan"
    ; Characters
    "isAlpha" "isAlphaNumeric" "isAscii" "isControl" "isDigit" "isGraph"
    "isHexadecimalDigit" "isLowerCase" "isPrintable" "isPunct" "isSpace"
    "isUpperCase" "isWhitespace"
    ; Random Numbers
    "random" "randomSeed"
    ; Bits and Bytes
    "bit" "bitClear" "bitRead" "bitSet" "bitWrite" "highByte" "lowByte"
    ; External Interrupts
    "attachInterrupt" "detachInterrupt"
    ; Interrupts
    "interrupts" "noInterrupts"))

((identifier) @type.builtin
  (#any-of? @type.builtin "Serial" "SPI" "Stream" "Wire" "Keyboard" "Mouse" "String"))

((identifier) @constant.builtin
  (#any-of? @constant.builtin "HIGH" "LOW" "INPUT" "OUTPUT" "INPUT_PULLUP" "LED_BUILTIN"))

(function_definition
  (function_declarator
    declarator: (identifier) @function.builtin)
  (#any-of? @function.builtin "loop" "setup"))

(call_expression
  function: (identifier) @constructor.builtin
  (#any-of? @constructor.builtin "SPISettings" "String"))

; C++ base highlights
(identifier) @variable
(field_identifier) @property
(namespace_identifier) @namespace

(concept_definition
  name: (identifier) @concept)

(requires_clause
  constraint: (template_type
    name: (type_identifier) @concept))

(call_expression
  function: (qualified_identifier
    name: (identifier) @function))

(call_expression
  (qualified_identifier
    (identifier) @function.call))

(call_expression
  (qualified_identifier
    (qualified_identifier
      (identifier) @function.call)))

(call_expression
  (qualified_identifier
    (qualified_identifier
      (qualified_identifier
        (identifier) @function.call))))

((qualified_identifier
  (qualified_identifier
    (qualified_identifier
      (qualified_identifier
        (identifier) @function.call)))) @_parent
  (#has-ancestor? @_parent call_expression))

(call_expression
  function: (identifier) @function)

(call_expression
  function: (field_expression
    field: (field_identifier) @function))

(preproc_function_def
  name: (identifier) @function.special)

(template_function
  name: (identifier) @function)

(template_method
  name: (field_identifier) @function)

(function_declarator
  declarator: (identifier) @function)

(function_declarator
  declarator: (qualified_identifier
    name: (identifier) @function))

(function_declarator
  declarator: (field_identifier) @function)

(operator_name
  (identifier)? @operator) @function

(destructor_name (identifier) @function)

((namespace_identifier) @type
 (#match? @type "^[A-Z]"))

(auto) @type
(type_identifier) @type
type: (primitive_type) @type.builtin
(sized_type_specifier) @type.builtin

(attribute
  name: (identifier) @attribute)

((identifier) @constant.builtin
 (#match? @constant.builtin "^_*[A-Z][A-Z\\d_]*$"))

(statement_identifier) @label
(this) @variable.builtin
("static_assert") @function.builtin

[
  "alignas"
  "alignof"
  "class"
  "concept"
  "consteval"
  "constexpr"
  "constinit"
  "decltype"
  "delete"
  "enum"
  "explicit"
  "extern"
  "final"
  "friend"
  "inline"
  "namespace"
  "new"
  "noexcept"
  "override"
  "private"
  "protected"
  "public"
  "requires"
  "sizeof"
  "struct"
  "template"
  "typedef"
  "typename"
  "union"
  "using"
  "virtual"
  (storage_class_specifier)
  (type_qualifier)
] @keyword

[
  "break"
  "case"
  "catch"
  "co_await"
  "co_return"
  "co_yield"
  "continue"
  "default"
  "do"
  "else"
  "for"
  "goto"
  "if"
  "return"
  "switch"
  "throw"
  "try"
  "while"
] @keyword.control

[
  "#define"
  "#elif"
  "#elifdef"
  "#elifndef"
  "#else"
  "#endif"
  "#if"
  "#ifdef"
  "#ifndef"
  "#include"
  (preproc_directive)
] @preproc

(comment) @comment

[
  (true)
  (false)
] @boolean

[
  (null)
  ("nullptr")
] @constant.builtin

(number_literal) @number

[
  (string_literal)
  (system_lib_string)
  (char_literal)
  (raw_string_literal)
] @string

(escape_sequence) @string.escape

[
  ","
  ":"
  "::"
  ";"
  (raw_string_delimiter)
] @punctuation.delimiter

[
  "{"
  "}"
  "("
  ")"
  "["
  "]"
] @punctuation.bracket

[
  "."
  ".*"
  "->*"
  "~"
  "-"
  "--"
  "-="
  "->"
  "="
  "!"
  "!="
  "|"
  "|="
  "||"
  "^"
  "^="
  "&"
  "&="
  "&&"
  "+"
  "++"
  "+="
  "*"
  "*="
  "/"
  "/="
  "%"
  "%="
  "<<"
  "<<="
  ">>"
  ">>="
  "<"
  "=="
  ">"
  "<="
  ">="
  "?"
  "and"
  "and_eq"
  "bitand"
  "bitor"
  "compl"
  "not"
  "not_eq"
  "or"
  "or_eq"
  "xor"
  "xor_eq"
] @operator

(conditional_expression ":" @operator)
(user_defined_literal (literal_suffix) @operator)

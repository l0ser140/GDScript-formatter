; GDScript formatting queries for Topiary
; This is an early work-in-progress!

; Add a space after keywords
[
  "class_name" "extends" "var" "func" "class"
  "if" "elif" "else" "for" "while"
  "const" "return" "match" "signal" "enum"
  (static_keyword)]
@append_space

; Preserve comments and strings as they are
(comment) @leaf @append_hardline
(string) @leaf

; TYPE ANNOTATION SPACING
(typed_parameter ":" @append_space)
(typed_default_parameter ":" @append_space)
(variable_statement ":" @append_space)

; PARAMETER SPACING
(parameters "," @append_space)

; ARRAY AND DICTIONARY FORMATTING
; If the array is on a single line, only insert spaces between values. If it's
; multi-line, format it with new lines.
(array
  "[" @append_empty_softline @append_indent_start
  "]" @prepend_empty_softline @append_empty_softline @prepend_indent_end)

(array "," @append_spaced_softline)

(dictionary
  "{" @append_empty_softline @append_indent_start
  "}" @prepend_empty_softline @append_empty_softline @prepend_indent_end)

(dictionary "," @append_spaced_softline)
(pair ":" @append_space)


; FUNCTIONS
(arguments "," @append_space)
"->" @prepend_space @append_space
(annotation) @append_space

(function_definition (name) @append_antispace)
(function_definition ":" @append_hardline)

(class_definition ":" @append_hardline)

(class_name_statement) @append_space
(source
    (extends_statement) @append_delimiter @append_hardline
    (#delimiter! "\n"))
(extends_statement) @prepend_space

; EMPTY LINES BETWEEN DEFINITIONS
;
; Add 2 newlines between top-level property definitions and function definitions
; Note: the . between nodes constrains the query to direct siblings (instead of
; matching a series of indirect siblings like e.g. variable + class + ... +
; function)
(source
    [(variable_statement) (function_definition) (class_definition) (signal_statement) (const_statement) (enum_definition) (constructor_definition)] @append_delimiter
    .
    [(function_definition) (constructor_definition) (class_definition)]
    (#delimiter! "\n\n"))

; CONST DEFINITIONS
(const_statement ":" @append_space)

; ENUMS
(enumerator_list
  "{" @append_input_softline @append_indent_start
  "}" @prepend_input_softline @prepend_indent_end)
(enumerator_list "," @append_spaced_softline)
(enumerator_list) @prepend_space

; CONSTRUCTORS
(constructor_definition ":" @append_hardline)


; Allow line breaks around binary operators for long expressions
; This means that if the programmer has a long expression, they can break it up by wrapping something on a line
(binary_operator
  [
    "+" "-" "*" "/" "%" "**"
    "==" "!=" "<" ">" "<=" ">=" "and"
    "or" "in" "is"]
  @prepend_spaced_softline @append_spaced_softline)


; OPERATORS
; Calculation operators (restrict to binary operator context to avoid added spaces in other contexts)
(binary_operator [
                  "+" "-" "*" "/" "%" "**"])
@prepend_space @append_space
; Comparison operators
[
    "==" "!=" "<" ">" "<=" ">="
    "and" "or" "in" "is"]
@prepend_space @append_space
; not can be at the start of an expression, so we handle it separately - needs another query for the case "is not"
"not" @append_space
; Bitwise operators
[
  "&" "|" "^" "<<" ">>"]
@prepend_space @append_space
; ~ is generally right next to the variable it operates on, so we don't add a space before it
"~" @append_space
[
    "=" ":=" "+=" "-=" "*=" "/=" "%=" "**=" "&=" "|=" "^=" "<<=" ">>="]
@prepend_space @append_space

; CONTROL FLOW FORMATTING
; Colons in control structures - remove space before colon
(if_statement ":" @prepend_antispace @append_hardline)
(elif_clause ":" @prepend_antispace @append_hardline)
(else_clause ":" @prepend_antispace @append_hardline)
(for_statement ":" @prepend_antispace @append_hardline)
(while_statement ":" @prepend_antispace @append_hardline)

; Make sure the body of control structures is indented (the preprended and
; appended indents target the body)
(body) @prepend_indent_start @append_indent_end

([(return_statement)
  (pass_statement)
  (breakpoint_statement)
  (break_statement)
  (continue_statement)
  (tool_statement)
  (enum_definition)
  (const_statement)
  (signal_statement)
  (variable_statement)
  (expression_statement)
  (if_statement)
  (elif_clause)
  (else_clause)
  (for_statement)
  (while_statement)
  (match_statement)] @append_hardline)

; allow one blank line before statement except when previous statement is extends_statement
; because we force one empty line after it in another rule
; we are using @append_space capture name here because topiary currently does not allow for custom capture names
; TODO: find a better way to allow blank line only if previous sibling is not an extends_statement
; this increased formatting time from ~140ms to ~160ms
; probably insert blank line in postprocess code, outside of this query
((_) @append_space (#not-match? @append_space "^extends")
 .
 [(return_statement)
  (pass_statement)
  (breakpoint_statement)
  (break_statement)
  (continue_statement)
  (tool_statement)
  (enum_definition)
  (const_statement)
  (signal_statement)
  (variable_statement)
  (expression_statement)
  (if_statement)
  (elif_clause)
  (else_clause)
  (for_statement)
  (while_statement)
  (match_statement)] @allow_blank_line_before)

; tree-sitter parses @tool statement as an annotation node for some reason instead of tool_statement
(source . (annotation) @append_hardline)

(setget) @prepend_indent_start @append_indent_end
(setget ":" @prepend_antispace @append_hardline)
; why body node in set_body/get_body not getting new indent even though we added indent to all body node?
(set_body ":" @prepend_antispace @append_hardline @append_indent_start)
(get_body ":" @prepend_antispace @append_hardline @append_indent_start)
(set_body) @append_indent_end
(get_body) @append_indent_end

(match_statement ":" @prepend_antispace @append_hardline)
(match_body) @prepend_indent_start @append_indent_end
(pattern_section ":" @prepend_antispace @append_hardline)
(pattern_section "," @prepend_antispace @append_space)

; not sure if something except statemenent ending uses semicolon in gdscript
(";") @delete

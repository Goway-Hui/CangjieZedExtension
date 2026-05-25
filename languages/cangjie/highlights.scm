; Cangjie syntax highlighting — basic syntax only.
; Semantic highlighting (class, function, variable, enum, interface,
; property, macro, parameter, typeParameter) is provided by the LSP
; server via textDocument/semanticTokens.

; --- Comments ---
[
  (lineComment)
  (blockComment)
] @comment

; --- Strings ---
(stringLiteral) @string
(runeLiteral) @character

; --- Numbers ---
[
  (integerLiteral)
  (floatLiteral)
  (byteLiteral)
] @number

(booleanLiteral) @constant.builtin
(unitLiteral) @constant

; --- Keywords ---
[
  "struct" "enum" "class" "interface" "extend" "type"
] @keyword.type

[
  "func" "main" "init" "operator" "macro" "prop"
] @keyword.function

[
  "let" "var" "const"
] @keyword.storage

[
  "if" "else" "match" "case"
] @keyword.conditional

[
  "for" "do" "while" "in" "break" "continue"
] @keyword.repeat

[
  "try" "catch" "finally" "throw"
] @keyword.exception

[
  "return"
] @keyword.return

[
  "import" "package"
] @keyword.import

[
  "public" "private" "protected" "internal"
  "open" "abstract" "sealed" "static"
  "override" "redef" "mut" "unsafe" "foreign" "inout"
] @keyword.modifier

[
  "is" "as" "where" "super" "this"
  "spawn" "synchronized" "quote"
  "get" "set"
] @keyword

; --- Operators ---
[
  "**" "*" "/" "%" "+" "-"
  "++" "--"
  "=" "+=" "-=" "*=" "/=" "%=" "**="
  "&&" "||" "!"
  "&" "|" "^"
  "&=" "|=" "^="
  "&&=" "||="
  "<<" ">>" "<<=" ">>="
  "==" "!="
  "<" ">" "<=" ">="
  "->" "<-" "=>"
  "|>" "~>" "??"
  "<:"
  ".." "..="
  "?" "~"
] @operator

; --- Punctuation ---
[
  "." "," ";" ":"
  "@" "..."
] @punctuation.delimiter

[
  "(" ")" "[" "]" "{" "}"
] @punctuation.bracket

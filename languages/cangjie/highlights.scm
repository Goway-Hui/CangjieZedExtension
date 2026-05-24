; ============================================================
; Cangjie Syntax Highlighting for Zed (v1.0.5 grammar)
; ============================================================

; --- Literals ---
(stringLiteral) @string
(runeLiteral) @character

(booleanLiteral) @constant.builtin

[
  (integerLiteral)
  (floatLiteral)
  (byteLiteral)
] @number

(unitLiteral) @constant

; --- Comments ---
[
  (lineComment)
  (blockComment)
] @comment

; --- Types ---
(className) @type
(structName) @type
(interfaceName) @type
(enumName) @type
(typeAliasName) @type
(superOrInterface) @type
(Thistype) @type.builtin

; Type annotations in declarations (match any child of the field)
(returnType (_) @type)
(variableDeclaration type: (_) @type)
(parameter type: (_) @type)
(namedParameter type: (_) @type)
(typeAlias type: (_) @type)

; Arrow function type
(arrowType) @type
(prefixType) @type

; Type parameter (generics)
(typeParameters
  (identifier) @type)

; Generic user type references
(userType
  (identifier) @type)

; Built-in primitive types
[
  "Int8" "Int16" "Int32" "Int64" "IntNative"
  "UInt8" "UInt16" "UInt32" "UInt64" "UIntNative"
  "Float16" "Float32" "Float64"
  "Rune" "Bool" "Unit" "Nothing" "String"
  "Array" "Range"
] @type.builtin

; --- Functions ---
(funcName) @function
(macroName) @function.macro

(functionDefinition
  (funcName) @function)
(operatorFunctionDefinition
  (operator) @function)

; Constructor / initializer
(init) @constructor
(staticInit) @constructor
(primaryInit
  (className) @constructor)

; Property
(propertyName) @property

; --- Variables ---
(varBindingPattern) @variable

(variableDeclaration
  (variableName) @variable)

; Parameters
(parameter
  paraName: (identifier) @variable.parameter)
(namedParameter
  paraName: (identifier) @variable.parameter)

; Lambda parameters
(lambdaParameter
  (varBindingPattern) @variable.parameter)

; Wildcard
(wildcardPattern) @variable.builtin

; --- Built-in variables ---
(thisSuperExpression) @variable.builtin

; --- Constants ---
(constantPattern) @constant

; --- Namespaces ---
(scoped_identifier
  (identifier) @namespace)

(packageDeclaration
  (identifier) @namespace)
(macroPackageDeclaration
  (identifier) @namespace)

; --- Macro expressions ---
(macroExpression
  (macroName) @function.macro)

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
  "spawn" "synchronized" "quote" "get" "set"
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
  "?"
] @operator

; --- Punctuation ---
[
  "." "," ";" ":"
  "@" "~" "..."
] @punctuation.delimiter

[
  "(" ")"
] @punctuation.bracket

[
  "["
  "]"
] @punctuation.bracket

[
  "{"
  "}"
] @punctuation.bracket

; String interpolation
(inlineExpression
  "${" @punctuation.special
  "}" @punctuation.special)

(inMultiLineStringExpression
  "${" @punctuation.special
  "}" @punctuation.special)

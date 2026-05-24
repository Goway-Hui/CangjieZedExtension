; ============================================================
; Cangjie Auto-Indentation for Zed (v1.0.5 grammar)
; ============================================================

; Type definition bodies
(classDefinition
  (classBody) @indent.begin)

(structDefinition
  (structBody) @indent.begin)

(interfaceDefinition
  (interfaceBody) @indent.begin)

(enumDefinition
  (enumBody) @indent.begin)

(extendDefinition
  (extendBody) @indent.begin)

; Constructor and initializer blocks
(init) @indent.begin
(primaryInit) @indent.begin

; Function blocks
(functionDefinition
  (block) @indent.begin)

(operatorFunctionDefinition
  (block) @indent.begin)

(mainDefinition
  (block) @indent.begin)

; Property getter/setter
(propertyDefinition) @indent.begin

; Call and array constructs
(callSuffix) @indent.begin
(arrayLiteral) @indent.begin

; Control flow
(ifExpression) @indent.begin

(matchExpression
  (matchCase) @indent.begin) @indent.begin

(forInExpression) @indent.begin
(whileExpression) @indent.begin
(doWhileExpression) @indent.begin

; Exception handling
(tryExpression) @indent.begin

; Lambda expressions
(lambdaExpression) @indent.begin
(trailingLambdaExpression) @indent.begin

; Foreign body
(foreignBody) @indent.begin

; Closing brackets
[
  "]" ")" "}"
] @indent.end @indent.branch

; Comments auto-indent
[
  (lineComment)
  (blockComment)
] @indent.auto

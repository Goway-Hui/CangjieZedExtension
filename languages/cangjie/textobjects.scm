; ============================================================
; Cangjie Text Objects for Zed Vim Mode
; ============================================================

; Classes and similar constructs
[
  (classDefinition (classBody) @class.inside)
  (structDefinition (structBody) @class.inside)
  (interfaceDefinition (interfaceBody) @class.inside)
  (enumDefinition (enumBody) @class.inside)
  (extendDefinition (extendBody) @class.inside)
] @class.around

; Functions
[
  (functionDefinition (block) @function.inside)
  (operatorFunctionDefinition (block) @function.inside)
  (mainDefinition (block) @function.inside)
  (macroDefinition (block) @function.inside)
  (init (block) @function.inside)
  (primaryInit (block) @function.inside)
] @function.around

; Loops
[
  (forInExpression (block) @loop.inside)
  (whileExpression (block) @loop.inside)
  (doWhileExpression body: (block) @loop.inside)
] @loop.around

; Conditionals
(ifExpression
  consequence: (block) @conditional.inside) @conditional.around

; Match cases
(matchCase
  (matchCaseBody) @conditional.inside) @conditional.around

; Comments
[
  (lineComment)
  (blockComment)
] @comment.inside

[
  (lineComment)+
  (blockComment)
] @comment.around

; Parameters
[
  (parameter)
  (namedParameter)
] @parameter.inside

[
  (typeParameters (identifier) @parameter.inside)
  (lambdaParameters (lambdaParameter) @parameter.inside)
  (parameterList)
  (primaryInitParamList)
] @parameter.around

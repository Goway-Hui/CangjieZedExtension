; Cangjie scope definitions for local variable isolation.
; Semantic-level definition/reference resolution (class, function,
; variable names) is handled by the LSP server.

; --- Scope definitions ---
(translationUnit) @local.scope
(classDefinition) @local.scope
(structDefinition) @local.scope
(interfaceDefinition) @local.scope
(enumDefinition) @local.scope
(extendDefinition) @local.scope
(functionDefinition) @local.scope
(operatorFunctionDefinition) @local.scope
(mainDefinition) @local.scope
(macroDefinition) @local.scope
(init) @local.scope
(primaryInit) @local.scope
(staticInit) @local.scope
(finalizer) @local.scope
(block) @local.scope
(lambdaExpression) @local.scope
(trailingLambdaExpression) @local.scope
(forInExpression) @local.scope
(whileExpression) @local.scope
(ifExpression) @local.scope
(matchCase) @local.scope
(matchCaseBody) @local.scope
(tryExpression) @local.scope
(synchronizedExpression) @local.scope
(spawnExpression) @local.scope
(unsafeExpression) @local.scope
(foreignBody) @local.scope
(propertyDefinition) @local.scope

; --- Local variable definitions ---
(variableDeclaration
  (variableName
    (varBindingPattern) @local.definition))

(parameter
  paraName: (identifier) @local.definition)

(namedParameter
  paraName: (identifier) @local.definition)

(lambdaParameter
  (varBindingPattern) @local.definition)

(forInExpression
  (varBindingPattern) @local.definition)

(whileExpression
  (varBindingPattern) @local.definition)

(ifExpression
  (varBindingPattern) @local.definition)

(matchCase
  (varBindingPattern) @local.definition)

(catchPattern
  (varBindingPattern) @local.definition)

(resourceSpecification
  (identifier) @local.definition)

(typeParameters
  (identifier) @local.definition)

; --- Local variable references ---
(identifier) @local.reference

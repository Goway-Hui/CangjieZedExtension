; ============================================================
; Cangjie Local Variable Scoping for Zed (v1.0.5 grammar)
; ============================================================

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

(block) @local.scope
(lambdaExpression) @local.scope
(trailingLambdaExpression) @local.scope

(forInExpression) @local.scope
(whileExpression) @local.scope
(ifExpression) @local.scope
(matchCase) @local.scope
(matchCaseBody) @local.scope

(propertyDefinition) @local.scope
(tryExpression) @local.scope

; --- Definitions ---
(variableDeclaration
  (variableName
    (varBindingPattern) @local.definition))

(variableDeclaration
  (variableName
    (tuplePattern
      (varBindingPattern) @local.definition)))

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

(matchCase
  (tuplePattern
    (varBindingPattern) @local.definition))

(matchCase
  (enumPattern
    (tuplePattern
      (varBindingPattern) @local.definition)))

(catchPattern
  (varBindingPattern) @local.definition)

(resourceSpecification
  (identifier) @local.definition)

(funcName) @local.definition
(className) @local.definition
(structName) @local.definition
(interfaceName) @local.definition
(enumName) @local.definition
(typeAliasName) @local.definition
(macroName) @local.definition
(propertyName) @local.definition

(typeParameters
  (identifier) @local.definition)

; --- References ---
(atomicVariable
  (varBindingPattern) @local.reference)

(userType
  (identifier) @local.reference)

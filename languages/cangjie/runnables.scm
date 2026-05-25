; Run main entry point
(translationUnit
  (mainDefinition) @run
  (#set! tag cangjie-main))

; Test function in class
(translationUnit
  (classDefinition
    (classBody
      (functionDefinition
        (funcName) @run @cangjie_test_name)))
  (#set! tag cangjie-test-method))

; Test class
(translationUnit
  (classDefinition
    (className) @run @cangjie_class_name)
  (#set! tag cangjie-test-class))

; Run all tests
(translationUnit)
(#set! tag cangjie-test-all)

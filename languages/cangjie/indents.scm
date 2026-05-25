; ============================================================
; Cangjie Auto-Indentation for Zed
; ============================================================
; Zed uses @indent (start) and @end (finish) captures.
; The `_ { ... }` pattern auto-indents any brace-delimited body.

(_
  "{"
  "}" @end) @indent

(_
  "["
  "]" @end) @indent

(_
  "("
  ")" @end) @indent

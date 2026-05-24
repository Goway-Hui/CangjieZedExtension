; ============================================================
; Cangjie Bracket Matching for Zed (v1.0.5 grammar)
; ============================================================

("(" @open ")" @close)
("[" @open "]" @close)
("{" @open "}" @close)

; String brackets
("\"" @open "\"" @close)
("'" @open "'" @close)

; Multi-line string brackets
("\"\"\"" @open "\"\"\"" @close)
("'''" @open "'''" @close)

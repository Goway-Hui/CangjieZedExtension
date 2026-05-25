; ============================================================
; Cangjie Bracket Matching for Zed
; ============================================================

("(" @open ")" @close)
("[" @open "]" @close)
("{" @open "}" @close)
("<" @open ">" @close)

; String brackets
("\"" @open "\"" @close)
("'" @open "'" @close)

; Multi-line string brackets
("\"\"\"" @open "\"\"\"" @close)
("'''" @open "'''" @close)

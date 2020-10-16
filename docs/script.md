# Script format
Version 0.1

The input format is UTF-8 text.

## Syntax
```
WHITESPACE : [\r\n\t ]*

SCRIPT : (INSTRUCTION WHITESPACE)*

INSTRUCTION : IDENTIFIER WHITESPACE (ARGUMENT WHITESPACE "," WHITESPACE)* "\n"

ARGUMENT : IDENTIFIER | NUMBER | STRING

IDENTIFIER : [a-zA-Z_] [a-zA-Z0-9_]*
NUMBER : HEX_NUMBER | DECIMAL_NUMBER

HEX_NUMBER : "0x" [0-9]+
DECIMAL_NUMBER : [1-9][0-9]*
```

## Instructions

### text\_count
Opcode: `0x7000`. This should usually be used as `text_count auto`.

### text
Opcode: `0x7002`. Displays text in the dialogue UI. Example use:

```
text `
This is some text! Newlines will be encoded
as they are.
`
```

### ui
Opcode: `0x7025`. Use: `ui <element>, <state>`, where `<element>` is one of:
| Name | Value | Description |
| ---- | ----- | ----------- |
| ? | 0 | unknown |
| `dialogue` | 1 | Dialogue UI (speaker/text). |
| ? | 2 | unknown |
| `sprite` | 9 | Dialogue sprite display. Necessary so the `sprite` instruction works properly in such context. |
| ? | 18 | choice? |

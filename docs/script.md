# Instructions

* [text\_count](#text_count)
* [text](#text)
* [ui](#ui)

<a name="text_count"/>
## text\_count
Opcode: `0x7000`. This should usually be used as `text_count auto`.

<a name="text"/>
## text
Opcode: `0x7002`. Displays text in the dialogue UI. Example use:

```
text `
This is some text! Newlines will be encoded
as they are.
`
```

<a name="ui"/>
## ui
Opcode: `0x7025`. Use: `ui <element>, <state>`, where `<element>` is one of:
| Name | Value | Description |
| ---- | ----- | ----------- |
| ? | 0 | unknown |
| `dialogue` | 1 | Dialogue UI (speaker/text). |
| ? | 2 | unknown |
| `sprite` | 9 | Dialogue sprite display. Necessary so the `sprite` instruction works properly in such context. |
| ? | 18 | choice? |

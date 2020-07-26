use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "formats/lin/script.pest"]
pub struct ScriptParser;

#[cfg(test)]
mod tests {
    use pest::Parser as ParserTrait;
    use super::{ScriptParser as Parser, Rule};

    #[test]
    fn text() {
        let input = r"`
this is some text
`";

        let mut result = Parser::parse(Rule::text, input).unwrap();
        let text = result.next().unwrap();
        let text_inner = text.into_inner().next().unwrap();
        assert_eq!(text_inner.as_str(), "this is some text\n");
    }

    #[test]
    fn escaped_text() {
        let input = r"`
this is some\tescaped\\text\`
\``";

        let mut result = Parser::parse(Rule::text, input).unwrap();
        let text = result.next().unwrap();
        let text_inner = text.into_inner().next().unwrap();
        assert_eq!(text_inner.as_str(), "this is some\\tescaped\\\\text\\`\n\\`");
    }
}

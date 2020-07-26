use pest_derive::Parser;
use crate::errors::*;

pub enum Arg<'a> {
    Ident(&'a str),
    Int(i64),
    Text(&'a str),
}

pub struct Instr<'a> {
    pub operation: &'a str,
    pub args: Vec<Arg<'a>>,
}

pub struct Script<'a> {
    pub instrs: Vec<Instr<'a>>,
}

pub fn parse(input: &str) -> Result<()> {
    use pest::Parser;
    let result = ScriptParser::parse(Rule::script, input)?;

    for instr in result {
        println!("{:?}", instr);
    }

    Ok(())
}

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

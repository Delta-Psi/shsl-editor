use pest_derive::Parser;
use crate::errors::*;

#[derive(Debug)]
pub enum Arg<'a> {
    Ident(&'a str),
    Int(i64),
    Text(&'a str),
}

#[derive(Debug)]
pub struct Instr<'a> {
    pub operation: &'a str,
    pub args: Vec<Arg<'a>>,
}

#[derive(Debug)]
pub struct Script<'a> {
    pub instrs: Vec<Instr<'a>>,
}

pub fn parse(input: &str) -> Result<Script> {
    use pest::Parser;
    let mut result = ScriptParser::parse(Rule::script, input)?;
    let instrs: Vec<_> = result.next().unwrap()
        .into_inner()
        .filter(|p| p.as_rule() == Rule::instr)
        .map(|instr| {
            let mut pairs = instr.into_inner();
            let operation = pairs.next().unwrap().as_str();
            
            Instr {
                operation,
                args: pairs.filter(|p| p.as_rule() == Rule::arg)
                    .map(|p| {
                        let arg = p.into_inner().next().unwrap();
                        match arg.as_rule() {
                            Rule::ident => Arg::Ident(arg.as_str()),
                            Rule::int_hex => Arg::Int(i64::from_str_radix(&arg.as_str()[2..], 16).unwrap()),
                            Rule::int_dec => Arg::Int(arg.as_str().parse().unwrap()),
                            Rule::text => Arg::Text(
                                arg.into_inner().next().unwrap().as_str()
                            ),

                            _ => unreachable!("invalid arg rule"),
                        }
                    }).collect(),
            }
        }).collect();

    Ok(Script {
        instrs,
    })
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

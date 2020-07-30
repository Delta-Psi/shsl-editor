#[derive(Debug)]
pub struct Error {
    input: String,
    expected: &'static str,
    position: usize,
}

impl Error {
    pub fn new(input: &str, expected: &'static str, position: usize) -> Self {
        Self {
            input: input.to_string(),
            expected,
            position,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // determine line and column
        let (line, line_start) = std::iter::once(0) // handle first line
            .chain(self.input.match_indices('\n')
                .map(|(i, _)| i+1))
            .enumerate()
            .find(|(_, i)| *i <= self.position)
            .unwrap();
        let column = self.position - line_start;

        let line_end = match self.input[line_start..]
            .match_indices('\n')
            .next()
        {
            Some((i, _)) => i,
            None => self.input.len(),
        };

        writeln!(f, "expected {} at {}:{}", self.expected, line+1, column+1)?;
        writeln!(f, "{}", &self.input[line_start..line_end])?;

        for _ in 0..column {
            write!(f, " ")?;
        }
        writeln!(f, "^")
    }
}

impl std::error::Error for Error {
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Token {
    pub begin: usize,
    pub end: usize,
}

impl Token {
    pub fn new(begin: usize, end: usize) -> Self {
        Self {
            begin,
            end,
        }
    }
}

#[derive(Debug)]
pub struct Ident<'a>(pub Token, pub &'a str);
#[derive(Debug)]
pub struct Int(pub Token, pub i32);
#[derive(Debug)]
pub struct Text<'a>(pub Token, pub &'a str);

#[derive(Debug)]
pub enum Arg<'a> {
    Ident(Ident<'a>),
    Int(Int),
    Text(Text<'a>),
}

impl<'a> Arg<'a> {
    pub fn as_ident(&self) -> Option<&'a str> {
        match self {
            Arg::Ident(ident) => Some(ident.1),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i32> {
        match self {
            Arg::Int(int) => Some(int.1),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&'a str> {
        match self {
            Arg::Text(text) => Some(text.1),
            _ => None,
        }
    }

    pub fn token(&self) -> &Token {
        match self {
            Arg::Ident(ident) => &ident.0,
            Arg::Int(int) => &int.0,
            Arg::Text(text) => &text.0,
        }
    }
}

#[derive(Debug)]
pub struct Instr<'a> {
    pub token: Token,
    pub operation: Ident<'a>,
    pub args: Vec<Arg<'a>>,
}

#[derive(Debug)]
pub struct Script<'a> {
    pub instrs: Vec<Instr<'a>>,
}

pub fn skip_whitespace(input: &str, pos: usize) -> usize {
    match input[pos..].char_indices().find(|(_, c)| *c != ' ' && *c != '\t') {
        Some((i, _)) => pos + i,
        None => input.len(),
    }
}

pub fn is_ident_head(c: char) -> bool {
    c == '_' || c.is_ascii_alphabetic()
}

pub fn is_ident_rest(c: char) -> bool {
    c == '_' || c.is_ascii_alphanumeric()
}

pub fn parse_ident(input: &str, pos: usize) -> Result<(Ident, usize)> {
    let begin = pos;
    let mut it = input[pos..].char_indices();

    match it.next() {
        None => return Err(Error::new(input, "identifier", pos)),
        Some((i, c)) => if !is_ident_head(c) {
            return Err(Error::new(input, "identifier", pos+i));
        },
    }

    let end = match it.find(|(_, c)| !is_ident_rest(*c)) {
        Some((i, _)) => begin+i,
        None => input.len(),
    };

    Ok((Ident(Token::new(begin, end), &input[begin..end]), end))
}

pub fn parse_int(input: &str, pos: usize) -> Result<(Int, usize)> {
    let begin = pos;

    let (negative, pos) = if input[pos..].starts_with('-') {
        (true, skip_whitespace(input, pos+1))
    } else {
        (false, pos)
    };

    if input[pos..].starts_with("0x") {
        // hexadecimal
        let pos = pos+2;
        let end = match input[pos..].char_indices()
            .find(|(_, c)| !c.is_ascii_hexdigit())
        {
            Some((i, _)) => pos+i,
            None => input.len(),
        };
        if end == pos {
            return Err(Error::new(input, "hex digit", end));
        }

        let mut value = i32::from_str_radix(&input[pos..end], 16).unwrap();
        if negative {
            value = -value;
        }

        Ok((Int(Token::new(begin, end), value), end))
    } else {
        // decimal
        let end = match input[pos..].char_indices()
            .find(|(_, c)| !c.is_ascii_digit())
        {
            Some((i, _)) => pos+i,
            None => input.len(),
        };
        if end == pos {
            return Err(Error::new(input, "digit", end));
        }

        let mut value = i32::from_str_radix(&input[pos..end], 10).unwrap();
        if negative {
            value = -value;
        }

        Ok((Int(Token::new(begin, end), value), end))
    }
}

pub fn parse_text(input: &str, pos: usize) -> Result<(Text, usize)> {
    let begin = pos;

    if !input[pos..].starts_with("`\n") {
        return Err(Error::new(input, "backquote+newline", pos));
    }
    let pos = pos+2;

    let mut end = None;
    let mut escaped = false;
    for (i, c) in input[pos..].char_indices() {
        if escaped {
            match c {
                '\\' => (),
                't' => (),
                '`' => (),

                _ => return Err(Error::new(input, "escape character", pos+i)),
            }

            escaped = false;
        } else {
            match c {
                '\\' => escaped = true,
                '`' => {
                    end = Some(pos+i+1);
                    break
                },

                _ => (),
            }
        }
    }

    if escaped {
        return Err(Error::new(input, "escape character", input.len()));
    }

    let end = match end {
        Some(end) => end,
        None => input.len(),
    };

    Ok((Text(Token::new(begin, end), &input[begin+2 .. end-1]), end))
}

pub fn parse_arg(input: &str, pos: usize) -> Result<(Arg, usize)> {
    if let Ok((ident, pos)) = parse_ident(input, pos) {
        Ok((Arg::Ident(ident), pos))
    } else if let Ok((int, pos)) = parse_int(input, pos) {
        Ok((Arg::Int(int), pos))
    } else if let Ok((text, pos)) = parse_text(input, pos) {
        Ok((Arg::Text(text), pos))
    } else {
        Err(Error::new(input, "identifier, integer or text", pos))
    }
}

pub fn parse_instr(input: &str, pos: usize) -> Result<(Instr, usize)> {
    let begin = pos;
    let (operation, pos) = parse_ident(input, pos)?;
    let pos = skip_whitespace(input, pos);
    
    let mut args = Vec::new();

    // first argument
    let pos = if let Ok((arg, pos)) = parse_arg(input, pos) {
        args.push(arg);
        let mut pos = skip_whitespace(input, pos);

        // other arguments
        while input[pos..].starts_with(',') {
            pos = skip_whitespace(input, pos+1);
            let (arg, new_pos) = parse_arg(input, pos)?;
            args.push(arg);

            pos = skip_whitespace(input, new_pos);
        }

        pos
    } else {
        pos
    };

    if !input[pos..].starts_with('\n') {
        Err(Error::new(input, "newline", pos))
    } else {
        Ok((Instr {
            token: Token::new(begin, pos+1),
            operation,
            args,
        }, pos+1))
    }
}

pub fn parse_script(input: &str) -> Result<Script> {
    let mut instrs = Vec::new();
    let mut pos = 0;
    while pos < input.len() {
        let (instr, new_pos) = parse_instr(input, pos)?;
        instrs.push(instr);
        pos = new_pos;
    }

    Ok(Script {
        instrs,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ident() {
        let right_input = &[
            "ident",
            "id_ent",
            "_idEnt",
            "_183423",
            "a123abb",
        ];

        for input in right_input {
            let (_, pos) = parse_ident(input, 0).unwrap();
            assert_eq!(pos, input.len());
        }

        let wrong_input = &[
            "0xxxx",
            "\twhitespace",
            "1234",
            "`",
        ];

        for input in wrong_input {
            assert!(parse_ident(input, 0).is_err());
        }
    }

    #[test]
    fn int() {
        let test_cases = &[
            ("0", 0),
            ("0x0", 0),
            ("100", 100),
            ("0x46f", 0x46f),
            ("-24", -24),
            ("- 0x44", -0x44),
        ];

        for (input, output) in test_cases {
            assert_eq!(
                parse_int(input, 0).unwrap().0 . 1,
                *output,
            );
        }
    }

    #[test]
    fn text() {
        let input = r#"`
this is some \`text\t
"#;
        let result = parse_text(input, 0).unwrap();

        assert_eq!(result.0 .0.begin, 0);
        assert_eq!(result.0 .0.end, input.len());
        assert_eq!(result.0 .1, &input[2..input.len()-1]);
        assert_eq!(result.1, input.len());
    }
}

use std::borrow::Cow;
use std::convert::TryInto;
use std::ops::Range;

type Result<T, E = ParseError> = std::result::Result<T, E>;

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct ParseError {
    pub pos: usize,
    pub reason: String,
}

pub struct ParsedQuery<'a> {
    pub(crate) query: &'a str,
    pub(crate) placeholders: Vec<Placeholder<'a>>,
}

#[derive(Clone, Debug)]
pub struct Placeholder<'a> {
    pub token: Range<usize>,
    pub ident: Ident<'a>,
    pub kleene: Option<Kleene>,
}

pub enum Ident<'a> {
    Positional(u16),
    Named(Cow<'a, str>),
}

impl Ident<'_> {
    fn into_static(self) -> Ident<'static> {
        match self {
            Self::Positional(pos) => Self::Positional(pos),
            Self::Named(named) => Self::Named(named.into_owned().into()),
        }
    }
}

pub enum Kleene {
    Question,
    Star,
    Plus,
}

struct ParseCtxt<'a> {
    query: &'a str,
    pos: usize,
    placeholders: Vec<Placeholder<'a>>,
}

impl<'a> ParseCtxt<'a> {
    fn new(query: &'a str) -> Self {
        ParseCtxt { query, pos: 0, placeholders: vec![] }
    }

    fn run(mut self) -> Result<Vec<Placeholder<'a>>> {
        while self.pos < self.query.len() {
            if let Some(i) = self.query[self.pos..].find(&['{', '\'', '"', '`'][..]) {
                self.pos += i;

                if self.query[self.pos] == '{' {
                    self.placeholders.push(self.parse_placeholder()?);
                } else {
                    self.consume_string()?;
                }
            } else {
                break;
            }
        }

        Ok(self.placeholders)
    }

    /// Find the paired quote for the character at `self.query[self.pos]` and skip past it
    fn consume_string(&mut self) -> Result<()> {
        let start = self.pos;
        let delim = self.query[self.pos];

        while let Some(i) = self.query[self.pos + 1..].find(delim) {
            // the current position in `self.query`
            self.pos = self.pos + i + 1;

            if self.query[self.pos - 1] == '\\' {
                self.pos += 1;
                continue;
            }
        }

        Err(ParseError {
            pos: start,
            reason: format!("unpaired delimiter: '{}'", self.query[start]),
        })
    }

    fn parse_placeholder(&mut self) -> Result<Placeholder<'a>> {
        let start = self.pos;
        self.pos += 1;

        let (next_idx, next) =
            self.query[self.pos..].trim_start().char_indices().next().ok_or_else(|| {
                ParseError { pos: start, reason: "unpaired delimiter: '{'".into() }
            })?;

        self.pos += next_idx;

        let ident = match next {
            '}' => {
                let idx: u16 = (self.placeholders.len() + 1).try_into().map_err(|_| {
                    ParseError { pos: start, reason: "placeholder limit exceeded".into() }
                })?;

                // advance beyond the placeholder
                self.pos += 1;

                return Ok(Placeholder {
                    token: start..self.pos,
                    ident: Ident::Positional(idx),
                    kleene: None,
                });
            }
            _ if next.is_digit(10) => Ident::Positional(self.parse_positional_index()?),
            _ if next == '_' || next.is_alphabetic() => {
                Ident::Named(self.parse_named_ident()?.into())
            }
            _ => {
                return Err(ParseError {
                    pos: self.pos,
                    reason: format!("unexpected character: '{}'", next),
                })
            }
        };
    }

    fn parse_positional_index(&mut self) -> Result<u16> {
        let start = self.pos;
        if let Some(end) = self.query[self.pos].find(|c: char| !c.is_digit(10)) {
            self.pos += end;
            self.query[start..self.pos]
                .parse::<u16>()
                .map_err(|e| ParseError { pos: start, reason: e.to_string() })
        } else {
            Err(ParseError { pos: start, reason: "unexpected end of query string".into() })
        }
    }

    fn parse_named_ident(&mut self) -> Result<&'a str> {}
}

pub fn parse_query(query: &str) -> Result<ParsedQuery> {
    let placeholders = ParseCtxt::new(query).run()?;

    Ok(ParsedQuery { query, placeholders })
}

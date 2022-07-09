use std::str::Chars;

pub enum KeywordKind {
    Fn,
    If,
}

#[derive(Debug)]
pub enum TokenKind {
    // Multi Char
    Ident,
    // Multi Char
    Literal,
    // Multi Char, beginning with #
    Comment,
    // Multi Char
    Keyword,

    // +, -, *, /
    Operator,
    // " "
    Whitespace,
    // (
    OpenParen,
    // )
    CloseParen,
    // {
    OpenBrace,
    // }
    CloseBrace,

    Unknown,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    len: usize,
}

impl Token {
    fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len }
    }

    fn is_keyword(kw: &str) -> bool {
        let keywords = ["fn", "if"];

        keywords.contains(&kw)
    }

    fn str_to_keyword(kw: &str) -> Option<KeywordKind> {
        match kw {
            "fn" => Some(KeywordKind::Fn),
            "if" => Some(KeywordKind::If),

            _ => None,
        }
    }
}

pub struct Cursor<'a> {
    input: Chars<'a>,
    initial_len: usize,
}

impl<'a> Cursor<'a> {
    fn new(input: Chars<'a>) -> Self {
        Self {
            input: input.clone(),
            initial_len: input.as_str().len(),
        }
    }

    fn is_eof(&self) -> bool {
        self.input.as_str().is_empty()
    }

    fn bump(&mut self) -> Option<char> {
        self.input.next()
    }

    fn consume_until_inner<F>(&mut self, acc: String, f: F) -> String
    where
        F: Fn(char) -> bool,
    {
        self.peek()
            .map(|c| (f(c) || self.is_eof(), c))
            .map(|(r, c)| match r {
                true => acc,
                false => {
                    self.bump();
                    self.consume_until_inner(format!("{}{}", acc, c), f)
                }
            })
            .unwrap()
    }

    fn consume_until<F>(&mut self, c: char, f: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let acc = c.to_string();
        self.consume_until_inner(acc, f)
    }

    fn len_consumed(&self) -> usize {
        self.initial_len - self.input.as_str().len()
    }
    fn reset_len_consumed(&mut self) {
        self.initial_len = self.input.as_str().len()
    }

    fn advance(&mut self) -> Option<Token> {
        self.bump()
            .map(|c| match c {
                x if x.is_whitespace() => TokenKind::Whitespace,
                '#' => self.comment(c),
                '(' => TokenKind::OpenParen,
                ')' => TokenKind::CloseParen,
                '{' => TokenKind::OpenBrace,
                '}' => TokenKind::CloseBrace,

                first_char if valid_multi_char_start(first_char) => match self.mutli_char(c) {
                    kw if Token::is_keyword(&kw) => TokenKind::Keyword,
                    _ => TokenKind::Ident,
                },

                // If it doesn't have a valid multi_char_start, then it's likely a literal.
                first_char if valid_literal_start(first_char) => {
                    let _ = self.literal(c);
                    TokenKind::Literal
                }

                _ => TokenKind::Unknown,
            })
            .map(|r| Token::new(r, self.len_consumed()))
    }

    fn peek(&self) -> Option<char> {
        self.input.clone().next()
    }

    fn comment(&mut self, start: char) -> TokenKind {
        self.consume_until(start, eol);

        TokenKind::Comment
    }

    fn mutli_char(&mut self, start: char) -> String {
        self.consume_until(start, |c| !valid_multi_char(c) || eol(c))
    }

    fn literal(&mut self, start: char) -> String {
        self.consume_until(start, |c| c.is_whitespace() || c == '\"' || eol(c))
    }

    pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
        let mut cursor = Cursor::new(input.chars());
        std::iter::from_fn(move || {
            if cursor.is_eof() {
                None
            } else {
                cursor.reset_len_consumed();
                cursor.advance()
            }
        })
    }
}

fn valid_literal_start(c: char) -> bool {
    c.is_numeric() || c == '\"'
}

fn valid_multi_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn valid_multi_char_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

pub fn eol(c: char) -> bool {
    c == '\n'
}

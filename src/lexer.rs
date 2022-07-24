use std::str::Chars;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeywordKind {
    Fn,
    If,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Multi Char
    Ident(String),
    // Multi Char
    Literal(String),
    // Multi Char, beginning with #
    Comment(String),
    // Multi Char
    Keyword(KeywordKind),

    // +, -, *, /
    Operator(char),
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
    // =
    Equals,
    // ,
    Comma,

    Unknown,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {}

impl Token {
    fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len }
    }

    fn is_keyword(kw: &str) -> bool {
        Self::str_to_keyword(kw).is_some()
    }

    fn str_to_keyword(kw: &str) -> Option<KeywordKind> {
        match kw {
            "fn" => Some(KeywordKind::Fn),
            "if" => Some(KeywordKind::If),

            _ => None,
        }
    }

    /// Returns an owned string of the inner.
    pub fn inner_string(&self) -> Option<String> {
        match &self.kind {
            TokenKind::Ident(i) => Some(i.to_string()),
            TokenKind::Literal(i) => Some(i.to_string()),
            TokenKind::Comment(i) => Some(i.to_string()),

            _ => None,
        }
    }

    pub fn inner_int(&self) -> Option<usize> {
        match &self.kind {
            TokenKind::Ident(i) => Some(i.parse::<usize>().ok()?),

            _ => None,
        }
    }

    pub fn inner_keyword(&self) -> Option<KeywordKind> {
        match self.kind {
            TokenKind::Keyword(i) => Some(i),
            _ => None,
        }
    }

    pub fn inner_operator(&self) -> Option<char> {
        match self.kind {
            TokenKind::Operator(i) => Some(i),
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

                // Operators
                '+' | '-' | '*' | '/' => TokenKind::Operator(c),

                '#' => self.comment(c),
                '(' => TokenKind::OpenParen,
                ')' => TokenKind::CloseParen,
                '{' => TokenKind::OpenBrace,
                '}' => TokenKind::CloseBrace,
                '=' => TokenKind::Equals,
                ',' => TokenKind::Comma,

                first_char if valid_multi_char_start(first_char) => match self.mutli_char(c) {
                    kw if Token::is_keyword(&kw) => {
                        TokenKind::Keyword(Token::str_to_keyword(&kw).unwrap())
                    }
                    kw => TokenKind::Ident(kw),
                },

                // If it doesn't have a valid multi_char_start, then it's likely a literal.
                first_char if valid_literal_start(first_char) => {
                    let lit = self.literal(c);
                    TokenKind::Literal(lit)
                }

                _ => TokenKind::Unknown,
            })
            .map(|r| Token::new(r, self.len_consumed()))
    }

    fn peek(&self) -> Option<char> {
        self.input.clone().next()
    }

    fn comment(&mut self, start: char) -> TokenKind {
        let com = self.consume_until(start, eol);
        TokenKind::Comment(com)
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

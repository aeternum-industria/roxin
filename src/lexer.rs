use crate::token::{LexError, Span, Token, TokenKind};

fn keyword(word: &str) -> Option<TokenKind> {
    use TokenKind::*;

    Some(match word {
        "allowed" => Allowed,
        "and" => And,
        "assume" => Assume,
        "bool" => TyBool,
        "data" => Data,
        "decimal" => TyDecimal,
        "else" => Else,
        "extern" => Extern,
        "false" => False,
        "float" => TyFloat,
        "for" => For,
        "from" => From,
        "if" => If,
        "in" => In,
        "int" => TyInt,
        "input" => Input,
        "invariant" => Invariant,
        "lemma" => Lemma,
        "mechanism" => Mechanism,
        "metric" => Metric,
        "not" => Not,
        "or" => Or,
        "param" => Param,
        "run" => Run,
        "state" => State,
        "step" => Step,
        "steps" => Steps,
        "sweep" => Sweep,
        "system" => System,
        "then" => Then,
        "tolerance" => Tolerance,
        "true" => True,
        "type" => Type,
        "unique" => Unique,
        "until" => Until,
        "within" => Within,
        _ => return None,
    })
}

pub struct Lexer<'src> {
    src: &'src str,
    bytes: &'src [u8],
    pos: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            bytes: src.as_bytes(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn peek_at(&self, n: usize) -> Option<u8> {
        self.bytes.get(self.pos + n).copied()
    }

    fn peek_char(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn skip_trivia(&mut self) -> bool {
        let before = self.pos;
        loop {
            match self.peek() {
                Some(b' ' | b'\t' | b'\r') => self.pos += 1,
                Some(b'-') if self.peek_at(1) == Some(b'-') => {
                    while !matches!(self.peek(), None | Some(b'\n')) {
                        self.pos += 1;
                    }
                }

                _ => break,
            }
        }

        self.pos > before
    }

    fn identifier_or_keyword(&mut self, start: usize) -> TokenKind {
        while matches!(
            self.peek(),
            Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_')
        ) {
            self.pos += 1;
        }

        if let Some(ch) = self.peek_char()
            && !ch.is_ascii() && ch.is_alphanumeric() {
                self.eat_word();
                return TokenKind::Error(LexError::NonAsciiIdentifier);
            }

        keyword(&self.src[start..self.pos]).unwrap_or(TokenKind::Identifier)
    }

    fn eat_word(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch == '_' || ch.is_alphanumeric() {
                self.pos += ch.len_utf8();
            } else {
                break;
            }
        }
    }

    fn eat_digits(&mut self) {
        loop {
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }

            if self.peek() == Some(b'_') && matches!(self.peek_at(1), Some(b'0'..=b'9')) {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn number(&mut self) -> TokenKind {
        self.eat_digits();
        if self.peek() == Some(b'.') && matches!(self.peek_at(1), Some(b'0'..=b'9')) {
            self.pos += 1;
            self.eat_digits();
        }

        if matches!(self.peek(), Some(b'e' | b'E')) {
            let sign = matches!(self.peek_at(1), Some(b'+' | b'-')) as usize;
            if matches!(self.peek_at(1 + sign), Some(b'0'..=b'9')) {
                self.pos += 1 + sign;
                self.eat_digits();
            }
        }

        if matches!(self.peek(), Some(b'a'..=b'z' | b'A'..=b'Z' | b'_')) {
            let error = if self.peek() == Some(b'_') {
                LexError::MisplacedUnderscore
            } else {
                LexError::NumberGluedToLetters
            };

            while matches!(
                self.peek(),
                Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_')
            ) {
                self.pos += 1;
            }

            return TokenKind::Error(error);
        }

        TokenKind::Number
    }

    fn string(&mut self) -> TokenKind {
        self.pos += 1; // I keep forgetting why we do this but its for the opening quote

        let mut invalid_escape = false;

        loop {
            match self.peek() {
                None | Some(b'\n') => return TokenKind::Error(LexError::UnterminatedString),
                Some(b'"') => {
                    self.pos += 1;
                    return if invalid_escape {
                        TokenKind::Error(LexError::InvalidEscape)
                    } else {
                        TokenKind::Str
                    };
                }
                Some(b'\\') => {
                    self.pos += 1;
                    match self.peek() {
                        Some(b'"' | b'\\' | b'n' | b't') => self.pos += 1,
                        None | Some(b'\n') => {}
                        Some(_) => {
                            invalid_escape = true;
                            self.pos += 1;
                        }
                    }
                }
                Some(_) => self.pos += 1,
            }
        }
    }

    fn operator(&mut self) -> TokenKind {
        use TokenKind::*;

        let b = self.bytes[self.pos];
        let next = self.peek_at(1);
        let (kind, len) = match (b, next) {
            (b'=', Some(b'=')) => (EqEq, 2),
            (b'!', Some(b'=')) => (BangEq, 2),
            (b'<', Some(b'=')) => (LtEq, 2),
            (b'>', Some(b'=')) => (GtEq, 2),
            (b'>', Some(b'>')) => (Compose, 2),
            (b'-', Some(b'>')) => (Arrow, 2),
            (b'/', Some(b'/')) => (SlashSlash, 2),
            (b'.', Some(b'.')) => (DotDot, 2),
            (b'+', _) => (Plus, 1),
            (b'-', _) => (Minus, 1),
            (b'*', _) => (Star, 1),
            (b'/', _) => (Slash, 1),
            (b'%', _) => (Percent, 1),
            (b'^', _) => (Caret, 1),
            (b'<', _) => (Lt, 1),
            (b'>', _) => (Gt, 1),
            (b'=', _) => (Eq, 1),
            (b':', _) => (Colon, 1),
            (b',', _) => (Comma, 1),
            (b'.', _) => (Dot, 1),
            (b'|', _) => (Pipe, 1),
            (b'(', _) => (LParen, 1),
            (b')', _) => (RParen, 1),
            (b'[', _) => (LBracket, 1),
            (b']', _) => (RBracket, 1),
            (b'{', _) => (LBrace, 1),
            (b'}', _) => (RBrace, 1),
            _ => {
                let ch = self.peek_char().unwrap();
                if !ch.is_ascii() && ch.is_alphanumeric() {
                    self.eat_word();
                    return Error(LexError::NonAsciiIdentifier);
                }
                self.pos += ch.len_utf8();
                return Error(LexError::UnexpectedChar);
            }
        };
        self.pos += len;
        kind
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let space_before = self.skip_trivia();
        let start = self.pos;
        let b = self.peek()?;
        let kind = match b {
            b'\n' => {
                self.pos += 1;
                TokenKind::NewLine
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.identifier_or_keyword(start),
            b'0'..=b'9' => self.number(),
            b'"' => self.string(),
            _ => self.operator(),
        };

        Some(Token {
            kind,
            span: Span {
                start,
                end: self.pos,
            },
            space_before,
        })
    }
}

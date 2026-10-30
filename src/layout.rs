use std::collections::VecDeque;

use crate::lexer::Lexer;
use crate::token::{LexError, Span, Token, TokenKind};

pub struct Layout<'src> {
    src: &'src str,
    inner: Lexer<'src>,
    depth: usize,
    brackets: usize,
    at_line_start: bool,
    pending: VecDeque<Token>,
    finished: bool,
}

impl<'src> Layout<'src> {
    pub fn new(src: &'src str) -> Self {
        Layout {
            src,
            inner: Lexer::new(src),
            depth: 0,
            brackets: 0,
            at_line_start: true,
            pending: VecDeque::new(),
            finished: false,
        }
    }

    fn synthetic(kind: TokenKind, at: usize) -> Token {
        Token {
            kind,
            span: Span { start: at, end: at },
            space_before: false,
        }
    }

    fn error_token(error: LexError, start: usize, end: usize) -> Token {
        Token {
            kind: TokenKind::Error(error),
            span: Span { start, end },
            space_before: false,
        }
    }

    fn handle_indentation(&mut self, first: &Token) {
        let at = first.span.start;
        let line_start = self.src[..at].rfind('\n').map_or(0, |i| i + 1);
        let leading = &self.src[line_start..at];

        if leading.contains('\t') {
            self.pending
                .push_back(Self::error_token(LexError::TabInIndent, line_start, at));
            return;
        }

        let spaces = leading.len();
        let mut target = spaces / 4; // TODO: Maybe allow 2/4 space indents both? TBD.

        if !spaces.is_multiple_of(4) {
            self.pending
                .push_back(Self::error_token(LexError::BadIndent, line_start, at));
        }

        if target > self.depth + 1 {
            self.pending
                .push_back(Self::error_token(LexError::IndentTooDeep, line_start, at));
            target = self.depth + 1;
        }

        while self.depth < target {
            self.depth += 1;
            self.pending
                .push_back(Self::synthetic(TokenKind::Indent, at));
        }

        while self.depth > target {
            self.depth -= 1;
            self.pending
                .push_back(Self::synthetic(TokenKind::Dedent, at));
        }
    }
}

impl<'src> Iterator for Layout<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            if let Some(token) = self.pending.pop_front() {
                return Some(token);
            }

            if self.finished {
                return None;
            }

            let Some(token) = self.inner.next() else {
                self.finished = true;
                let end = self.src.len();

                if !self.at_line_start {
                    self.pending
                        .push_back(Self::synthetic(TokenKind::NewLine, end));
                }

                for _ in 0..self.depth {
                    self.pending
                        .push_back(Self::synthetic(TokenKind::Dedent, end));
                }

                self.depth = 0;
                continue;
            };

            if token.kind == TokenKind::NewLine {
                if self.brackets > 0 || self.at_line_start {
                    continue;
                }

                self.at_line_start = true;
                return Some(token);
            }

            if self.at_line_start && self.brackets == 0 {
                self.handle_indentation(&token);
            }

            self.at_line_start = false;

            match token.kind {
                TokenKind::LParen | TokenKind::LBrace | TokenKind::LBracket => self.brackets += 1,
                TokenKind::RParen | TokenKind::RBrace | TokenKind::RBracket => {
                    self.brackets = self.brackets.saturating_sub(1)
                }
                _ => {}
            }

            self.pending.push_back(token);
        }
    }
}

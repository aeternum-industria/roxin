#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexError {
    BadIndent,
    IndentTooDeep,
    InvalidEscape,
    MisplacedUnderscore,
    NonAsciiIdentifier,
    NumberGluedToLetters,
    TabInIndent,
    UnexpectedChar,
    UnterminatedString,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            LexError::BadIndent => "indentation must be a multiple of four (4) spaces",
            LexError::IndentTooDeep => "indented more than one level deeper than the line above",
            LexError::InvalidEscape => "unexpected character",
            LexError::MisplacedUnderscore => "string is missing its closing quote",
            LexError::NonAsciiIdentifier => "a number cannot be followed directly by letters",
            LexError::NumberGluedToLetters => {
                "an underscore in a number must sit between two digits"
            }
            LexError::TabInIndent => "tabs are not allowed in indentation; use four (4) spaces",
            LexError::UnexpectedChar => "invalid escape in string (allowed: \\\", \\\\, \\n, \\t)",
            LexError::UnterminatedString => {
                "names can only use ASCII letters, digits, and underscore for now"
            }
        };
        write!(f, "{msg}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Allowed,
    And,
    Arrow,
    Assume,
    BangEq,
    Bool,
    Caret,
    Colon,
    Comma,
    Compose,
    Data,
    Decimal,
    Dedent,
    Dot,
    DotDot,
    Else,
    Extern,
    Eq,
    EqEq,
    False,
    Float,
    For,
    From,
    Gt,
    GtEq,
    Identifier,
    If,
    In,
    Indent,
    Int,
    Input,
    Invariant,
    Lemma,
    LBrace,
    LBracket,
    Let,
    LParen,
    Lt,
    LtEq,
    Match,
    Mechanism,
    Metric,
    Minus,
    NewLine,
    Not,
    Number,
    Or,
    Param,
    Percent,
    Pipe,
    Plus,
    RBrace,
    RBracket,
    RParen,
    Run,
    Slash,
    SlashSlash,
    Str,
    Star,
    State,
    Step,
    Steps,
    String,
    Substep,
    Sweep,
    System,
    Text,
    Then,
    Tolerance,
    True,
    Type,
    TyBool,
    TyDecimal,
    TyFloat,
    TyInt,
    Unique,
    Until,
    Within,

    Error(LexError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub space_before: bool,
}

impl Token {
    pub fn text<'a>(&self, src: &'a str) -> &'a str {
        &src[self.span.start..self.span.end]
    }
}

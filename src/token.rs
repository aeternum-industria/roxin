#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexError {
    InvalidEscape,
    MisplacedUnderscore,
    NonAsciiIdentifier,
    NumberGluedToLetters,
    UnexpectedChar,
    UnterminatedString,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            LexError::InvalidEscape => "unexpected character",
            LexError::MisplacedUnderscore => "string is missing its closing quote",
            LexError::NonAsciiIdentifier => "a number cannot be followed directly by letters",
            LexError::NumberGluedToLetters => {
                "an underscore in a number must sit between two digits"
            }
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
    Dot,
    DotDot,
    Else,
    Every,
    Export,
    Extern,
    Eq,
    EqEq,
    False,
    Float,
    For,
    From,
    Gt,
    GtEq,
    Ident,
    If,
    In,
    Int,
    Input,
    Invariant,
    Keep,
    Lemma,
    LBrace,
    LBracket,
    Let,
    LParen,
    Lt,
    LtEq,
    Map,
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
    Reach,
    Record,
    RParen,
    Run,
    Running,
    Runs,
    Seed,
    Slash,
    SlashSlash,
    Str,
    Star,
    State,
    Step,
    Steps,
    String,
    Substep,
    Substeps,
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
    Verbose,
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

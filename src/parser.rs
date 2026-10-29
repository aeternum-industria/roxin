use crate::ast::{Expr, Field, Name, ParamDecl, StateDecl, TypeExpr};
use crate::layout::Layout;
use crate::token::{Span, Token, TokenKind};

pub struct ParseError {
    pub message: String,
    pub span: Span,
}

pub struct Parser<'src> {
    pos: usize,
    src: &'src str,
    tokens: Vec<Token>,
}

impl<'src> Parser<'src> {
    pub fn new(src: &'src str) -> Self {
        Parser {
            src,
            tokens: Layout::new(src).collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<TokenKind> {
        self.tokens.get(self.pos).map(|t| t.kind)
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos).copied()?;
        self.pos += 1;
        Some(token)
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.peek() == Some(kind)
    }

    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn text(&self, token: Token) -> &'src str {
        &self.src[token.span.start..token.span.end]
    }

    fn describe_current(&self) -> String {
        match self.tokens.get(self.pos) {
            None => "end of input".to_string(),
            Some(token) => match token.kind {
                TokenKind::NewLine => "end of line".to_string(),
                TokenKind::Indent => "an indent".to_string(),
                TokenKind::Dedent => "a dedent".to_string(),
                TokenKind::Error(e) => format!("a bad token ({e})"),
                _ => format!("`{}`", self.text(*token)),
            },
        }
    }

    fn current_span(&self) -> Span {
        match self.tokens.get(self.pos) {
            Some(token) => token.span,
            None => Span {
                start: self.src.len(),
                end: self.src.len(),
            },
        }
    }

    fn error_here(&self, message: String) -> ParseError {
        ParseError {
            message,
            span: self.current_span(),
        }
    }

    fn expect(&mut self, kind: TokenKind, what: &str) -> Result<Token, ParseError> {
        let token = self.tokens.get(self.pos).copied();
        match token {
            Some(t) if t.kind == kind => {
                self.pos += 1;
                Ok(t)
            }
            _ => Err(self.error_here(format!(
                "expected {what}, found {}",
                self.describe_current()
            ))),
        }
    }

    /// EXPR -> Number (for the moment)
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let token = self.expect(TokenKind::Number, "a number")?;
        Ok(Expr::Number {
            span: token.span,
            text: self.text(token).to_string(),
        })
    }

    /// FIELD -> NAME ":" TYPE
    fn parse_field(&mut self) -> Result<Field, ParseError> {
        let name = self.parse_name("a field name")?;
        self.expect(TokenKind::Colon, "`:` after the field name")?;
        let ty = self.parse_type()?;
        Ok(Field { name, ty })
    }

    /// NAME -> Identifier
    fn parse_name(&mut self, what: &str) -> Result<Name, ParseError> {
        let token = self.expect(TokenKind::Identifier, what)?;
        Ok(Name {
            span: token.span,
            text: self.text(token).to_string(),
        })
    }

    /// PARAM -> "param" NAME ":" TYPE "=" EXPR NEWLINE
    fn parse_param(&mut self) -> Result<ParamDecl, ParseError> {
        self.expect(TokenKind::Param, "`param`")?;
        let name = self.parse_name("a parameter name")?;
        self.expect(TokenKind::Colon, "`:` after the parameter name")?;
        let ty = self.parse_type()?;
        self.expect(TokenKind::Eq, "`=` followed by a default value")?;
        let default = self.parse_expr()?;
        self.expect(TokenKind::NewLine, "end of line")?;
        Ok(ParamDecl { name, ty, default })
    }

    /// STATE -> "state" NAME "{" [ FIELD ("," FIELD)* [","] ] "}" NEWLINE (This one is complicated)
    fn parse_state(&mut self) -> Result<StateDecl, ParseError> {
        let mut fields = Vec::new();

        self.expect(TokenKind::State, "`state`")?;
        let name = self.parse_name("a state name")?;
        self.expect(TokenKind::LBrace, "`{` after the state name")?;

        while !self.at(TokenKind::RBrace) {
            fields.push(self.parse_field()?);
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }

        self.expect(TokenKind::RBrace, "`,` or `}` after a field")?;
        self.expect(TokenKind::NewLine, "end of line")?;
        Ok(StateDecl { name, fields })
    }

    /// TYPE -> "bool" | "decimal" | "float" | "int" | NAME
    fn parse_type(&mut self) -> Result<TypeExpr, ParseError> {
        match self.peek() {
            Some(TokenKind::TyInt) => {
                self.advance();
                Ok(TypeExpr::Int)
            }
            Some(TokenKind::TyFloat) => {
                self.advance();
                Ok(TypeExpr::Float)
            }
            Some(TokenKind::TyDecimal) => {
                self.advance();
                Ok(TypeExpr::Decimal)
            }
            Some(TokenKind::TyBool) => {
                self.advance();
                Ok(TypeExpr::Bool)
            }
            Some(TokenKind::Identifier) => Ok(TypeExpr::Named(self.parse_name("a type")?)),
            _ => Err(self.error_here(format!(
                "expected a type, found {}",
                self.describe_current()
            ))),
        }
    }
}

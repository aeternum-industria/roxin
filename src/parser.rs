use crate::ast::{
    BinOp, Expr, Field, FieldInit, InputField, Item, MechanismField, Module, Name, ParamDecl,
    RecordLit, RunClause, RunDecl, StateDecl, StepDecl, SweepClause, SystemDecl, TypeExpr, UnOp,
};
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

    fn expect_block_open(&mut self) -> Result<(), ParseError> {
        self.expect(TokenKind::Colon, "`:`")?;
        self.expect(TokenKind::NewLine, "end of line after `:`")?;
        self.expect(TokenKind::Indent, "an indented block")?;
        Ok(())
    }

    /// INPUT -> "input" " ":" NEWLINE INDENT INPUTFIELD+ DEDENT
    fn parse_input(&mut self) -> Result<Vec<InputField>, ParseError> {
        self.expect(TokenKind::Input, "`input`")?;
        self.expect_block_open()?;

        let mut fields = Vec::new();
        while !self.at(TokenKind::Dedent) {
            fields.push(self.parse_input_field()?);
        }

        self.expect(TokenKind::Dedent, "the end of the block")?;
        Ok(fields)
    }

    /// INPUTFIELD -> NAME ":" TYPE "=" EXPR NEWLINE
    fn parse_input_field(&mut self) -> Result<InputField, ParseError> {
        let name = self.parse_name("an input field name")?;
        self.expect(TokenKind::Colon, "`:` after the field name")?;
        let ty = self.parse_type()?;
        self.expect(TokenKind::Eq, "`=` followed by the inputs expression")?;
        let value = self.parse_expr()?;
        self.expect(TokenKind::NewLine, "end of line")?;
        Ok(InputField { name, ty, value })
    }

    /// MECHANISM -> "mechanism" ":" NEWLINE INDENT MECHANISMFIELD+ DEDENT
    fn parse_mechanism(&mut self) -> Result<Vec<MechanismField>, ParseError> {
        self.expect(TokenKind::Mechanism, "`mechanism`")?;
        self.expect_block_open()?;

        let mut fields = Vec::new();
        while !self.at(TokenKind::Dedent) {
            fields.push(self.parse_mechanism_field()?);
        }

        self.expect(TokenKind::Dedent, "the end of the block")?;
        Ok(fields)
    }

    /// MECHANISMFIELD -> NAME ":" EXPR NEWLINE
    fn parse_mechanism_field(&mut self) -> Result<MechanismField, ParseError> {
        let name = self.parse_name("a field name")?;
        self.expect(TokenKind::Colon, "`:` after the field name")?;
        let value = self.parse_expr()?;
        self.expect(TokenKind::NewLine, "end of line")?;
        Ok(MechanismField { name, value })
    }

    // STEP -> "step" NAME ":" NEWLINE INDENT [INPUT] MECHANISM DEDENT
    fn parse_step(&mut self) -> Result<StepDecl, ParseError> {
        self.expect(TokenKind::Step, "`step`")?;
        let name = self.parse_name("a step name")?;
        self.expect_block_open()?;

        let inputs = if self.at(TokenKind::Input) {
            self.parse_input()?
        } else {
            Vec::new()
        };

        if !self.at(TokenKind::Mechanism) {
            return Err(self.error_here(format!(
                "expected the `mechanism:` section, found {}",
                self.describe_current()
            )));
        }

        let mechanism = self.parse_mechanism()?;
        self.expect(TokenKind::Dedent, "the end of the step")?;
        Ok(StepDecl {
            name,
            inputs,
            allowed: None,
            mechanism,
        })
    }

    /// FIELDINIT -> NAME ":" EXPR
    fn parse_field_init(&mut self) -> Result<FieldInit, ParseError> {
        let name = self.parse_name("a field name")?;
        self.expect(TokenKind::Colon, "`:` after the field name")?;
        let value = self.parse_expr()?;
        Ok(FieldInit { name, value })
    }

    /// RECORD -> NAME "{" [ FIELDINIT ("," FIELDINIT)* [","] ] "}"
    fn parse_record_lit(&mut self) -> Result<RecordLit, ParseError> {
        let ty = self.parse_name("a state type name")?;
        self.expect(TokenKind::LBrace, "`{` after the type name")?;
        let mut fields = Vec::new();

        while !self.at(TokenKind::RBrace) {
            fields.push(self.parse_field_init()?);
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }

        self.expect(TokenKind::RBrace, "`,` or `}` after a field")?;
        Ok(RecordLit { ty, fields })
    }

    /// SWEEP -> "sweep" NAME "in" "[" EXPR ("," EXPR)* [","] "]" NEWLINE
    fn parse_sweep(&mut self) -> Result<SweepClause, ParseError> {
        self.expect(TokenKind::Sweep, "`sweep`")?;
        let param = self.parse_name("a parameter name")?;
        self.expect(TokenKind::In, "`in` after the parameter name")?;
        self.expect(TokenKind::LBracket, "`[` to start the list of values")?;
        let mut values = vec![self.parse_expr()?];

        while self.eat(TokenKind::Comma) {
            if self.at(TokenKind::RBracket) {
                break;
            }

            values.push(self.parse_expr()?);
        }

        self.expect(TokenKind::RBracket, "`,` or `]` after a value")?;
        self.expect(TokenKind::NewLine, "end of line")?;
        Ok(SweepClause { param, values })
    }

    /// CLAUSE -> SWEEP (more to come: until, runs, seed, etc)
    fn parse_run_clause(&mut self) -> Result<RunClause, ParseError> {
        match self.peek() {
            Some(TokenKind::Sweep) => Ok(RunClause::Sweep(self.parse_sweep()?)),
            _ => Err(self.error_here(format!(
                "expected a run clause such as `sweep`, found {}",
                self.describe_current()
            ))),
        }
    }

    /// RUN -> "run" NAME "from" RECORD "for" EXPR "steps" NEWLINE [ INDENT CLAUSE+ DEDENT ]
    fn parse_run(&mut self) -> Result<RunDecl, ParseError> {
        self.expect(TokenKind::Run, "`run`")?;
        let system = self.parse_name("a system name")?;
        self.expect(TokenKind::From, "`from` followed by the initial state")?;
        let initial = self.parse_record_lit()?;
        self.expect(TokenKind::For, "`for` followed by the step count")?;
        let steps = self.parse_expr()?;
        self.expect(TokenKind::Steps, "`steps` after the step count")?;
        self.expect(TokenKind::NewLine, "end of line")?;
        let mut clauses = Vec::new();

        if self.eat(TokenKind::Indent) {
            while !self.at(TokenKind::Dedent) {
                clauses.push(self.parse_run_clause()?);
            }

            self.expect(TokenKind::Dedent, "the end of the runs clauses")?;
        }

        Ok(RunDecl {
            system,
            initial,
            steps,
            clauses,
        })
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

    /// EXPR -> any expression (see `parse_expr_bp`)
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_expr_bp(0)
    }

    fn parse_expr_bp(&mut self, min_bp: u8) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_expr_prefix()?;
        loop {
            if self.at(TokenKind::Dot) {
                if POSTFIX_BP < min_bp {
                    break;
                }

                self.advance();
                let field = self.parse_name("a field name after `.`")?;
                lhs = Expr::FieldAccess {
                    base: Box::new(lhs),
                    field,
                };
                continue;
            }

            let Some(kind) = self.peek() else { break };
            let Some((op, left_bp, right_bp)) = binary_op(kind) else {
                break;
            };
            if left_bp < min_bp {
                break;
            }

            self.advance();

            let rhs = self.parse_expr_bp(right_bp)?;
            lhs = Expr::Binary {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_expr_prefix(&mut self) -> Result<Expr, ParseError> {
        match self.peek() {
            Some(TokenKind::Number) => {
                let token = self.expect(TokenKind::Number, "a number")?;
                Ok(Expr::Number {
                    span: token.span,
                    text: self.text(token).to_string(),
                })
            }
            Some(TokenKind::Identifier) => Ok(Expr::Name(self.parse_name("a name")?)),
            Some(TokenKind::LParen) => {
                self.advance();
                let inner = self.parse_expr_bp(0)?;
                self.expect(TokenKind::RParen, "`)` to close the parenthesis")?;
                Ok(inner)
            }
            Some(TokenKind::Minus) => {
                let op_token = self.expect(TokenKind::Minus, "`-`")?;
                let operand = self.parse_expr_bp(PREFIX_BP)?;
                Ok(Expr::Unary {
                    op: UnOp::Neg,
                    op_span: op_token.span,
                    operand: Box::new(operand),
                })
            }
            _ => Err(self.error_here(format!(
                "expected an expression, found {}",
                self.describe_current()
            ))),
        }
    }

    /// FIELD -> NAME ":" TYPE
    fn parse_field(&mut self) -> Result<Field, ParseError> {
        let name = self.parse_name("a field name")?;
        self.expect(TokenKind::Colon, "`:` after the field name")?;
        let ty = self.parse_type()?;
        Ok(Field { name, ty })
    }

    /// MODULE -> ITEM*  ITEM -> PARAM | STATE | SYSTEM | STEP
    fn parse_module(&mut self) -> Result<Module, ParseError> {
        let mut items = Vec::new();
        while let Some(kind) = self.peek() {
            let item = match kind {
                TokenKind::Param => Item::Param(self.parse_param()?),
                TokenKind::State => Item::State(self.parse_state()?),
                TokenKind::System => Item::System(self.parse_system()?),
                TokenKind::Step => Item::Step(self.parse_step()?),
                TokenKind::Run => Item::Run(self.parse_run()?),
                _ => {
                    return Err(self.error_here(format!(
                        "expected a declaration, found {}",
                        self.describe_current()
                    )));
                }
            };

            items.push(item);
        }

        Ok(Module { items })
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

    /// SYSTEM -> "system" NAME ":" NAME "=" NAME (">>" NAME)* NEWLINE
    fn parse_system(&mut self) -> Result<SystemDecl, ParseError> {
        self.expect(TokenKind::System, "`system`")?;
        let name = self.parse_name("a system name")?;
        self.expect(TokenKind::Colon, "`:` after the system name")?;
        let state = self.parse_name("a state type name")?;
        self.expect(TokenKind::Eq, "`=` followed by the steps")?;
        let mut steps = vec![self.parse_name("a step name")?];

        while self.eat(TokenKind::Compose) {
            steps.push(self.parse_name("a step name after `>>`")?);
        }

        self.expect(TokenKind::NewLine, "end of line")?;
        Ok(SystemDecl { name, state, steps })
    }
}

pub fn parse(src: &str) -> Result<Module, ParseError> {
    Parser::new(src).parse_module()
}

const PREFIX_BP: u8 = 25;
const POSTFIX_BP: u8 = 40;

fn binary_op(kind: TokenKind) -> Option<(BinOp, u8, u8)> {
    Some(match kind {
        TokenKind::Plus => (BinOp::Add, 10, 11),
        TokenKind::Minus => (BinOp::Sub, 10, 11),
        TokenKind::Star => (BinOp::Mul, 20, 21),
        TokenKind::Slash => (BinOp::Div, 20, 21),
        TokenKind::SlashSlash => (BinOp::FloorDiv, 20, 21),
        TokenKind::Percent => (BinOp::Rem, 20, 21),
        TokenKind::Caret => (BinOp::Pow, 31, 30),
        _ => return None,
    })
}

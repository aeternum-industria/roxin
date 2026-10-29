use crate::token::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Param(ParamDecl),
    State(StateDecl),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeError {
    Bool,
    Decimal,
    Float,
    Int,
    Named(Name),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParamDecl {
    pub name: Name,
    pub ty: TypeExpr,
    pub default: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateDecl {
    pub name: Name,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: Name,
    pub ty: TypeExpr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Number {
        text: String,
        span: Span,
    },
    Name(Name),
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Unary {
        op: UnOp,
        op_span: Span,
        operand: Box<Expr>,
    },
    FieldAccess {
        base: Box<Expr>,
        field: Name,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeExpr {
    Bool,
    Decimal,
    Float,
    Int,
    Named(Name),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    pub text: String,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    FloorDiv,
    Rem,
    Pow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg,
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Number { span, .. } => *span,
            Expr::Name(name) => name.span,
            Expr::Unary {
                op_span, operand, ..
            } => Span {
                start: op_span.start,
                end: operand.span().end,
            },
            Expr::Binary { lhs, rhs, .. } => Span {
                start: lhs.span().start,
                end: rhs.span().end,
            },
            Expr::FieldAccess { base, field } => Span {
                start: base.span().start,
                end: field.span.end,
            },
        }
    }
}

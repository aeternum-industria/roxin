use crate::token::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Param(ParamDecl),
    State(StateDecl), // Here we fuckin' go! Let's start small cause I don't know what I'm doing...
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeError {
    Bool,
    Decimal,
    Float,
    Int,
    Named(Name), // State types
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
    Number { text: String, span: Span }, // Again, starting small. Baby steps.
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

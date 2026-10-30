use std::collections::HashMap;
use std::fmt;

use crate::ast::{BinOp, Expr, UnOp};
use crate::token::Span;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalError {
    pub span: Span,
    pub message: String,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn error(span: Span, message: impl Into<String>) -> EvalError {
    EvalError {
        span,
        message: message.into(),
    }
}

#[derive(Debug, Clone, Default)]
pub struct Env {
    vars: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Env::default()
    }

    pub fn set(&mut self, name: &str, value: Value) {
        self.vars.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.vars.get(name)
    }
}

pub fn eval_expr(expr: &Expr, env: &Env) -> Result<Value, EvalError> {
    match expr {
        Expr::Number { text, span } => eval_number(text, *span),
        Expr::Name(name) => match env.get(&name.text) {
            Some(value) => Ok(value.clone()),
            None => Err(error(name.span, format!("unknown name `{}`", name.text))),
        },
        Expr::Unary { op, operand, .. } => {
            let value = eval_expr(operand, env)?;
            match (op, value) {
                (UnOp::Neg, Value::Int(n)) => match n.checked_neg() {
                    Some(r) => Ok(Value::Int(r)),
                    None => Err(error(expr.span(), "integer overflow")),
                },
                (UnOp::Neg, Value::Float(x)) => Ok(Value::Float(-x)),
                (UnOp::Neg, other) => Err(error(
                    expr.span(),
                    format!("cannot negate {}", other.kind_phrase()),
                )),
            }
        }
        Expr::Binary { op, lhs, rhs } => {
            let a = eval_expr(lhs, env)?;
            let b = eval_expr(rhs, env)?;
            eval_binary(*op, a, b, expr.span())
        }
        Expr::FieldAccess { base, field } => {
            let value = eval_expr(base, env)?;
            match value.field(&field.text) {
                Some(v) => Ok(v.clone()),
                None => Err(error(
                    expr.span(),
                    format!("{} has no field `{}`", value.kind_phrase(), field.text),
                )),
            }
        }
    }
}

fn eval_number(text: &str, span: Span) -> Result<Value, EvalError> {
    let clean: String = text.chars().filter(|c| *c != '_').collect();
    if clean.contains(['.', 'e', 'E']) {
        match clean.parse::<f64>() {
            Ok(x) if x.is_finite() => Ok(Value::Float(x)),
            _ => Err(error(span, "number out of range")),
        }
    } else {
        match clean.parse::<i128>() {
            Ok(n) => Ok(Value::Int(n)),
            Err(_) => Err(error(span, "number out of range")),
        }
    }
}

fn op_symbol(op: BinOp) -> &'static str {
    match op {
        BinOp::Add => "+",
        BinOp::Sub => "-",
        BinOp::Mul => "*",
        BinOp::Div => "/",
        BinOp::FloorDiv => "//",
        BinOp::Rem => "%",
        BinOp::Pow => "^",
    }
}

fn eval_binary(op: BinOp, a: Value, b: Value, span: Span) -> Result<Value, EvalError> {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => int_op(op, x, y, span),
        (Value::Float(x), Value::Float(y)) => float_op(op, x, y, span),
        (Value::Int(x), Value::Float(y)) => float_op(op, x as f64, y, span),
        (Value::Float(x), Value::Int(y)) => float_op(op, x, y as f64, span),
        (a, b) => Err(error(
            span,
            format!(
                "cannot use `{}` on {} and {}",
                op_symbol(op),
                a.kind_phrase(),
                b.kind_phrase()
            ),
        )),
    }
}

fn int_op(op: BinOp, a: i128, b: i128, span: Span) -> Result<Value, EvalError> {
    let overflow = || error(span, "integer overflow");
    let result = match op {
        BinOp::Add => a.checked_add(b).ok_or_else(overflow)?,
        BinOp::Sub => a.checked_sub(b).ok_or_else(overflow)?,
        BinOp::Mul => a.checked_mul(b).ok_or_else(overflow)?,
        BinOp::Div => {
            return Err(error(
                span,
                "`/` is not allowed on two ints; use `//` for floor division",
            ));
        }
        BinOp::FloorDiv => {
            if b == 0 {
                return Err(error(span, "division by zero"));
            }

            let q = a.checked_div(b).ok_or_else(overflow)?;
            if a % b != 0 && ((a < 0) != (b < 0)) {
                q - 1
            } else {
                q
            }
        }
        BinOp::Rem => {
            if b == 0 {
                return Err(error(span, "division by zero"));
            }

            let r = a.checked_rem(b).ok_or_else(overflow)?;
            if r != 0 && ((r < 0) != (b < 0)) {
                r + b
            } else {
                r
            }
        }
        BinOp::Pow => {
            let exp = u32::try_from(b)
                .map_err(|_| error(span, "an int power must be a non-negative number that fits"))?;
            a.checked_pow(exp).ok_or_else(overflow)?
        }
    };

    Ok(Value::Int(result))
}

fn float_op(op: BinOp, a: f64, b: f64, span: Span) -> Result<Value, EvalError> {
    let result = match op {
        BinOp::Add => a + b,
        BinOp::Sub => a - b,
        BinOp::Mul => a * b,
        BinOp::Div => {
            if b == 0.0 {
                return Err(error(span, "division by zero"));
            }

            a / b
        }
        BinOp::Pow => a.powf(b),
        BinOp::FloorDiv | BinOp::Rem => {
            return Err(error(
                span,
                format!("`{}` is only defined for ints", op_symbol(op)),
            ));
        }
    };

    if result.is_finite() {
        Ok(Value::Float(result))
    } else {
        Err(error(
            span,
            "arithmetic error: the result is not a finite number",
        ))
    }
}

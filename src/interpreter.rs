use crate::ast::{
    InputField, Item, Module, Name, ParamDecl, RecordLit, RunDecl, StateDecl, StepDecl, SystemDecl,
    TypeExpr,
};
use crate::eval::{error, eval_expr, Env, EvalError};
use crate::token::Span;
use crate::value::Value;

#[derive(Debug, Clone, Default)]
pub struct Model {
    pub params: Vec<ParamDecl>,
    pub states: Vec<StateDecl>,
    pub steps: Vec<StepDecl>,
    pub systems: Vec<SystemDecl>,
    pub runs: Vec<RunDecl>,
}

fn type_phrase(ty: &TypeExpr) -> String {
    match ty {
        TypeExpr::Int => "an int".to_string(),
        TypeExpr::Float => "a float".to_string(),
        TypeExpr::Bool => "a bool".to_string(),
        TypeExpr::Decimal => "a decimal".to_string(),
        TypeExpr::Named(name) => format!("a {}", name.text),
    }
}

fn coerce(value: Value, ty: &TypeExpr, span: Span, what: &str) -> Result<Value, EvalError> {
    match (ty, value) {
        (TypeExpr::Int, v @ Value::Int(_)) => Ok(v),
        (TypeExpr::Float, v @ Value::Float(_)) => Ok(v),
        (TypeExpr::Float, Value::Int(n)) => Ok(Value::Float(n as f64)),
        (TypeExpr::Bool, v @ Value::Bool(_)) => Ok(v),
        (TypeExpr::Decimal | TypeExpr::Named(_), _) => Err(error(
            span,
            format!("{what}: this type is not supported yet"),
        )),
        (ty, other) => Err(error(
            span,
            format!(
                "{what} must be {}, found {}",
                type_phrase(ty),
                other.kind_phrase()
            ),
        )),
    }
}

fn assemble_state(
    decl: &StateDecl,
    given: Vec<(&Name, Value)>,
    what: &str,
    span: Span,
) -> Result<Value, EvalError> {
    for (i, (name, _)) in given.iter().enumerate() {
        if !decl.fields.iter().any(|f| f.name.text == name.text) {
            return Err(error(
                name.span,
                format!(
                    "`{}` is not a field of state `{}`",
                    name.text, decl.name.text
                ),
            ));
        }

        if given[..i]
            .iter()
            .any(|(earlier, _)| earlier.text == name.text)
        {
            return Err(error(
                name.span,
                format!("field `{}` is set twice", name.text),
            ));
        }
    }

    let mut fields = Vec::new();

    for field in &decl.fields {
        let found = given.iter().find(|(name, _)| name.text == field.name.text);
        match found {
            Some((name, value)) => {
                let what_field = format!("field `{}`", field.name.text);
                let value = coerce(value.clone(), &field.ty, name.span, &what_field)?;
                fields.push((field.name.text.clone(), value));
            }
            None => {
                return Err(error(
                    span,
                    format!("{what} does not set field `{}`", field.name.text),
                ));
            }
        }
    }
    Ok(Value::Record(fields))
}

impl Model {
    pub fn from_module(module: Module) -> Model {
        let mut model = Model::default();
        for item in module.items {
            match item {
                Item::Param(p) => model.params.push(p),
                Item::State(s) => model.states.push(s),
                Item::Step(s) => model.steps.push(s),
                Item::System(s) => model.systems.push(s),
                Item::Run(r) => model.runs.push(r),
            }
        }

        model
    }

    pub fn find_state(&self, name: &Name) -> Result<&StateDecl, EvalError> {
        let found = self.states.iter().find(|s| s.name.text == name.text);
        found.ok_or_else(|| error(name.span, format!("unknown state type `{}`", name.text)))
    }

    pub fn find_step(&self, name: &Name) -> Result<&StepDecl, EvalError> {
        let found = self.steps.iter().find(|s| s.name.text == name.text);
        found.ok_or_else(|| error(name.span, format!("unknown step `{}`", name.text)))
    }

    pub fn find_system(&self, name: &Name) -> Result<&SystemDecl, EvalError> {
        let found = self.systems.iter().find(|s| s.name.text == name.text);
        found.ok_or_else(|| error(name.span, format!("unknown system `{}`", name.text)))
    }

    pub fn eval_params(&self, overrides: &[(String, Value)]) -> Result<Env, EvalError> {
        for (name, _) in overrides {
            if !self.params.iter().any(|p| &p.name.text == name) {
                let span = Span { start: 0, end: 0 };
                return Err(error(span, format!("unknown parameter `{name}`")));
            }
        }

        let mut env = Env::new();

        for param in &self.params {
            let what = format!("param `{}`", param.name.text);
            let given = overrides.iter().find(|(n, _)| n == &param.name.text);
            let value = match given {
                Some((_, v)) => v.clone(),
                None => eval_expr(&param.default, &env)?,
            };
            let value = coerce(value, &param.ty, param.default.span(), &what)?;
            env.set(&param.name.text, value);
        }

        Ok(env)
    }

    pub fn eval_initial(&self, lit: &RecordLit, params: &Env) -> Result<Value, EvalError> {
        let decl = self.find_state(&lit.ty)?;
        let mut given = Vec::new();

        for init in &lit.fields {
            given.push((&init.name, eval_expr(&init.value, params)?));
        }

        let span = Span {
            start: lit.ty.span.start,
            end: lit.ty.span.end,
        };
        assemble_state(decl, given, "the initial state", span)
    }

    pub fn apply_step(
        &self,
        step: &StepDecl,
        state: &StateDecl,
        s: &Value,
        params: &Env,
        k: i128,
    ) -> Result<Value, EvalError> {
        let mut input_env = params.clone();
        input_env.set("s", s.clone());
        input_env.set("step", Value::Int(k));
        let mut inputs = Vec::new();

        for field in &step.inputs {
            inputs.push((field.name.text.clone(), eval_input(field, &input_env)?));
        }

        let u = Value::Record(inputs);
        let mut mech_env = params.clone();
        mech_env.set("s", s.clone());
        mech_env.set("u", u);
        let mut given = Vec::new();

        for field in &step.mechanism {
            given.push((&field.name, eval_expr(&field.value, &mech_env)?));
        }

        assemble_state(state, given, "the mechanism", step.name.span)
    }

    pub fn apply_system(
        &self,
        system: &SystemDecl,
        s: &Value,
        params: &Env,
        k: i128,
    ) -> Result<Value, EvalError> {
        let state = self.find_state(&system.state)?;
        let mut current = s.clone();
        for step_name in &system.steps {
            let step = self.find_step(step_name)?;
            current = self.apply_step(step, state, &current, params, k)?;
        }

        Ok(current)
    }
}

fn eval_input(field: &InputField, env: &Env) -> Result<Value, EvalError> {
    let value = eval_expr(&field.value, env)?;
    let what = format!("input `{}`", field.name.text);
    coerce(value, &field.ty, field.value.span(), &what)
}

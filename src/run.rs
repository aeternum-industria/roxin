use crate::ast::{RunClause, RunDecl, SystemDecl};
use crate::eval::{error, eval_expr, Env, EvalError};
use crate::interpreter::Model;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Finished,
    Stopped { at_step: i128, error: EvalError },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Job {
    pub index: usize,
    pub overrides: Vec<(String, Value)>,
    pub rows: Vec<Value>,
    pub status: Status,
}

impl Model {
    pub fn run(&self, run: &RunDecl) -> Result<Vec<Job>, EvalError> {
        let system = self.find_system(&run.system)?;

        for clause in &run.clauses {
            let RunClause::Sweep(sweep) = clause;
            if !self.params.iter().any(|p| p.name.text == sweep.param.text) {
                return Err(error(
                    sweep.param.span,
                    format!("unknown parameter `{}`", sweep.param.text),
                ));
            }
        }

        let defaults = self.eval_params(&[])?;

        let mut combos: Vec<Vec<(String, Value)>> = vec![Vec::new()];

        for clause in &run.clauses {
            let RunClause::Sweep(sweep) = clause;
            let mut values = Vec::new();

            for expr in &sweep.values {
                values.push(eval_expr(expr, &defaults)?);
            }

            let mut next = Vec::new();

            for combo in &combos {
                for value in &values {
                    let mut extended = combo.clone();
                    extended.push((sweep.param.text.clone(), value.clone()));
                    next.push(extended);
                }
            }

            combos = next;
        }

        let mut jobs = Vec::new();

        for (index, overrides) in combos.into_iter().enumerate() {
            let mut rows = Vec::new();
            let outcome = self.run_job(run, system, &overrides, &mut rows);
            let status = match outcome {
                Ok(()) => Status::Finished,
                Err((at_step, error)) => Status::Stopped { at_step, error },
            };
            jobs.push(Job {
                index,
                overrides,
                rows,
                status,
            })
        }

        Ok(jobs)
    }

    fn run_job(
        &self,
        run: &RunDecl,
        system: &SystemDecl,
        overrides: &[(String, Value)],
        rows: &mut Vec<Value>,
    ) -> Result<(), (i128, EvalError)> {
        let params = self.eval_params(overrides).map_err(|e| (0, e))?;
        let steps = self.eval_step_count(run, &params).map_err(|e| (0, e))?;
        let mut state = self
            .eval_initial(&run.initial, &params)
            .map_err(|e| (0, e))?;
        rows.push(state.clone());

        for k in 1..=steps {
            state = self
                .apply_system(system, &state, &params, k)
                .map_err(|e| (k, e))?;
            rows.push(state.clone());
        }

        Ok(())
    }

    fn eval_step_count(&self, run: &RunDecl, params: &Env) -> Result<i128, EvalError> {
        match eval_expr(&run.steps, params)? {
            Value::Int(n) if n >= 0 => Ok(n),
            other => Err(error(
                run.steps.span(),
                format!("the step count must be a non-negative int, found `{other}`"),
            )),
        }
    }
}

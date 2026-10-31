# Roxin Core Semantics (draft 0)

**Status:** draft for review. Nothing here is settled until it has a decision record in [`registry.md`](registry.md).

**Purpose:** say precisely what a Roxin program *means*, independent of how it is written. The surface syntax is still provisional and may change; the semantics below are what the oracle evaluator and the VM must both implement. Where this document and [`registry.md`](registry.md) disagree, the registry wins until the disagreement is resolved.

**Reasoning for every settled decision lives in `docs/registry.md`.**

**How to read it.** Every section below carries a **Layer** marker:

| Layer | Meaning |
|---|---|
| **formalism** | Corresponds to a definition in Zargham & Shorish, *GDS Part I*. Changing it diverges from the paper, and the divergence should be deliberate and recorded. |
| **substrate** | Language machinery the paper never addresses — types, numbers, checking, errors. Free design space; constrained only by our own decisions. |
| **workflow** | The experiment loop — runs, sweeps, jobs, seeding, results. Outside the formalism, in scope because it serves the practice. |

Decision records (D-numbers) live in [`registry.md`](registry.md). Code blocks show *illustrative* syntax only.

---

## 1. Overview

A Roxin program declares types, parameters, one evolving **state** type, one or more **steps**, a **system** (a series of steps), and **runs** of that system. A run produces a table of states, one row per step, plus metrics. Everything that can be checked before running is checked before running.

The semantics has three layers:

1. **Values and types** (section 2): what a value is and when two values are equal.
2. **One step** (section 4): how a state becomes the next state.
3. **One run** (section 6): how steps repeat, when a job stops, and what is recorded.

## 2. Values and types

> **Layer:** substrate — the paper's state space is "a collection of all objects X" and says nothing about numeric types, equality or arithmetic. All of this is ours.

| Type | Meaning | Notes |
|---|---|---|
| `int` | whole number | Arbitrary precision; overflow does not exist **[settled, D1]**. |
| `decimal(N)` | exact decimal with exactly N places | Stored as an integer scaled by 10^N. |
| `float` | IEEE 754 binary64 | Round-to-nearest-even, no fused multiply-add, no fast-math. |
| `bool` | true or false | |
| record | named fields, each with a type | `{ temp: float }`. Equality is structural **[settled]**. |
| choice | one of several named options | `type Status = Open \| Passed \| Rejected`. No payloads in v1. |

**Not yet specified:** lists, maps, text, and the `data` connectors beyond CSV (section 11).

**There is no default numeric type [settled].** Every numeric field, parameter, and metric declares its type. Literals take their type from context: in `heat: float = 0.5 * s.temp`, the `0.5` is a float because the expected type is float. A literal with no expected type (for example a bare `20` with nothing to infer from) is a check-time error.

**No implicit conversion [settled].** Mixing numeric types in one operation is a check-time error. Explicit conversions are built-in functions (`to_float`, `to_decimal(x, N, mode)`, `to_int`), specified in section 11.

### 2.1 Equality and tolerance [settled, details D6]

- `int` and `decimal` equality is exact.
- `float` `==` and `!=` require a tolerance, from `within` on the comparison or a model-wide `tolerance` declaration. Without one it is a check-time error. `<`, `<=`, `>`, `>=` never need one.
- `a == b within t` (absolute) means `|a - b| <= t`.
- `a == b within p%` (relative) means `|a - b| <= (p / 100) * max(|a|, |b|)` **[settled, D6]**. A relative tolerance therefore makes comparison against exactly 0 exact; use an absolute tolerance there.
- `within` is also allowed on `decimal` (exactness is the default there).
- `within` on `int` is a check-time error.
- Record equality is field-by-field using the rules above. A tolerance applies to every float field inside.

**Why relative tolerance uses the larger magnitude (D6).** A reader will reasonably ask why not `|a - b| <= tol * |b|`. The reasons:

1. *Symmetry.* Equality must not depend on argument order. Measuring against `b` makes `a == b` and `b == a` disagree near the boundary. The max-based test gives the same answer both ways, which also means reachability does not depend on which state was found first.
2. *No division, no special case at zero.* The test multiplies instead of dividing, so there is no divide-by-zero to define.
3. *Zero is deliberately exact under a purely relative tolerance.* Relative error against zero is meaningless (any nonzero number is infinitely far away in relative terms), so a relative tolerance never makes a number equal to 0. Where "near zero" is meant, the author states an absolute tolerance. This follows the rule that the author decides where a guarantee could weaken.
4. *Known limitation.* Tolerance equality is not transitive (`a ≈ b` and `b ≈ c` do not imply `a ≈ c`). Reachability must therefore pick a deterministic rule for merging near-equal states (for example, keep the first one found). That rule is not yet decided.

### 2.2 Arithmetic

- `+`, `-`, `*` on `int`: exact and unbounded **[settled, D1]**. A later range-refined integer type (section 11) can restrict values; a value outside its range is a run-time error.
- `//` is floor division on `int`; `%` is the matching modulo (result has the sign of the divisor, so `step % 2` is 0 or 1 for positive steps). Plain `/` on two `int`s is a check-time error **[settled, D5]**.
- `decimal(N)`: `+` and `-` on two `decimal(N)` are exact and give `decimal(N)`. `decimal(N) * int` is exact and gives `decimal(N)`. `decimal(N) * decimal(M)` is exact and gives `decimal(N+M)`, which must be brought back with `round` before it is stored in a `decimal(N)` field. `/` on decimals is only legal as the direct argument of `round` **[settled, D9]**.
- `round(e, mode)` converts to the *expected* decimal type, rounding with `down`, `up`, or `nearest` (ties to even). The mode is mandatory **[settled]**.
- `float`: ordinary IEEE binary64 arithmetic. A result that is NaN or infinite is an arithmetic error that stops the job **[settled, D4]**. Division by zero (any type) is an arithmetic error.

### 2.3 Expressions

```
expr  ::= literal | name | expr.field
        | expr op expr | op expr
        | if expr then expr else expr
        | { field: expr, ... }            -- record construction
        | s with { field: expr, ... }     -- record update (copy, then replace)
        | name(expr, ...)                 -- built-in call
```

Operators, loosest to tightest: `or`; `and`; `not`; comparisons (`== != < <= > >=`, with optional `within`); `+ -`; `* // %`; unary `-`; field access. Evaluation is strict and left to right. `if` evaluates only the taken branch. `and` and `or` short-circuit.

Built-ins in v1: `sign`, `abs`, `min`, `max`, `round`, `sqrt` (float), `to_float`, `sample` distributions (section 7). Every built-in has an effect label (`pure` or `random`) in the one function registry **[settled]**. Only `random` functions may appear in `input:` expressions.

## 3. Declarations

> **Layer:** mixed — `state`, `step`, `system` and `input map` are **formalism** (Defs 2.1, 2.8, 2.9, and closure under composition). `param`, `tolerance`, `type`, `data` are **substrate**; `metric` and `run` are **workflow**.

| Form | Meaning |
|---|---|
| `param name: Type = default` | a per-job constant, visible in every expression |
| `tolerance t` | model-wide float tolerance |
| `type Name = ...` | record, choice, or alias **[settled]** |
| `state Name { ... }` | the one record that evolves **[settled]** |
| `input map Name(s: State) -> Record = expr` | a named, replaceable input stage |
| `step Name: input: ... allowed: ... mechanism: ...` | one transition |
| `system Name: State = Step >> Step ...` | a series of steps |
| `invariant Name(s: State) = bool_expr` | true on every recorded state |
| `metric Name(s: State): Type = expr` | a value recorded each step |
| `data name = csv("file")` | an external series, loaded before the run |
| `run ...` | one experiment (section 6) |

These forms fall into two categories (D16). **Model** forms — `param`, `tolerance`, `type`, `state`,
`input map`, `step`, `system`, `invariant`, `metric`, `data` — define what the system *is*. **Experiment**
forms — `run` and its clauses — define a question asked *of* it. The engine loads the model once and applies
experiment forms to it; a file carrying no experiment form is valid and does nothing.

Name resolution is by declaration, in any order. Names are unique within their kind. Circular type aliases are errors.

## 4. One step

> **Layer:** formalism — this is the canonical form, equations (1) and (2): the Input Map `g` (Def 2.8), the Admissible Input Map `U` (Def 2.5), and the State Update Map `f` (Def 2.6), composing to `h` (Def 2.9).

A step has three parts. The first two are optional together: a step with no `input:` has no `allowed:` (nothing to constrain) and `u` is the empty record.

Given the current state `s`, the step index `k` (section 5), and the parameters:

1. **Input.** Evaluate each field of `input:` in declaration order. Each expression may read `s`, parameters, `step`, and `data`; it may **not** read other input fields **[settled, D3]**. The results form the record `u`. If the step uses a named `input map`, the map's expression is evaluated instead and must produce a record of the declared input type.
2. **Admissibility.** Evaluate `allowed:` with `s`, `u`, parameters, and `step` in scope (section 8). If it is false, the job stops with an **admissibility error** carrying `s`, `u`, `k`, and the step name.
3. **Mechanism.** Evaluate the `mechanism:` record. It reads `s`, `u`, and parameters. It must produce exactly one value for every field of the state type. A field not mentioned is a check-time error **[settled, D2]**, unless the mechanism includes `..s` (copy every unmentioned field from `s`). All fields read the *old* state, so updates are simultaneous.

The mechanism is **pure** and the checker verifies it **[settled]**: it may not call `random` functions, read data, or use `step`.

The result is the new state. It must have the state type; the checker verifies this statically.

## 5. Systems and the step index

> **Layer:** mixed — composition of steps is **formalism** (the abstract's closure under composition). The `step` index is **substrate**: the paper's `h : X → X` is autonomous, so a time index is our addition, justified as sugar for a counter in the state.

A system applies its steps in order, each step taking the previous step's output as its `s`. One pass through all steps is one **system step**.

- `step` is the index `k` of the system step being computed, counting from 1 **[settled]**. Every step inside one system step sees the same `k`.
- `step` is readable only in `input:` and `allowed:` **[settled]**.
- Intermediate states between steps inside one system step are not recorded by default; recording them is opt-in **[settled]**.
- Steps must each map the state type to itself; the checker verifies the composition **[settled]**.

## 6. One run

> **Layer:** workflow — only the Trajectory (Def 2.3) is formalism. Step counts, `until`, sweeps, jobs and the results schema are the experiment loop and have no counterpart in the paper.

A run supplies: a system, an initial state, a step count `N`, optional `until` predicate, parameter overrides (sweeps), a Monte Carlo count, a seed, and recording options. For a single job:

```
x0 = initial state
check invariants on x0           -- the start must satisfy them [settled, D10]
record row 0 (state + metrics)
for k in 1..=N:
    if until(x_{k-1}): stop      -- absorbing state [settled, D10]
    x_k = system_step(x_{k-1}, k)
    check invariants on x_k      -- see failure behavior below
    record row k (state + metrics)
```

- `for N steps` applies the system exactly N times, so a finished run has N+1 rows **[settled]**.
- `until` is a predicate on the state. If it holds, the job ends normally; the rows end at that state.
- **Failure.** Any error ends that job and records why. Other jobs in the sweep continue **[settled]**. For an invariant violation, the violating state **is** recorded as the job's final row, flagged, so the failure can be inspected **[settled, D8]**. For an admissibility, arithmetic, or data error, no new state exists, so the last row is the previous one and the error record carries the offending state and input. `keep running` (opt-in, per invariant) records the violation and continues **[settled]**.

### 6.1 Sweeps and jobs

A `sweep p in [v1, v2, ...]` clause overrides parameter `p`. Several sweep clauses form the **Cartesian product** (the spec's definition). A run with `runs M` repeats each configuration M times with different random streams. Jobs are numbered in row-major order: sweep clauses in the order written, first clause slowest, Monte Carlo run fastest. Jobs are independent, so they may run in parallel without changing results.

### 6.2 Results

One row per recorded state, with columns, in this order:

| Column | Meaning |
|---|---|
| `job` | job number (section 6.1) |
| `run` | Monte Carlo index within the configuration |
| `seed` | the job's effective seed (section 7) |
| `step` | 0 for the initial state, k after step k |
| `substep` | present only when substeps are recorded; the step's name |
| one column per swept or declared param | the job's parameter values |
| one column per state field | records flatten with dots (`pool.apples`) |
| one column per metric | recorded every step **[settled]** |

A separate **job table** has one row per job: `job`, `run`, `seed`, parameters, `status` (`finished`, `stopped`), `rows`, and for stopped jobs the `reason` (invariant name, admissibility, arithmetic, data) and `at_step`.

Metrics and invariants are pure functions of the state and parameters. They may not read `step` or `data`.

## 7. Randomness

> **Layer:** mixed — that stochastic influence enters as an *input* is **formalism** (Def 2.4's note: uncertainty is treated in the same framework as determinism). The specific generator, seeding and Box–Muller are **substrate**.

`sample` functions are the only source of randomness and are legal only in `input:` expressions **[settled]**.

Reproducibility requirements, which the oracle and the VM must both meet:

- A job's seed is an unsigned 64-bit integer. If the run gives no `seed`, the runtime draws a base seed from the operating system **and records it**, so every run can be replayed **[settled]**. Job j's seed is `splitmix64(base ^ j)` **[settled, D7]**.
- The generator is xoshiro256++, seeded by four outputs of SplitMix64 from the job seed **[settled, D7]**.
- `u01()` returns `(next_u64 >> 11) * 2^-53`, a float in [0, 1).
- `uniform(a, b)` is `a + (b - a) * u01()`.
- `normal(mu, sigma)` uses Box–Muller with a fresh pair each call and no caching: `z = sqrt(-2 ln(1 - u1)) * cos(2π u2)`, then `mu + sigma * z` **[settled, D7]**.
- Draws happen when `sample` is evaluated, in evaluation order. Because `if` evaluates only one branch, a draw inside an untaken branch consumes nothing. Evaluation order is therefore part of the language definition.

**Cross-platform caveat [settled, D7]:** `ln` and `cos` come from a math library, and different libraries may differ in the last bit. For bit-identical streams across machines the implementation should use a fixed pure-Rust math library (for example the `libm` crate) and not the platform's.

## 8. `allowed`

> **Layer:** formalism — Def 2.5, the Admissible Input Map `U : X → ℘(U)`. Bounds that depend on state are the definition itself, not an extension. The enumeration rules exist to serve Def 4.1, `R(x) = ∪_{u∈U_x} f(x,u)`.

`allowed:` is a conjunction of clauses joined by `and`. Each clause is either:

- **a membership clause** `field in set`, or
- **a predicate**: any boolean expression over `s`, `u`, parameters, and `step`.

A set expression is: an inclusive range `a..b` (bounds are expressions over `s`, parameters, and `step`, not over `u`); a finite set `{v1, v2, ...}`; or a choice type name (all its options).

Meaning:

- **Check (simulation).** After the input stage, every clause must be true for the produced `u`. A range on a float field is a check of `a <= x <= b`.
- **Menu (reachability, later).** The admissible set is the product of the membership sets, filtered by the predicates. It is **enumerable** only if every input field has a membership clause whose set is finite: an `int` range, a `decimal(N)` range (stepping by 10^-N), a finite set, or a choice type. A `float` range is not enumerable. Reachability on a step with a non-enumerable input is an error unless a sampling mode is chosen. Reachability ignores the `input:` expressions and iterates the menu **[settled lean]**.

## 9. Check-time (static) rules

> **Layer:** substrate — the paper has no checker. Every rule here is ours, and each one should trace to a decision record.

Before any run, the checker verifies:

1. every name resolves, every field exists, and every expression has a type;
2. no implicit numeric conversions; literals have an expected type;
3. float `==`/`!=` have a tolerance; `within` is not used on `int`;
4. mechanisms are pure; `random` functions appear only in `input:`; `step` appears only in `input:` and `allowed:`;
5. each mechanism produces every state field (or uses `..s`); each step maps state to state; the system composes;
6. `allowed` exists only on steps with an input stage; every clause is boolean;
7. input maps return the declared input record type;
8. sweeps name declared parameters, with values of the parameter's type;
9. metrics and invariants read only the state and parameters;
10. reachability requests are made only on models whose state has a `tolerance` if it has float fields **[settled]**.

## 10. Errors

> **Layer:** substrate — the paper has no failure model. Note D4 (NaN/infinity/divide-by-zero stops the job) is where this layer pushes back on the formalism, which would happily let a trajectory continue.

| Error | When | Recorded |
|---|---|---|
| check error | before running | message with source location; nothing runs |
| admissibility error | `allowed` is false | state, input, step number, step name |
| invariant violation | an invariant is false | invariant name, step; the state is the final row **[settled, D8]** |
| arithmetic error | division by zero, NaN, infinity | expression location, step |
| data error | indexing past the end of data | data name, index, step |

All run-time errors stop only their own job.

## 11. Not yet specified

Range-refined integer types; lists and maps; text values; choices with payloads; `data` connectors beyond CSV; conversion functions in full; reachability search (depth, deduplication, result shape); metric export for nested types; `keep running` syntax; checkpoint and resume; the result store; the REPL's behavior (what an unfinished definition does, how a model is reloaded); `every N steps`.

## 12. Decisions needed

We will go through these one at a time. Each has my proposal.

| # | Decision | Proposal |
|---|---|---|
| D1 | Width of `int` | **Settled:** arbitrary precision. Range-refined integers (`type U256 = int in 0..2^256-1`) are a planned later addition. See the verification note below. |
| D2 | Mechanism omits a state field | **Settled:** check error, with `..s` to copy the rest. The author must account for every field; forgetting one cannot happen silently. |
| D3 | Independence of input fields | **Settled:** fields are independent. Input expressions cannot read each other (evaluated in declaration order). The admissible set is the product of per-field membership sets, filtered by any joint predicates, so coupling is always an explicit predicate. Keeps enumeration, random-draw order, and proofs simple. |
| D4 | NaN, infinity, division by zero | **Settled:** errors that stop the job, recorded with step, field, and expression; other jobs continue. No propagate mode for now. |
| D5 | Integer division | **Settled:** `//` floors, `%` is floored modulo (`a == (a // b) * b + a % b`), and plain `/` on `int` is a type error. May be revisited. |
| D6 | Relative tolerance | **Settled:** relative to `max(\|a\|, \|b\|)`. See the rationale after section 2.1's rules. |
| D7 | Random generator | **Settled:** xoshiro256++ seeded by SplitMix64; Box–Muller for `normal`; a pinned pure-Rust math library for bit-identical streams. Reasoning in `docs/registry.md`. |
| D8 | Violating state | **Settled:** recorded as the job's final row, flagged with the invariant's name. Reasoning in `docs/registry.md`. |
| D9 | Decimal multiplication and division | **Settled:** `decimal(N) * decimal(M)` is exact and gives `decimal(N+M)`; `decimal(N) * int` stays `decimal(N)`; `/` only appears as the direct argument of `round`. Reasoning in `docs/registry.md`. |
| D10 | Order of checks in a run | **Settled:** invariants on the initial state first; `until` before each step; invariants after each step. Reasoning in `docs/registry.md`. |

**Verification note (D1).** Unbounded integers map directly onto the integer theory of SMT solvers, so proofs about a model never need bit-vector reasoning about wraparound. Overflow is a stated property (`invariant NoOverflow(s) = s.balance < 2^256`) instead of hidden operator behavior. A model of a contract that wraps or reverts at a fixed width is an abstraction unless that bound is stated, so the guides must say so.

## 13. Illustrative Rust types (starting point for M1)

```rust
enum Ty { Int, Decimal(u32), Float, Bool, Record(Vec<(Name, Ty)>), Choice(Name) }

enum Value {
    Int(i128),                         // D1 settled: unbounded semantics; start with i128, fall back to a big integer on overflow
    Decimal { unscaled: i128, scale: u32 },
    Float(f64),
    Bool(bool),
    Record(Vec<(Name, Value)>),
    Choice(Name, Name),                // (type, option)
}

enum Expr {
    Lit(Value), Var(Name), Field(Box<Expr>, Name),
    Un(UnOp, Box<Expr>), Bin(BinOp, Box<Expr>, Box<Expr>),
    Cmp { op: CmpOp, l: Box<Expr>, r: Box<Expr>, within: Option<Tol> },
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Record(Vec<(Name, Expr)>), With(Box<Expr>, Vec<(Name, Expr)>),
    Call(Name, Vec<Expr>),
}

struct Step {
    name: Name,
    input: Vec<(Name, Ty, Expr)>,      // may be empty
    allowed: Vec<Clause>,              // empty only if input is empty
    mechanism: Mechanism,              // fields plus an optional spread of `s`
}
```

The evaluator takes a state, a step index, parameters, and a random generator, and returns a new state or an error.

## 14. Test vectors

Every value below was computed independently with exact rational arithmetic (and IEEE doubles where noted), not by a Roxin implementation, so they can serve as the oracle for M1–M3. Float comparisons in tests should use a relative tolerance of 1e-12 unless the value is exactly representable.

**Thermostat** (start 10, setpoint 20, 8 steps, `heat = k * (setpoint - temp)`; all values are exact binary fractions, so float arithmetic reproduces them exactly)

| k | rows 0–3 | row 8 |
|---|---|---|
| 0.25 | 10, 12.5, 14.375, 15.78125 | 18.998870849609375 |
| 0.5 | 10, 15, 17.5, 18.75 | 19.9609375 |
| 1.5 | 10, 25, 17.5, 21.25 | 19.9609375 |

With `allowed: heat in 0..5`: k = 0.5 first heat is 5 (boundary, legal); k = 1.5 first heat is 15, an admissibility error at step 1 with state `temp = 10`.

**Marbles** (arm 1 acts when `step % 2 == 0`, arm 2 when `step % 3 == 0`; start `(10, 0)`, 10 steps). Rows 0–10:
`(10,0) (10,0) (9,1) (8,2) (7,3) (7,3) (5,5) (5,5) (5,5) (5,5) (5,5)`

**Savings pool** (withdraw 10% of balance, then add 1%; start 100): 100, 90.9, 82.6281, 75.1089429. In `decimal(7)` these are exact. In `float` compare with a tolerance.

**Predator–prey** (birth 0.1, hunt 0.002, conv 0.5, death 0.1; start prey 40, pred 9; `kills = hunt * prey * pred`):

| step | prey | pred |
|---|---|---|
| 0 | 40 | 9 |
| 1 | 43.28 | 8.46 |
| 2 | 46.8757024 | 7.9801488 |

Step-1 kills = 0.72, step-2 kills = 0.7322976. Equilibrium: start `(100, 50)` and every step returns `(100, 50)`.

**SIR** (N = 1000, beta 0.3, gamma 0.1; start S 990, I 10, R 0):

| step | S | I | R | S + I + R |
|---|---|---|---|---|
| 1 | 987.03 | 11.97 | 1 | 1000 |
| 2 | 983.48557527 | 14.31742473 | 2.197 | 1000 |

**Linear bonding curve** (m = 0.01, supply 100, reserve 50; buy 10 tokens): cost = 10.5, new reserve = 60.5, and `m * 110^2 / 2 = 60.5` (invariant holds exactly).

**Constant-product market, float trap:** with apples 100 and oranges 100 (k = 10000), a swap of 39 apples gives apples = 139 and oranges = `10000 / 139`; in IEEE double the product is `9999.999999999998`, so an exact `==` fails and a tolerance of 1e-9 relative passes.

**Decimal rounding, constant-product with `decimal(6)`:** apples 110, oranges = `round(10000 / 110, mode)`:

| mode | oranges | apples × oranges | `>= 10000`? |
|---|---|---|---|
| up | 90.909091 | 10000.000010 | yes |
| down | 90.909090 | 9999.999900 | no |

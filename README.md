# Roxin

A domain-specific language for modeling, simulating, and checking **generalized dynamical systems** (GDS).

You describe a system (its state, how inputs are chosen, which inputs are legal, and what the rules do), run it at scale, and export typed results to the tools you already use.

Named for Emilio Roxin, whose 1960s work on generalized dynamical systems is the mathematical origin of the idea.

> **Status: design phase.** Nothing is implemented yet. This README records the current design, what has been decided, what is still open, and what could go wrong. All syntax below is provisional. Precise semantics are drafted in `docs/core-semantics.md`, and the reasoning behind each decision is in `docs/decisions.md`.

---

## Contents

1. [Why this exists](#why-this-exists)
2. [GDS in 60 seconds](#gds-in-60-seconds)
3. [A taste of Roxin](#a-taste-of-roxin-provisional-syntax)
4. [Numbers](#numbers-provisional-syntax)
5. [Declarations](#declarations-provisional-syntax)
6. [Time and replayed data](#time-and-replayed-data-provisional-syntax)
7. [Scope](#scope)
8. [Design decisions](#design-decisions) (principles, defaults, decided, leans, `extern`)
9. [Planned features](#planned-features)
10. [Documentation plan](#documentation-plan)
11. [Roadmap](#roadmap) and benchmark models
12. [Risks](#risks) and known gaps
13. [References](#references)
14. [Open Questions](#open-questions)

---

## Why this exists

[cadCAD](https://github.com/cadCAD-org) showed that GDS is a useful way to model complex systems such as markets, token economies, and governance. It is a framework on top of Python, which forced heavy metaprogramming to get the structure it needed, left most names as strings, and made it hard to check anything before a long run started.

Roxin asks what happens if the GDS ideas are first-class parts of a language:

- The compiler can reject a mis-wired model before it runs.
- A rule that must stay pure can be enforced instead of trusted.
- One program can be *simulated*, *explored exhaustively*, and *checked against invariants* without rewriting it.
- Results are typed, reproducible, and cached.

Roxin is also a vehicle for learning language design and implementation: type checking, interpreters, bytecode VMs, and eventually verification.

---

## GDS in 60 seconds

A GDS is a **state** plus a **rule** that produces the next state. Roxin follows the vocabulary of the GDS paper (Zargham & Shorish, 2022):

| Paper term | Meaning | Video-game analogy | Roxin |
|---|---|---|---|
| State `x` | everything about the system right now | your save file | `state` |
| Input `u` | what is chosen or arrives this step | the move you pick | the `input:` record |
| Admissible Input Map `U(x)` | which inputs are legal *in this state* | you can't buy a sword with 0 coins | `allowed:` |
| Input Map `g` | how the input is produced from the state | the player: human, bot, or dice | expressions in `input:`, or a named `input map` |
| State Update Map `f` | the law of motion: next state from state and input | the game rules | `mechanism:` |
| State Transition Map `h` | `h(x) = f(x, g(x))` | one full turn | `step` |
| Trajectory | `x0, x1, x2, ...` | the replay | the run output |

**An input is not a change to state.** The input is what gets chosen or arrives; the mechanism decides what that does. A thermostat's input is "heat 5," and the mechanism is the physics that turns it into a new temperature.

The central idea is **separating who decides from what happens**. Swap the decision rule without touching the laws of motion, or change the laws without touching the decision rule.

Because inputs come from a *set* of legal choices, one model can answer three questions:

1. **Simulate.** What happens if inputs are produced this way?
2. **Reach.** What *could* happen, given every legal input?
3. **Check.** Is there any reachable state that breaks a rule?

---

## A taste of Roxin (provisional syntax)

A thermostat. The decision is the `input:` line; the physics is the `mechanism:` line.

```
param setpoint: float = 20
param k: float = 0.5

state Room { temp: float }

step Heating:
    input:
        heat: float = k * (setpoint - s.temp)    -- the decision
    mechanism:
        temp: s.temp + u.heat                    -- the physics

system Thermostat: Room = Heating

run Thermostat from Room { temp: 10 } for 8 steps
    sweep k in [0.25, 0.5, 1.5]
```

Inside a step, `s` is the old state and `u` is the input record. The mechanism is one record of per-field expressions that all read the old state, so fields update simultaneously. A mechanism must account for every field of the state: fields it does not change are carried over with `..s`.

```
state Room { temp: float, humidity: float }

mechanism:
    temp: s.temp + u.heat
    ..s                                -- humidity passes through unchanged
```

Decision rules can be named and swapped for A/B comparison, and `allowed` limits what is legal:

```
input map Proportional(s: Room) -> { heat: float } =
    { heat: k * (setpoint - s.temp) }

input map BangBang(s: Room) -> { heat: float } =
    { heat: if s.temp < setpoint then 5 else 0 }

step Heating:
    input: Proportional
    allowed: heat in 0..5               -- the heater can't exceed 5
    mechanism:
        temp: s.temp + u.heat
```

A step with no choice to make omits `input:` entirely:

```
state Savings { balance: decimal(2) }

step Interest:
    mechanism:
        balance: round(s.balance * 1.01, nearest)
```

`allowed` is declared at the step level, so it can constrain several input fields together (for example `alice + bob <= s.balance`).

Design intent for the surface language:

- Ordinary `name: Type` annotations everywhere. No invented type syntax.
- Keywords only for GDS ideas and a few everyday words: `state`, `type`, `param`, `step`, `input`, `allowed`, `mechanism`, `input map`, `system`, `invariant`, `metric`, `data`, `tolerance`, `run`, `sweep`, and later `reach`.
- **A feature you don't use should cost you no syntax.**

---

## Numbers (provisional syntax)

Every numeric field says which kind of number it is. There is no default type and no default tolerance, so the author decides how each value is stored and compared.

```
tolerance 0.0001%                      -- model-wide, stated once

state Booth { apples: float, oranges: float }
param k: float = 10000

invariant Conserved(s: Booth)    = s.apples * s.oranges == k                  -- uses the model tolerance
invariant Balanced(s: Booth)     = s.apples == s.oranges within 0.5           -- overrides it (absolute)
invariant NotShrinking(s: Booth) = s.apples * s.oranges >= k                  -- no tolerance needed
```

- `within 0.5` is an absolute tolerance; `within 0.0001%` is relative.
- A `float` `==` or `!=` with neither `within` nor a model `tolerance` is a check-time error: *"Float equality needs a tolerance. Add `within`, or declare `tolerance` for the model."*
- A purely relative tolerance never makes a nonzero number equal to 0, so use an absolute `within` there.
- `decimal` equality is exact unless `within` is given. `int` equality is always exact.
- `int` has no size limit. Mixing `float` with `decimal` or `int` without a conversion is a type error.
- Integer division is `//` (floor) and `%` (floored modulo). Plain `/` on two `int` values is a check error.
- A NaN, an infinity, or a division by zero stops the job and is recorded.

Decimals say how many places they keep, and rounding is explicit:

```
state Pool { apples: decimal(6), oranges: decimal(6) }
param k: decimal(6) = 10000

mechanism:
    oranges: round(k / (s.apples + u.in), up)      -- the rounding choice is visible
    ..s
```

With 110 apples, rounding **up** gives `90.909091` and a product of `10000.000010` (so `apples * oranges >= k` holds). Rounding **down** gives `90.909090` and `9999.999900` (so it fails). Both values were checked with exact arithmetic.

---

## Declarations (provisional syntax)

```
state Room { temp: float }

type Agent  = { id: int, balance: decimal(2) }
type Status = Open | Passed | Rejected          -- a choice among named options
type Money  = decimal(2)                        -- an alias

system Thermostat: Room = Heating
```

`state` marks the record that evolves. The checker confirms that the type named in `system` was declared as a `state`.

---

## Time and replayed data (provisional syntax)

`step` counts the transition being computed, starting at 1. A model with parts that act at different frequencies (adapted from the cadCAD "robot and marbles" tutorial, where robot 1 acts every 2 steps and robot 2 every 3):

```
state Boxes { box_a: int, box_b: int }

step Robots:
    input:
        arm1: int = if step % 2 == 0 then sign(s.box_b - s.box_a) else 0
        arm2: int = if step % 3 == 0 then sign(s.box_b - s.box_a) else 0
    mechanism:
        box_a: s.box_a + u.arm1 + u.arm2
        box_b: s.box_b - u.arm1 - u.arm2

system Marbles: Boxes = Robots
run Marbles from Boxes { box_a: 10, box_b: 0 } for 10 steps
```

Each decision gets its own input field, and the mechanism combines them explicitly. Nothing is merged implicitly.

Replaying outside data:

```
state Account { balance: decimal(2), holdings: int }

data prices = csv("prices.csv")           -- columns: step, price

step Trade:
    input:
        price: decimal(2) = prices.price[step]
    mechanism:
        balance: s.balance + s.holdings * u.price
        ..s
```

A possible later addition is `every N steps` on a step as sugar for "this step does nothing on other steps."

---

## Scope

**In scope**

- Typed records, steps, and series pipelines
- Invariants and metrics, parameter sweeps, Monte Carlo runs, A/B variants, seeded reproducibility
- Features that are not part of the GDS formalism but serve the simulation workflow (metrics, A/B variants, sweeps, a result store)
- Reachability analysis
- A small, pure expression language for step bodies
- A closed set of built-in data connectors
- A typed result store with caching, plus export

**Out of scope (for now)**

- A general-purpose programming language
- Statistics and plotting inside the language (use the export layer)
- Notebook replacement
- User-defined effects and a package ecosystem
- Continuous time and ODE solvers
- Uneven or event-driven time (periodic activation through `step` is supported)

---

## Design decisions

### Design principles

**Defaults over configuration.** The 99% case must need no configuration. Every option has a named default chosen for the common case, rarely used machinery is hidden, and anything hidden stays configurable. A new feature is judged by how little it costs someone who does not use it. One of cadCAD's biggest pain points was the boilerplate needed before a basic run would execute.

**Boundary rule.** A default is acceptable when the common case is clear and a wrong default is visible and harmless (the seed, the progress bar, the console output). Where a wrong default could silently weaken a guarantee, as with number types and float tolerance, the author must decide and the checker refuses to guess.

**Verification rule.** Every feature must state how it appears to a solver (what a proof about it would look like), or be explicitly marked unverifiable. The language is designed so that properties of models can be checked or proved, not only observed. The central proof is an inductive invariant: it holds at the start, and for every state satisfying it and every admissible input, the mechanism's result satisfies it again. Proof support is planned, not built; the table shows how each feature is expected to look to a solver.

| Feature | How a solver sees it | Difficulty |
|---|---|---|
| `int` | integer arithmetic (unbounded, so no wraparound reasoning) | straightforward |
| `decimal(N)` | integers scaled by a power of ten; `round` is a defined integer operation | straightforward |
| `allowed` | declarative constraints; ranges and membership become linear constraints | straightforward |
| Pure mechanisms | a mechanism is a function from old state and input to new state | straightforward |
| `sample` | a nondeterministic choice: the proof must hold for every outcome | moderate |
| `float` | floating-point theory, or an approximation with stated error | hard |
| Lists and maps | need quantifiers or bounded unrolling | hard |

**Authorship.** The implementation is written entirely by hand. AI assistance is limited to documentation, brainstorming, review, and explanation (see `docs/decisions.md`, P1).

### Defaults (provisional)

Every run has defaults chosen for the common case. Settings marked "settled" are already in the decided tables below.

| Setting | Default | To override |
|---|---|---|
| Number of steps | the only thing you must state besides the model and the start | n/a |
| Steps counted | N transitions (settled) | none |
| In-between rows | off (settled) | `record substeps` |
| Monte Carlo runs | 1 | `runs 1000` |
| Random seed | random, but saved in the results and printed, so any run can be replayed | `seed 42` |
| Parallelism | all cores when there are several jobs | `jobs 1` |
| Results table | one row per step, with job, seed, and parameter columns, saved to the result store | `export parquet` |
| Console output | quiet, one summary line; progress bar only for runs over a few seconds | `verbose` |
| Invariant violation | stops that job and is recorded; other jobs continue (settled) | `keep running` |
| Number type | none: every numeric field declares `int`, `decimal`, or `float` (settled) | n/a |
| Float equality | none: needs `within` or a model-wide `tolerance` (settled) | `within`, `tolerance` |
| Recorded columns | full state plus all metrics, every step (settled) | `record metrics only` |

### Decided

Each decision's alternatives, reasoning, costs, and revisit triggers are in `docs/decisions.md` (D1 to D10 so far). Rows without a D-number were settled earlier in the design discussion.

**Scope and vocabulary**

| Decision | Choice |
|---|---|
| Scope | A DSL for the model → experiment → results loop. Not general-purpose. |
| Analysis | Export layer: Parquet/CSV for R and Python, Vega-Lite for charts. |
| Vocabulary | The GDS paper's terms win over cadCAD's. There is no `policy` keyword. |
| Syntax style | Standard typed annotations; keywords only for GDS concepts. |
| Audience and vocabulary | The core language and main documentation are domain-neutral, with a deliberate mix of example families (games, economics and tokens, life sciences, physical control) so that no single field dominates. |
| One vocabulary | Every audience ends up using the same GDS-derived terms (`state`, `input`, `mechanism`, `step`, `invariant`, and so on). Field-specific words (turn, block, generation) may appear once as an on-ramp in an audience guide, and are then retired in favor of the Roxin term. They never become keywords or aliases. |
| Audience guides | Planned: one guide per field (for example video game developers, economics and tokenomics, life sciences), each covering the same ground in the same order with its own family of examples. A concept appears in all guides under the same name. |

**Model structure**

| Decision | Choice |
|---|---|
| Declarations | `state` declares the one record that evolves (a GDS concept). `type` declares everything else: records, choices among named options, and aliases. There is no `space` or `record` keyword. |
| System state type | Stated explicitly: `system Thermostat: Room = ...` |
| Step shape | A step has an optional `input:` stage, a step-level `allowed:`, and one `mechanism:`. |
| Steps without choices | A step may omit `input:` entirely (the paper describes the input stage as optional). |
| Admissibility | `allowed` belongs to the step, because the paper defines the admissible input map separately from the input map. |
| Mechanism form | One record of per-field expressions, all reading the old state. (D2) |
| Unmentioned fields | A mechanism must account for every state field. A field it neither sets nor copies is a check error. `..s` copies all unmentioned fields from the old state, so the author acknowledges them as passthroughs. (D2) |
| Input field independence | Input fields do not read each other. The admissible set is the product of per-field membership sets, filtered by joint predicates; any coupling between fields is an explicit predicate. (D3) |
| Simultaneous decisions | Several decisions in one step are fields of one input record. Joint rules go in the step-level `allowed`. Composing separate subsystems is deferred. |
| Decision swapping | Named `input map` forms can be replaced for A/B comparison. |
| Composition | `>>` joins steps only; each step takes the state in and returns it out. |

**Time, running, and randomness**

| Decision | Choice |
|---|---|
| Step counting | `for N steps` applies the system N times, so a run records N+1 states including the initial one. |
| Intermediate states | Rows between steps inside one system step are **not** recorded by default; recording them is opt-in (`record substeps`, syntax provisional). |
| Step index | A built-in `step` is readable in `input:` and `allowed:` expressions only, not in mechanisms or metrics. It counts the transition being computed, starting at 1: results row k is the state after step k, and row 0 is the initial state. Anything that depends on time enters the law of motion as an input. |
| Time model | Uniform discrete steps. Periodic activation (an actor that acts every N steps) uses `step`, for example `if step % 3 == 0 then ... else 0`. Uneven or event-driven time is deferred. |
| Replayed data | `data` declarations load outside series before the run (for example a price file); `input:` expressions index them by `step`. Running past the end of the data is an error, not a silent repeat. Syntax provisional. |
| Order of checks in a run | Invariants are checked on the initial state before step 1. `until` is tested before each step, so one that is already true does zero steps. Invariants are checked after each step. If a state violates an invariant and satisfies `until`, the violation wins. (D10) |
| Random numbers | xoshiro256++ seeded by SplitMix64 (job j uses `splitmix64(base ^ j)`); uniform floats `(next_u64 >> 11) * 2^-53`; Box–Muller for normal draws; a pinned pure-Rust math library, so a seed reproduces bit-for-bit across machines and across the VM and the oracle. (D7) |

**Numbers**

| Decision | Choice |
|---|---|
| Number types | There is **no default numeric type**. Every numeric field declares `int`, `decimal`, or `float`. Literals take their type from context. Mixing numeric types is a type error. |
| Integer size | `int` is an arbitrary-precision mathematical integer; overflow does not exist. Implementations may use a fast small-integer path with a big-integer fallback. Range-refined integers (for example `type U256 = int in 0..2^256-1`), reusing the range form from `allowed`, are a planned later addition; a value outside its type's range is a run-time error. (D1) |
| Integer division | `//` is floor division and `%` is floored modulo, so `a == (a // b) * b + a % b` and `%` takes the sign of the divisor. Plain `/` on two `int` values is a check error: use `//`, or convert to `decimal` or `float`. (D5) |
| Decimal arithmetic | The number of places is part of the type: `decimal(N)` has exactly N places. `+` and `-` are exact. `decimal(N) * decimal(M)` is exact and has type `decimal(N+M)`; `decimal(N) * int` stays `decimal(N)`. Storing a product back into a `decimal(N)` field needs `round(e, mode)` with `down`, `up`, or `nearest`. `/` on decimals is legal only as the direct argument of `round`, so the rounding policy is always written down. (D9) |
| Float tolerance | There is **no default tolerance**. Any `==` or `!=` on floats needs a tolerance from `within` (per comparison) or a model-wide `tolerance` declaration; otherwise it is a check-time error. `<`, `>`, `<=`, `>=` never need one. `decimal` equality is exact unless `within` is given; `int` equality is always exact. Syntax provisional. |
| Relative tolerance | `a == b within p%` means `\|a - b\| <= p * max(\|a\|, \|b\|)`. It is symmetric (argument order does not matter), needs no division, and never makes a nonzero number equal to 0: use an absolute tolerance for "near zero". Tolerance equality is not transitive, so reachability needs a deterministic rule for merging near-equal states (open). (D6) |
| NaN, infinity, division by zero | A NaN or infinite float result, or division by zero of any type, is an error that stops that job. The error is recorded with the step, field, and expression, and other jobs continue. There is no propagate mode. (D4) |
| State equality (reachability) | Two states are the same when all `int` and `decimal` fields match exactly and `float` fields match within the model's `tolerance`. Reachability on a model with float fields and no declared `tolerance` is an error. Near-equal states on opposite sides of a rounding boundary may fail to merge, which costs search size but never correctness. |

**Observables, results, and failure**

| Decision | Choice |
|---|---|
| Observables | `metric` records a value computed from the state at each step; `invariant` checks a true/false rule and stops the job when it fails. Both are pure functions of state. A metric may have any type (the author declares it). Neither is part of the formal GDS framework, and both are in scope as features of the simulation workflow. |
| Recording | Every declared metric is recorded every step. The results table holds the full state plus the metrics by default; `record metrics only` is opt-in for large states. |
| Metric export | Record-typed metrics flatten into dotted columns (for example `Pop.prey`). Choices export as categorical text. Lists and maps stay nested in Parquet and are serialized as JSON text in CSV. |
| Invariant violation | A violation stops that job; other jobs in the same sweep continue. The violating state is recorded as the job's final row, flagged with the invariant's name, so it can be inspected. Continuing past a violation is opt-in per invariant (provisional: `keep running`). Other failures (admissibility, arithmetic, data) create no new state, so the last row is the previous state and the error record carries the offending state and input. (D8) |

**Implementation and process**

| Decision | Choice |
|---|---|
| Build order | AST and semantics first. An s-expression reader comes early so programs can be typed into a REPL; the friendly surface syntax comes last, once the semantics are settled. |
| Implementation | Written in Rust, with no Python implementation. A **bytecode compiler and VM** is the primary way to run programs. A small **tree-walking evaluator** is kept as a test oracle and a learning aid, not a product. Programs run on demand from the command line or an interactive **REPL**. |
| Backends | The compiler is organized as front end → checker → typed IR → backend, so backends are modular. Only the bytecode VM is planned. Compiling to C or WebAssembly is deferred until real feedback from cadCAD users has been addressed. |
| Testing | The hand-computed benchmark values are the fixed oracle (checked independently with exact arithmetic). The VM and the tree-walker must agree on every benchmark; any later backend must agree too. |
| Performance | Treated as a measured trigger, revisited throughout development. Measurements: time per step for one job (agent-based traders at 100, 1,000, and 10,000 agents), total time for a 1,000-job sweep on one core and on all cores, and a breakdown of where time goes. Thresholds are set once the VM runs. Benchmarks are mainly comparisons against the same models implemented in cadCAD (original) and cadCAD.jl. Escalation order: parallel jobs, in-place state updates, a faster backend, then a JIT. |
| `extern` (foreign code) | Deferred. Placeholder rules below. |
| Data connectors | Closed menu: CSV, Parquet, SQLite, HTTP GET. Effects only in `data` declarations and `input:` stages. |
| Authorship | The implementation is written entirely by hand; AI is used for documentation, brainstorming, review, and explanation only (P1 in `docs/decisions.md`). |

### Current leans (not yet confirmed)

| Topic | Lean |
|---|---|
| Type equality of states and records | Exact structural (same field names and types). Width subtyping and row polymorphism later. |
| Purity | Mechanisms must be pure and the checker verifies it. `input:` expressions may `sample`. |
| `allowed` form | Set expressions (ranges, finite sets, products, filters), so they can be both checked and enumerated. Checked at runtime in v1. |
| Reachability | Enumerates the `allowed` set and ignores the input expressions, so no branching happens inside user code. |
| Results | Parameters, job index, and seed attached as columns; per-step timing built in. |
| Self-describing results | Results record the effective settings used (seed, tolerance, step counting), so a results file describes how it was produced. |
| Checking | A pre-run checker for wiring, types, and mechanism purity. Admissibility and invariants checked at runtime. |
| Expression language | Small and pure: records, lists, maps, `if` as an expression, `map`/`filter`/`fold`. Termination as an opt-in guarantee. |
| Parameters | A per-job record. Time-varying parameters live in the state explicitly. |
| Global state | None. An experiment is a value, not an entry in a global registry. |

### The `extern` placeholder rules

Foreign code is deferred, but four cheap rules keep the door open:

1. Every callable, built-in or later foreign, is looked up through **one function registry**.
2. Every function carries an **effect label** (pure, random, reads-file, network).
3. The word `extern` is **reserved** in the grammar.
4. Effects never appear inside mechanisms or ordinary expressions.

If a real model needs foreign code, the first candidate is WebAssembly for pure functions, since a sandboxed module can have its purity enforced rather than promised.

---

## Planned features

- Typed states, steps, and pipelines
- Invariants and metrics checked during simulation and over reachable states
- Parameter sweeps, Monte Carlo, and A/B variants with run metadata (job index, seed, parameters)
- Reachability analysis (bounded breadth-first search over admissible inputs)
- Parallel execution of independent jobs
- A result store keyed by model, parameters, and seeds; checkpoint and resume
- Typed result tables, so a misspelled field fails before a long run
- Export to Parquet, CSV, and Vega-Lite chart specifications
- Later: range-refined integers, and proofs of invariants (inductive invariant checking with a solver)

---

## Documentation plan

- **Core reference and tutorial:** domain-neutral, using a balanced mix of examples. The thermostat, marbles, predator–prey, SIR, and market models already span control, games, biology, and economics. A rough guard: no field supplies more than about a third of the examples in the core docs.
- **Audience guides:** same chapters, same order, same terms, different example families. The test for a guide is that a model written from one guide is readable by someone who learned from another. The guides must say plainly that unbounded `int` can diverge from a real contract at overflow boundaries, and show how to state the bound as an invariant (for example `invariant NoOverflow(s: Pool) = s.balance < 2^256`).
- **Terminology rule:** a guide may introduce a concept through a familiar word once, then switches to the Roxin term for good. Roxin ships no domain aliases.
- **Defaults reference:** every default listed in one page (see the defaults table), kept in sync with the implementation.
- **Decision log:** `docs/decisions.md` records every decision with its alternatives and reasoning; new decisions are appended, and changes are added as dated notes, never silent edits.

---

## Roadmap

| Milestone | Build | Done when |
|---|---|---|
| M0 | Read through `docs/core-semantics.md` (its decisions D1–D10 are settled); settle the questions in Open Questions at the end; collect 3–5 real models from GDS practitioners | the draft has been reviewed and the real models are in hand |
| M1 | Rust project, AST, and a tree-walking evaluator for one step (the oracle) | the thermostat with k = 0.5 goes `10 → 15 → 17.5 → 18.75` |
| M2 | Trajectory loop, stop rule, output table; s-expression reader and a minimal REPL | all three k values match the closed-form values below; predator–prey holds still at equilibrium; the marbles model reaches `(5,5)`; a model can be typed into the REPL |
| M3 | Typed IR and a bytecode compiler and VM | every M1–M2 benchmark gives identical results on the VM and the tree-walker |
| M4 | `allowed` sets with runtime checks; seeded sampling | the thermostat with k = 1.5 and `allowed: heat in 0..5` raises an error naming the state and input; a seed repeats a run on both engines |
| M5 | Sweeps, Monte Carlo, and A/B variants with run metadata; parallel jobs | a 3×2 sweep yields exactly 6 jobs; `Proportional` versus `BangBang` runs side by side |
| M6 | Pre-run checker: wiring, types, mechanism purity | an impure mechanism is rejected before running |
| M7 | Reachability, metrics, and invariant checking | the market model's reachable states all satisfy its invariant |
| M8 | The first surface syntax | all benchmark models are written as source text |
| M9 | Governance model; composing subsystems | the governance model runs with parameters in state |

The VM becomes the primary engine at M3. The tree-walker stays as the oracle. Backends beyond the VM are not scheduled.

### Benchmark models (all must work without `extern`)

1. **Thermostat** (closed form, no choice involved beyond the decision rule): after 8 steps, k = 0.25 gives `18.998870849609375`, and k = 0.5 and k = 1.5 both give `19.9609375`. With k = 1.5 the temperature overshoots first (`10 → 25 → 17.5 → 21.25`).
2. **Savings pool** and **discrete predator–prey** (equilibrium at prey = 100, predators = 50 for the chosen parameters)
3. **Constant-product market** (conserved product; a deliberate floating-point trap)
4. **Linear bonding curve** (exact conservation and lossless round trips)
5. **SIR epidemic** (conservation of S + I + R)
6. **Agent-based traders** (performance at 100, 1,000, and 10,000 agents)
7. **Robot arms and marbles** (periodic activation; from the cadCAD tutorials). The state after steps 0 to 10 is `(10,0)`, `(10,0)`, `(9,1)`, `(8,2)`, `(7,3)`, `(7,3)`, then `(5,5)` from step 6 onward, with `box_a + box_b == 10` throughout.

Expected values were computed independently with exact arithmetic, not by a Roxin implementation, and are listed with more models in `docs/core-semantics.md`. The thermostat values assume "8 steps" means 8 applications of the system.

---

## Risks

- **Scope creep** toward general-purpose or analysis tooling. The biggest risk.
- **Unwritten semantics** causing endless syntax churn. Mitigation: write the formal core first (drafted in `docs/core-semantics.md`).
- **Floating-point rounding** silently breaking invariant checks.
- **State explosion** in reachability, especially with joint `allowed` sets over several fields.
- **VM speed** on large agent-based models. Mitigation: parallel jobs first, then a faster backend only if measured timings justify it.
- **Learning Rust and building the compiler at the same time.** Mitigation: the tree-walking oracle comes first, and the VM is built second against known-good results.
- **Size of the build:** compiler, VM, checker, runtime, and result store, all from scratch. Mitigation: staged milestones that are each useful alone.
- **Adoption:** the cadCAD community faded, so the niche may be small. The project's value does not depend on adoption, but its direction might.
- **Partial evidence base:** only part of the cadCAD demo repository was reviewed (see below).

### Known gaps in the design notes

- Parts II and III of the GDS paper series, the Rust and Julia implementations, and Roxin's original papers have not been read.
- Of the cadCAD demos, only the repository README, an older networks tutorial notebook, and tutorial part 3 (periodic activation) were read. The Uniswap, bonding curve, adoption funnel, and three-sided market notebooks were not.
- Syntax for experiments, A/B variants, sweeps, and data declarations is a sketch.
- The result store, parallel runtime, and checkpointing are described but not specified.
- Reachability's rule for merging near-equal float states is undecided (see Relative tolerance).

---

## References

**In this repository**

- `docs/decisions.md`: decision log with alternatives, reasoning, costs, and revisit triggers (D1–D10 and project policy P1)
- `docs/core-semantics.md`: draft precise semantics and verified test vectors

**External**

- Zargham & Shorish, *Generalized Dynamical Systems Part I: Foundations* (2022): <https://research.wu.ac.at/ws/portalfiles/portal/23782375/Zargham_Shorish_GDS_Part_I_Foundations_2022.pdf>
- cadCAD formal specification and reference implementation (`cadCAD-org/spec`, `cadCAD-org/cadcad-ri`) and tutorials (`cadCAD-org/demos`)
- Roxin, E. (1965), *On Generalized Dynamical Systems Defined by Contingent Equations*, Journal of Differential Equations, vol. 1
- Nystrom, *Crafting Interpreters*: <https://craftinginterpreters.com>
- Pierce, *Types and Programming Languages* (records, subtyping)
- Lamport, *Specifying Systems* (TLA+; closest existing practice to reachability plus invariants)

---

## License

Not yet chosen.

---

## Open Questions

Resolved questions are recorded in the decided tables above and removed from this list. Questions are worked through in order.

| # | Question | Why it matters |
|---|---|---|
| Q1 | For GDS practitioners: was dropping admissibility from the reference code deliberate? Is the optional input stage intended? Are metrics part of the formalism? Does cadCAD intentionally merge same-named policy outputs, and how? What did the Julia and Rust efforts learn? | The specification and implementation disagree on several of these. This one needs answers from outside the project. |

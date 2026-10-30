# Roxin

**A language for describing how a system changes, who decides what happens next, and what must never go wrong.**

Roxin is a domain-specific language for modeling, simulating, and checking *generalized dynamical systems* (GDS). You write down a system's state, the choices that arrive or are made at each step, which choices are legal, and the rules that turn a choice into a new state. Then you run it at scale, sweep its parameters, and (as the project grows) check properties that must always hold.

It is named for Emilio Roxin, whose 1960s work on generalized dynamical systems is the mathematical origin of the idea.

> **Status: early preview.** The front end and a first evaluator work today: you can write simple models, sweep their parameters, and read the results. Most of the design is specified but not built yet. Every feature in this document carries a tag:
>
> - **[runs today]** works in the current build.
> - **[designed]** specified in the docs, not built yet. Syntax may still change.
> - **[exploring]** an open question; no decision yet.

---

## Contents

1. [Roxin in five minutes](#roxin-in-five-minutes)
2. [What is a generalized dynamical system, and why would you care?](#what-is-a-generalized-dynamical-system-and-why-would-you-care)
3. [Guides: GDS for your field](#guides-gds-for-your-field)
4. [What works today, and what is planned](#what-works-today-and-what-is-planned)
5. [A tour of the language](#a-tour-of-the-language)
6. [How the math works, and why](#how-the-math-works-and-why)
7. [Roxin and cadCAD](#roxin-and-cadcad)
8. [Scope](#scope)
9. [Project principles](#project-principles)
10. [Where it is going](#where-it-is-going)
11. [Documentation map](#documentation-map)
12. [References](#references) and [License](#license)
13. [Open Questions](#open-questions)

---

## Roxin in five minutes

A thermostat. The *decision* is the `input:` line. The *physics* is the `mechanism:` line.

<!-- example: thermostat.rox -->
```
-- Thermostat. Provisional syntax.
-- Start 10, setpoint 20, 8 steps. All values are exact binary fractions,
-- so float arithmetic reproduces them exactly (tolerance 1e-12 is still fine).
--
--   k      rows 0-3                        row 8
--   0.25   10, 12.5, 14.375, 15.78125      18.998870849609375
--   0.5    10, 15, 17.5, 18.75             19.9609375
--   1.5    10, 25, 17.5, 21.25             19.9609375
--
-- Variant with `allowed: heat in 0..5` on the step (add it under `input:`):
--   k = 0.5: first heat is 5 (boundary, legal).
--   k = 1.5: first heat is 15, an admissibility error at step 1 with temp = 10.

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

Run it:

```
cargo run --release -- examples/thermostat.rox
```

One run, three jobs (one per value of `k`). Each job prints its states, row 0 being the start:

<!-- output: thermostat.rox -->
```
job 0  k = 0.25
    0  { temp: 10 }
    1  { temp: 12.5 }
    2  { temp: 14.375 }
    3  { temp: 15.78125 }
    4  { temp: 16.8359375 }
    5  { temp: 17.626953125 }
    6  { temp: 18.22021484375 }
    7  { temp: 18.6651611328125 }
    8  { temp: 18.998870849609375 }
```

Inside a step, `s` is the old state and `u` is the input. Three things to notice:

- **The decision and the physics are separate.** Change how the heater is chosen without touching what heating does, or the reverse.
- **Nothing is hidden.** With `k = 0.25` the room creeps toward 20. With `k = 1.5` it overshoots (10, 25, 17.5, 21.25, ...) before settling. A sweep shows this in one run.
- **Every field is accounted for.** The mechanism sets `temp`, the only field of `Room`. A mechanism that forgets a field is an error, not a silent carry-over.

---

## What is a generalized dynamical system, and why would you care?

### The idea

A *dynamical system* is something with a **state** that changes over time by a rule: a population, a bank balance, a game world, a chemical reaction. Most simulations are exactly that, written as one big update function.

A *generalized* dynamical system makes one more thing explicit: **at each step, something is pushed into the system, and that something comes from a set of legal options.** The push might be a player's move, a trader's order, a drug dose, a heater setting, a vote, or a random draw. The system's rule says what the push does. A separate rule says which pushes are allowed in the current state.

The mathematics goes back to Emilio Roxin (1965), who studied systems where, from each state, a *set* of futures is possible rather than exactly one. The modern engineering treatment is Zargham & Shorish, *Generalized Dynamical Systems Part I: Foundations* (2022), which Roxin the language follows. It uses these terms, and so does Roxin:

| Paper term | Meaning | Everyday analogy | In Roxin |
|---|---|---|---|
| State `x` | everything about the system right now | your save file | `state` |
| Input `u` | what is chosen or arrives this step | the move you pick | the `input:` record |
| Admissible Input Map `U(x)` | which inputs are legal *in this state* | you can't buy a sword with 0 coins | `allowed:` |
| Input Map `g` | how the input is produced from the state | the player: human, bot, or dice | expressions in `input:`, or a named `input map` |
| State Update Map `f` | the law of motion: next state from state and input | the game rules | `mechanism:` |
| State Transition Map `h` | `h(x) = f(x, g(x))` | one full turn | `step` |
| Trajectory | `x0, x1, x2, ...` | the replay | the run output |

**An input is not a change to state.** The input is what gets chosen or arrives; the mechanism decides what that does. A thermostat's input is "heat 5," and the mechanism is the physics that turns it into a new temperature.

### Why it is worth the trouble

1. **Who decides is separate from what happens.** Swap the decision rule (a human, a bot, a policy, dice) without touching the laws of motion, or change the laws without touching the decision rule. A/B comparisons become a one-line change.
2. **What is legal is part of the model.** "You can't spend coins you don't have" is a statement in the model, not an `if` buried in code that someone may forget to apply.
3. **One model, three questions.** Because inputs come from a *set* of legal choices, the same model can answer:
   - **Simulate:** what happens if inputs are produced this way?
   - **Reach:** what *could* happen, given every legal input?
   - **Check:** is there any reachable state that breaks a rule?
4. **Rules that must always hold can be written down.** Total supply is conserved. A balance is never negative. Susceptible + infected + recovered equals the population. These become checkable statements instead of comments.
5. **A shared vocabulary.** A game designer, a token economist, an epidemiologist, and a control engineer all describe a system the same way: a state, a choice, a law of motion. Models become readable across fields.

### When not to use it

Roxin is for discrete-step models of systems with choices and rules. It is not a general programming language, a statistics or plotting environment, a continuous-time ODE solver, or a machine-learning framework. See [Scope](#scope).

---

## Guides: GDS for your field

Most explanations of GDS are written for mathematicians. These are written for **you**: each guide teaches the same ideas in the same order, using the vocabulary and the problems of one field, and every runnable example is a real model that runs.

| Guide | For | You will build |
|---|---|---|
| [GDS for game design and development](docs/guides/games.md) | designers, systems and economy designers, game programmers | a faucet-and-sink economy, cooldowns, an idle-game upgrade loop, mana-limited combat |
| [GDS for quant finance and tokenomics](docs/guides/tokenomics-quant.md) | quants, protocol and mechanism designers, token economists | an exact issuance schedule, a constant-product market maker, vesting with a conservation law |
| [GDS for life sciences](docs/guides/life-sciences.md) | epidemiologists, pharmacologists, ecologists, systems biologists | an epidemic with conserved population, a one-pill pharmacokinetic model, enzyme kinetics |
| [GDS for control and engineering](docs/guides/control-engineering.md) | control, mechanical and electrical engineers | a thermostat, a PID controller, an unstable system that diverges, a bang-bang controller |
| [GDS for collective decision-making and commons](docs/guides/governance-commons.md) | people who design or study councils, co-ops, commons, committees, and DAOs | a shared fishery under different numbers of users, and a quota rule compared against open access |

Each guide ends with a glossary that maps *your field's words* to the Roxin terms, so that someone who learned from one guide can read a model written from another. See the [guides index](docs/guides/README.md), which also explains how to write a new one. Your field not listed? That is the point of the format; guides are meant to multiply.

---

## What works today, and what is planned

| Feature | Status | Notes |
|---|---|---|
| `param` with defaults | **[runs today]** | typed, read-only inside steps |
| `state` records, `step`, `system`, `>>` composition | **[runs today]** | a mechanism must set every field exactly once |
| `input:` and `mechanism:` with simultaneous update | **[runs today]** | all mechanism fields read the old state |
| Arithmetic: `+ - * // % ^`, unary `-` | **[runs today]** | `/` on floats; on two ints it is an error (D5) |
| Exact integers | **[runs today]** with a limit | `i128` for now; arbitrary precision is decided (D1) but not built |
| Floats | **[runs today]** | a NaN, an infinity or a divide-by-zero stops the job (D4) |
| `run ... for N steps` | **[runs today]** | N+1 rows including the start |
| `sweep` over parameters | **[runs today]** | several sweeps take the Cartesian product |
| One failing job does not stop the others | **[runs today]** | rows computed before the failure are kept |
| Positioned error messages | **[runs today]** | line and column for syntax errors |
| Comparisons, `and`/`or`/`not`, `if/then/else`, built-ins (`sign`, `abs`, `min`, `max`) | **[designed]** | next up |
| `step` as a value (periodic actors) | **[designed]** | readable in `input:` and `allowed:` only |
| `allowed:` (legal inputs) with a clear error | **[designed]** | checked at runtime first; enumerable later |
| `invariant`, `metric`, `until` | **[designed]** | order of checks settled (D8, D10) |
| `..s` pass-through in mechanisms | **[designed]** | the author acknowledges untouched fields |
| Seeded randomness, `runs N`, `seed` | **[designed]** | xoshiro256++, reproducible bit-for-bit (D7) |
| `decimal(N)` with explicit rounding | **[designed]** | exact money-like arithmetic (D9) |
| Named, swappable `input map` | **[designed]** | for A/B comparison of decision rules |
| `type` (records, choices, aliases) | **[designed]** | `state` is the one evolving record |
| `data` (replayed CSV series) | **[designed]** | running past the end of the data is an error |
| Pre-run checker (wiring, types, purity) | **[designed]** | rejects a bad model before running |
| CSV / Parquet / Vega-Lite export | **[designed]** | typed result tables |
| Bytecode VM | **[designed]** | the tree-walker stays as a test oracle |
| Parallel jobs, result store, checkpoint/resume | **[designed]** | independent jobs, so results do not change |
| Reachability (bounded search over legal inputs) | **[designed]** | enumerates `allowed`, ignores input expressions |
| Proofs of invariants (inductive checking with a solver) | **[exploring]** | the language is built so this is possible |
| Lists, maps, agent populations | **[exploring]** | needed by agent-based models; not specified |
| Local `let`, bounded loops, float/int conversions | **[exploring]** | collected from real cadCAD models |

The test suite includes [52 models](docs/development-notes.md#implementation-status) with independently derived expected outputs. 32 run on today's build; the other 20 are written in advance, so each feature arrives with a ready-made test.

---

## A tour of the language

Provisional syntax throughout. Each block says whether it runs today.

### Models are state, steps, and a system **[runs today]**

A step has an optional `input:` stage and a `mechanism:`. A `system` chains steps with `>>`. Because all mechanism fields read the *old* state, updates are simultaneous, which is what makes this Fibonacci model correct:

<!-- example: fibonacci.rox -->
```
state Fib { a: int, b: int }

step Next:
    mechanism:
        a: s.b
        b: s.a + s.b

system Fibonacci: Fib = Next

run Fibonacci from Fib { a: 0, b: 1 } for 100 steps
```

Row 100 is `a: 354224848179261915075`, a number that does not fit in 64 bits. A step with no choice to make omits `input:` entirely.

### Decide, then happen **[runs today]**

The game-economy model splits "how much the player spends" (a decision) from "what spending does to the wallet" (the mechanism):

<!-- example: game-economy.rox -->
```
-- A gold faucet and a spending rule in an idle game.
-- The player decides how much to spend; the mechanism says what that does to the wallet.

param faucet: float = 100    -- gold earned per step
param spend_rate: float = 0.05  -- fraction of the wallet the player spends per step

state Wallet { gold: float }

step Economy:
    input:
        spend: float = spend_rate * s.gold     -- the decision
    mechanism:
        gold: s.gold + faucet - u.spend        -- the consequence

system Game: Wallet = Economy

run Game from Wallet { gold: 0 } for 60 steps
    sweep spend_rate in [0.02, 0.05, 0.10]
```

### Sweep a parameter **[runs today]**

`sweep spend_rate in [0.02, 0.05, 0.10]` runs three jobs. Several `sweep` lines take the Cartesian product; jobs are numbered with the first sweep varying slowest. Results carry the swept values, so a table of outcomes is one run away. The "Pick your guide" models above are all built this way.

### What is legal **[designed]**

`allowed` limits which inputs the step accepts. It is evaluated *before* the mechanism, and an input outside the set stops the job with an error that names the state and the input. Expected behavior for the thermostat with a heater that cannot exceed 5:

```
step Heating:
    input:
        heat: float = k * (setpoint - s.temp)
    allowed: heat in 0..5
    mechanism:
        temp: s.temp + u.heat
```

`k = 0.5` runs to the end (the first heat is exactly 5; the range includes its endpoints). `k = 1.5` stops at step 1: the first heat would be 15. This is the acceptance test for milestone M4.

### Rules that must hold **[designed]**

```
param total: int = 1_000_003
param months: int = 12

state Vest { locked: int, released: int }

invariant Conserved(s: Vest) = s.locked + s.released == total

step Unlock:
    input:
        amount: int = total * step // months - total * (step - 1) // months
    mechanism:
        locked: s.locked - u.amount
        released: s.released + u.amount
```

Tokens vest over twelve steps, and the integer remainder is spread across the schedule, so the sum is exact. The invariant is checked on the starting state and after every step. If it ever fails, the violating state is recorded as the job's last row, flagged with the invariant's name, and *other jobs keep running*.

### Time, periodic actors, and replayed data **[designed]**

`step` counts the transition being computed, starting at 1. A model where one robot acts every 2 steps and another every 3 (adapted from the cadCAD tutorials):

```
state Boxes { box_a: int, box_b: int }

step Robots:
    input:
        arm1: int = if step % 2 == 0 then sign(s.box_b - s.box_a) else 0
        arm2: int = if step % 3 == 0 then sign(s.box_b - s.box_a) else 0
    mechanism:
        box_a: s.box_a + u.arm1 + u.arm2
        box_b: s.box_b - u.arm1 - u.arm2
```

From `(10, 0)` the boxes read `(10,0) (10,0) (9,1) (8,2) (7,3) (7,3) (5,5)`, then stay at `(5,5)`, with `box_a + box_b == 10` throughout. Each decision gets its own input field and the mechanism combines them *explicitly*; nothing is merged implicitly.

Replaying outside data is a `data` declaration (for example `data prices = csv("prices.csv")`) indexed by `step` in an `input:` expression.

### Randomness **[designed]**

Randomness is only legal in `input:` expressions, never in the mechanism, so the law of motion stays pure and checkable. A fair coin walk:

```
step Move:
    input:
        dir: int = if uniform(0, 1) < 0.5 then 1 else -1
    mechanism:
        pos: s.pos + u.dir
```

A run says `runs 1000` and `seed 42`. Without a seed, a random one is drawn, saved with the results, and printed, so any run can be replayed.

---

## How the math works, and why

Roxin's numbers are deliberately strict. This section is the reasoning behind the choices that most surprise people. Each links to the decision log, `docs/decisions.md`, which records the alternatives considered, the costs, and what would make us revisit.

### There is no default number type **[designed]**

Every numeric field declares `int`, `decimal`, or `float`. Literals take their type from context, and mixing types without a conversion is a type error.

*Why:* a wrong default here is the kind that silently weakens a result. Python happily adds an `int` to a `float` to a `Decimal` in the same model, and a balance that was exact becomes approximate without anyone choosing that. Roxin makes the author say what each quantity *is*. (Today's evaluator still promotes int to float in mixed expressions; that is temporary and tracked in the development notes.)

### Integers never overflow (D1) **[runs today]** *with a limit*

`int` is a mathematical integer. `2 ^ 100` is `1267650600228229401496703205376`. The current build uses 128-bit integers as a stand-in: `30!` is exact, `33!` still fits, and `34!` stops the job with an overflow error instead of wrapping around. The decision is arbitrary precision; the stand-in is a documented gap.

*Why:* wraparound produces wrong answers that look fine. Real contracts and machines do overflow, so the guides show how to state a bound as an invariant (`invariant NoOverflow(s: Pool) = s.balance < 2^256`) rather than hide it in the number type.

### Floor division, and `/` is an error on integers (D5) **[runs today]**

`//` is floor division and `%` is the matching modulo, so `a == (a // b) * b + a % b` always. `-7 // 2` is `-4` and `-7 % 3` is `2` (the result takes the sign of the divisor). Plain `/` on two ints is an error:

```
job 0  
    0  { n: 8 }
  stopped at step 1: `/` is not allowed on two ints; use `//` for floor division
```

*Why:* integer division that silently truncates is a classic source of off-by-one drift. Making the author choose `//`, or convert to `decimal` or `float`, puts the rounding decision on the page. Floor (rather than truncate toward zero) keeps `%` consistent for negative numbers and makes `step % 2` always 0 or 1.

### Decimals carry their places, and rounding is written down (D9) **[designed]**

`decimal(2)` means exactly two places. Addition and subtraction are exact. Multiplying `decimal(N)` by `decimal(M)` is exact and has type `decimal(N+M)`; storing it back into a `decimal(N)` field requires `round(e, mode)` with `down`, `up`, or `nearest`. Division is only legal as the direct argument of `round`.

*Why:* for money-like quantities the rounding policy *is* the model. A constant-product pool with 110 apples that must keep `apples * oranges >= 10000`:

- rounding **up** gives `oranges = 90.909091` and a product of `10000.000010` (the invariant holds);
- rounding **down** gives `oranges = 90.909090` and a product of `9999.999900` (the invariant fails).

Both values were checked with exact arithmetic. A language that rounds for you cannot tell you which one you got.

### Floats need a stated tolerance (D6) **[designed]**

There is no default tolerance. `==` or `!=` on floats needs `within 0.5` (absolute), `within 0.0001%` (relative), or a model-wide `tolerance`; otherwise it is a check-time error. `<`, `<=`, `>`, `>=` never need one. `a == b within p%` means `|a - b| <= p * max(|a|, |b|)`: symmetric, no division, and a nonzero number is never "equal" to 0 under a relative tolerance.

*Why:* floating-point rounding quietly breaks invariant checks. In the fixtures, a constant-product pool with **no fee** and an invariant `x * y >= k0` fails at step 11 when 7 is traded each step: the product comes out `999999.9999999999`, below `k0` purely from rounding. A checker that pretends floats are exact would report a false alarm; one that hides the problem would miss real ones. Roxin makes the author choose.

### A bad number stops the job (D4) **[runs today]**

A NaN, an infinity, or a division by zero is an error that stops *that job*, recorded with the step. There is no "propagate NaN" mode. Other jobs in a sweep continue and rows computed before the failure are kept:

```
job 0  
    ...rows 0 to 2 (x = 1, then 1e150, then about 1e300), kept
  stopped at step 3: arithmetic error: the result is not a finite number
```

*Why:* a `nan` that flows quietly through ten thousand steps and shows up as an empty chart is the failure mode we are avoiding. Fail early, say where.

### One step is one record of simultaneous updates (D2) **[runs today]**

A mechanism produces the *whole* next state, and every field reads the *old* state. A mechanism that forgets a field is rejected:

```
job 0  
    0  { temp: 10, humidity: 40 }
  stopped at step 1: the mechanism does not set field `humidity`
```

(Once `..s` exists, `..s` acknowledges "everything else passes through unchanged.") *Why:* a single event often touches several variables (a transfer debits one account and credits another). When each variable has its own update function, the functions must agree by hand. One record, one expression, no disagreement. Today this is detected at run time, in the first step; the planned pre-run checker moves it before the run.

### Inputs do not read each other (D3) **[runs today]**

Input fields are independent: each may read the state and parameters, but not another input. The set of legal inputs is the product of per-field sets, filtered by explicit joint conditions in `allowed`. *Why:* it keeps the legal-input set enumerable (needed for reachability), the order of random draws fixed, and proofs simple. Any coupling is a visible predicate.

### Randomness is reproducible bit-for-bit (D7) **[designed]**

One generator (xoshiro256++, seeded by SplitMix64) per job; uniform floats are `(next_u64 >> 11) * 2^-53`; normal draws use Box–Muller; the math library is pinned. A seed reproduces the same run on every machine and in every engine. *Why:* simulation results that cannot be replayed cannot be debugged, compared, or trusted.

### Checks happen in a fixed order (D8, D10) **[designed]**

Invariants are checked on the start state first. `until` is tested before each step, so one already true does zero steps. Invariants are checked after each step. A state that both violates an invariant and satisfies `until` is reported as a violation: a stopping rule must not hide a broken state.

### Indentation is syntax, brackets suspend it (D12) **[runs today]**

Four spaces per level, tabs rejected, a line ending in `:` opens a block, and anything inside `( )`, `[ ]`, `{ }` may wrap freely. Comments are `--`. Identifiers are ASCII. *Why:* the blocks in a model are its structure, and a language that reads layout shows the structure it means; refusing tabs removes a class of invisible bugs.

### The words come from the paper (vocabulary rule) **[runs today]**

There is no `policy` keyword (the paper has an *input map*), and no domain aliases: a game's *turn*, a chain's *block*, a population's *generation* are all a `step`. The guides use familiar words once, to bridge, and then switch to the Roxin term. *Why:* one vocabulary means a model written by one field is readable by another.

---

## Roxin and cadCAD

[cadCAD](https://github.com/cadCAD-org) showed that GDS is a useful way to model complex systems such as markets, token economies, and governance. Roxin keeps cadCAD's model of a system (state, policies that choose inputs, and state update functions, composed into steps) and differs in where the structure lives. cadCAD is a Python library, so structure is carried by dictionaries, strings, and conventions. Roxin makes it part of the language so a compiler can check it.

The comparison below was written after reading every notebook and model in the public `cadCAD-org/demos` repository. Each "seen in" entry points at a concrete case there. Some are limits of the library and some are choices by the demo authors; the table says which where it matters. **Roxin's column is design intent unless marked [runs today].**

| Topic | In cadCAD (as seen in the demos) | In Roxin |
|---|---|---|
| Wiring errors | A state update block maps variable names (strings) to functions. In one dict, repeating a key keeps only the last entry. Seen in: the `ThreeSidedBasic` outflow block lists `fiat_reserve` three times, so only the last mechanism runs. | A mechanism must produce every state field exactly once (D2). **[runs today]**, detected at run time; a pre-run check is **[designed]**. |
| One update across several variables | Each update function returns one `(key, value)` pair, so one event touching two variables is split across functions that must agree by hand. Seen in: `mint1`/`mint2`, `capture_consumer_payments1`/`2`. | One mechanism returns the whole next state. **[runs today]** |
| Dynamic names | Keys are built by string concatenation (`'UNI_Q' + asset_id`). A typo is found at run time, if that branch runs. | Fields are names the checker resolves. Indexed collections are **[exploring]**. |
| Purity | Updates receive the previous state as a live Python object. Seen in: the adoption funnel mutates `prev_state['pool']` in place; the AMM agent code deep-copies to avoid that. Policies can also write into shared state: the AMM `actionDecoder` assigns into `params`. | Mechanisms and input maps are pure by construction; parameters are read-only. **[runs today]** for the evaluator; checker **[designed]**. |
| Reproducible randomness | Draws use whatever generator the author calls. Seen in: robot-marbles part 4 draws from the global `np.random`, and the `seeds` dicts passed to `append_configs` (holding `RandomState` objects) are never read anywhere in the repository; some adoption-funnel policies draw a value and then ignore it. | One seeded generator per job with a specified seed derivation (D7). **[designed]** |
| Policy outputs | Outputs from several policies are summed key by key. This is deliberate, but the sum is implicit and easy to misread. | Input fields are independent (D3), and combining them is written in the mechanism. **[runs today]** |
| Admissible inputs | No place in the model for "which inputs are legal". Seen in: legality is `if` branches or clamps inside policies, for example `max(x, 0)`. | `allowed:` is part of the model and is what exploration and checking work from (D3). **[designed]** |
| Properties | No construct for an invariant. Checking is done in notebooks after the run. | `invariant` is checked after every step and a violating state is recorded (D8, D10). **[designed]** |
| Numbers | Python `int` and `float` mix freely, and `Decimal` appears in some configs. Seen in: scaled-integer math in the control and Uniswap models, with hand-placed `//` and `+ 1` rounding. | No default numeric type; exact integers, exact decimals, explicit rounding, a stated float tolerance (D1, D5, D6, D9). Partly **[runs today]**. |
| Failure | A divide by zero or NaN is a Python exception or a silent `nan`, depending on the library used. | A bad number stops that job and is recorded; other jobs continue (D4). **[runs today]** |
| Parameter sweeps | Values are lists even for scalars. Equal-length lists are zipped and length-1 lists broadcast, which is easy to trigger unintentionally. | Several sweeps take a Cartesian product. **[runs today]** A way to *zip* sweeps is **[exploring]**. |
| Time inside a step | Substeps and timestep columns are bookkeeping that models reconstruct, for example `timestep % 2`, or a hard-coded `timestep == 990`. | `step` is readable in `input:` and `allowed:`. **[designed]** |
| Results | A table whose shape depends on config, and post-processing code to flatten it. | A typed results schema exported to Parquet, CSV, or SQLite. **[designed]** |
| Units | Written in comments (`#unit: fiat/labor`). | Not addressed. Roxin has no units either. |

**Same answers.** The thermostat and predator–prey models were run in cadCAD 0.5.3 as well, and the numbers match Roxin's, including the closed-form values in the model headers.

**What cadCAD has that Roxin does not (yet)**

- Any Python object as state: pandas tables, graphs, class instances, NumPy arrays. Several demos rely on this, especially the agent-based and AMM models. Roxin has records; lists and maps are unspecified.
- The Python ecosystem: plotting, notebooks, optimization, and statistics libraries.
- An existing user base, tutorials, and years of use. Roxin has none of these.
- Nothing about Roxin's performance is claimed here. A first informal measurement is in the [development notes](docs/development-notes.md#performance-notes-informal-prototype); it will be redone on the real build before anyone should quote it.

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
- Uneven or event-driven time (periodic activation through `step` is planned)

---

## Project principles

- **Defaults over configuration.** The 99% case should need no configuration. A new feature is judged by how little it costs someone who does not use it: *a feature you don't use should cost you no syntax.*
- **Boundary rule.** A default is acceptable where the common case is clear and a wrong default is visible and harmless (the seed, the progress bar). Where a wrong default could silently weaken a guarantee (number types, float tolerance), the author decides and the checker refuses to guess.
- **Verification rule.** Every feature must state how a solver would see it, or be marked unverifiable. The language is designed so that properties can be checked or proved, not only observed. The central proof is an inductive invariant: it holds at the start, and for every state satisfying it and every legal input, the mechanism's result satisfies it again.
- **Neutral, checkable documentation.** Claims are tied to evidence (a fixture, a decision record, a source). Comparisons say what was seen where.
- **Written by hand.** The implementation is written entirely by hand. AI assistance is limited to documentation, brainstorming, review, and explanation (policy P1, `docs/decisions.md`).

---

## Where it is going

| Milestone | What arrives | Done when |
|---|---|---|
| M1–M2 | evaluator, trajectory loop, output table **(in progress)** | the thermostat and predator–prey match their closed-form values; marbles reaches `(5,5)` |
| M3 | typed IR, bytecode compiler and VM | every benchmark gives identical results on the VM and the tree-walker |
| M4 | `allowed` with runtime checks; seeded sampling | thermostat with `k = 1.5` and `allowed: heat in 0..5` raises an error naming the state and input |
| M5 | sweeps, Monte Carlo, A/B variants; parallel jobs | a 3×2 sweep yields exactly 6 jobs |
| M6 | pre-run checker | an impure mechanism is rejected before running |
| M7 | reachability, metrics, invariant checking | the market model's reachable states all satisfy its invariant |
| M8–M9 | surface-syntax polish; governance model; composing subsystems | the governance model runs with parameters in state |

The detailed plan, the benchmark models, the risks, and the list of known gaps are in [`docs/development-notes.md`](docs/development-notes.md).

---

## Documentation map

| Document | What it is |
|---|---|
| [`docs/guides/`](docs/guides/README.md) | GDS taught from the point of view of a field (games, finance, life sciences, control, collective decision-making) |
| [`docs/core-semantics.md`](docs/core-semantics.md) | what a program *means*, precisely, with verified test vectors |
| [`docs/decisions.md`](docs/decisions.md) | every design decision: alternatives, reasoning, costs, revisit triggers |
| [`docs/development-notes.md`](docs/development-notes.md) | implementation status, roadmap, defaults, risks, known gaps |
| [`examples/`](examples/) | the models used in this README and the guides |

---

## References

**In this repository**

- `docs/decisions.md`: decision log with alternatives, reasoning, costs, and revisit triggers (D1–D13 and project policy P1)
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

Questions are worked through in order.

| # | Question | Why it matters |
|---|---|---|
| Q1 | For GDS practitioners: was dropping admissibility from the reference code deliberate? Is the optional input stage intended? Are metrics part of the formalism? Does cadCAD intentionally merge same-named policy outputs, and how? What did the Julia and Rust efforts learn? | The specification and implementation disagree on several of these. This one needs answers from outside the project. |

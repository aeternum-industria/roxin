# <img src="assets/roxin-logo-black.svg" width="36" />Roxin

**A language for describing how a system changes, who decides what happens next, and what must never go wrong.**

Roxin is a domain-specific language for modeling, simulating, and checking *generalized dynamical systems*. You write down a system's state, the choices that arrive or are made at each step, which of those choices are legal, and the rules that turn a choice into a new state. Then you run it, sweep its parameters, and check the properties that must always hold.

It is named for Emilio Roxin, whose 1960s work on generalized dynamical systems is the mathematical origin of the idea.

> **Early preview.** The front end and a first evaluator work today; much of what follows is specified and not yet built. The [checklist](#feature-checklist) at the end says exactly which is which. Syntax is provisional.

---

## Roxin in five minutes

Here is a complete Roxin model of a thermostat. The heater's **decision** is the `input:` line; the room's **physics** is the `mechanism:` line.

<!-- example: thermostat.rox -->
```
-- A proportional thermostat. The controller commands `k` times the error;
-- the room warms by exactly what was commanded.

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

```
cargo run --release -- examples/thermostat.rox
```

That command runs the model once for each value of `k`, producing three jobs. Each job prints its states, with row 0 being the starting state you gave it:

<!-- output: thermostat.rox -->
```
job 0 k = 0.25
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
- **Nothing is hidden.** At `k = 0.25` the room creeps toward 20. At `k = 1.5` it overshoots — 10, 25, 17.5, 21.25 — then rings down. One sweep shows both.
- **Every field is accounted for.** The mechanism sets `temp`, the only field of `Room`. A mechanism that forgets a field is an error, not a silent carry-over.

---

## The idea

A *dynamical system* has a **state** that changes over time by a rule: a population, a balance, a game world, a reaction.

A *generalized* dynamical system makes one more thing explicit: **at each step something is pushed into the system, and it comes from a set of legal options.** The push might be a player's move, a trader's order, a dose, a heater setting, a vote, or a random draw. One rule says what the push does. A separate rule says which pushes are legal from the current state.

That separation buys three things:

1. **Who decides is separate from what happens.** Swap a human for a bot for a policy for dice without touching the laws of motion — or change the laws without touching the decider.
2. **Legality is part of the model.** "You cannot spend coins you do not have" is a statement in the model, not an `if` buried in code that someone may forget to apply.
3. **One model, three questions.** *Simulate:* what happens if inputs are produced this way? *Reach:* what could happen, over every legal input? *Check:* is there any reachable state that breaks a rule?

Roxin uses the vocabulary of Zargham & Shorish, *Generalized Dynamical Systems Part I* (2022):

| Paper | Meaning | Everyday | Roxin |
|---|---|---|---|
| State `x` | everything about the system right now | your save file | `state` |
| Input `u` | what is chosen or arrives this step | the move you pick | the `input:` record |
| Admissible Input Map `U(x)` | which inputs are legal *in this state* | you cannot buy a sword with 0 coins | `allowed:` |
| Input Map `g` | how the input is produced from the state | the player: human, bot, or dice | `input:`, or a named `input map` |
| State Update Map `f` | next state, from state and input | the rules of the game | `mechanism:` |
| State Transition Map `h` | `h(x) = f(x, g(x))` | one full turn | `step` |
| Trajectory | `x0, x1, x2, …` | the replay | the run output |

**An input is not a change to state.** The input is what gets chosen; the mechanism decides what that does. A thermostat's input is "heat 5"; the mechanism is the physics that turns it into a temperature.

---

## A tour of the language

### State, steps, and a system

A step has an optional `input:` stage and a `mechanism:`. A `system` chains steps with `>>`. Every mechanism field reads the *old* state, so updates are simultaneous — which is what makes this correct:

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

Row 100 holds `a: 354224848179261915075`, a number too large for 64 bits. If those two mechanism lines ran one after the other instead of simultaneously, the same arithmetic would produce powers of two.

### Decide, then happen

This idle-game economy keeps how much a player chooses to spend separate from what spending does to their wallet:

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

The economy settles where the faucet equals the spend, at `faucet / spend_rate`. To ask what a different player would do, edit the `input:` line and leave the mechanism alone.

### Sweep a parameter

`sweep spend_rate in [0.02, 0.05, 0.10]` runs three jobs. Several `sweep` lines take the Cartesian product, with the first varying slowest. Results carry the swept values, so a table of outcomes is one line away.

### What is legal

`allowed:` sits between the decision and the consequence, and says which inputs the step accepts:

```
step Heating:
    input:
        heat: float = k * (setpoint - s.temp)
    allowed: heat in 0..5
    mechanism:
        temp: s.temp + u.heat
```

Bounds may depend on the state, so the legal set differs every step. An input outside it stops the job with an error naming the state and the input — it is not clamped and not skipped, because both of those are decisions about what happens when the impossible happens.

### Rules that must always hold

```
invariant Conserved(s: Vest) = s.locked + s.released == total
invariant Solvent(s: Wallet)  = s.gold >= 0
```

Roxin checks every invariant on the starting state and again after each step. A violation stops that job and records the violating state as the job's final row, flagged — so the state you most want to inspect is the last line of the output. Other jobs in the same run keep going.

### Time, and periodic actors

`step` counts the transition being computed, from 1, and is readable in `input:` and `allowed:`. Here two robots act on different cycles, one on every second step and one on every third:

```
step Robots:
    input:
        arm1: int = if step % 2 == 0 then sign(s.box_b - s.box_a) else 0
        arm2: int = if step % 3 == 0 then sign(s.box_b - s.box_a) else 0
    mechanism:
        box_a: s.box_a + u.arm1 + u.arm2
        box_b: s.box_b - u.arm1 - u.arm2
```

Each decision is its own input field and the mechanism combines them explicitly. Nothing is merged behind your back.

### Randomness

Random draws are legal only in `input:`, never in a mechanism, so the law of motion stays a pure function of state and input:

```
step Move:
    input:
        dir: int = if uniform(0, 1) < 0.5 then 1 else -1
    mechanism:
        pos: s.pos + u.dir
```

A run says `runs 1000` and `seed 42`. Without a seed one is drawn, recorded with the results, and printed, so any run can be replayed — on any machine, since the generator and the sampling method are part of the language rather than whatever the platform provides.

### Numbers that refuse to guess

Every numeric field declares `int`, `decimal`, or `float`. There is no default, and mixing types without a conversion is an error.

- **`int` is a mathematical integer.** No overflow, so `2 ^ 100` is exact. Real machines do wrap, so a model that needs that bound states it: `invariant NoOverflow(s: Pool) = s.balance < 2^256`.
- **`//` is floor division and `%` matches it**, so `a == (a // b) * b + a % b` always holds. Plain `/` on two ints is an error, on purpose — truncation that picks a direction silently is where off-by-one drift comes from.
- **`decimal(N)` keeps exactly N places.** Multiplying grows precision; storing the result back takes an explicit `round(e, mode)`. For money the rounding policy *is* the model.
- **Float `==` needs a stated tolerance** — `within 0.5`, `within 0.0001%`, or a model-wide `tolerance`. Ordering comparisons never do.
- **A NaN, an infinity, or a division by zero stops that job** and is recorded with the step. Other jobs continue. There is no propagate mode.

```
job 0
    0  { n: 8 }
  stopped at step 1: `/` is not allowed on two ints; use `//` for floor division
```

### Layout

Roxin uses indentation to express structure: four spaces per level, tabs rejected, and a line ending in `:` opens a block. Anything inside `( )`, `[ ]` or `{ }` may wrap freely, and comments start with `--`.

---

## Guides for your field

Each guide teaches the same concepts as this page, using the vocabulary and problems of one profession. Every example in a guide is a working model you can run from `examples/`.

| Guide | For | You will build |
|---|---|---|
| [Game design and development](docs/guides/games.md) | designers, economy and systems designers, gameplay programmers | a faucet-and-sink economy, cooldowns, an idle-game upgrade loop, mana-limited combat |
| [Quant finance and tokenomics](docs/guides/tokenomics-quant.md) | quants, protocol and mechanism designers, token economists | an exact issuance schedule, a constant-product market maker, vesting with a conservation law |
| [Life sciences](docs/guides/life-sciences.md) | epidemiologists, pharmacologists, ecologists, systems biologists | an epidemic with conserved population, a one-pill pharmacokinetic model, enzyme kinetics |
| [Control and engineering](docs/guides/control-engineering.md) | control, mechanical and electrical engineers | a thermostat, a PID controller, a system that diverges, a bang-bang controller |
| [Collective decision-making and commons](docs/guides/governance-commons.md) | people who design or study councils, co-ops, commons, committees, DAOs | a shared fishery under different numbers of users, a quota rule against open access |

If you have never built a model before, start with **[the from-zero modeling guide](docs/modeling.md)**. It assumes no mathematics beyond arithmetic, no programming and no prior vocabulary, and it hands you to a field guide at the end.

Each guide ends with a glossary mapping its field's words to Roxin's, so a model written from one is readable by someone who learned from another. See the [guides index](docs/guides/README.md) for how to write a new one.

---

## What Roxin is not

Roxin is not a general-purpose programming language, and it is not a statistics or plotting environment — export the results and use your usual tools for that. It is not a continuous-time ODE solver, a game engine, a bioinformatics toolkit, or a smart-contract language.

Roxin describes discrete-step systems with choices and rules, and answers questions about them.

---

## Feature checklist

**Working today**

- [x] `state`, `step`, `system`, `run`
- [x] `param` with typed defaults, read-only inside steps
- [x] `input:` and `mechanism:`, with simultaneous update
- [x] `>>` composition of steps
- [x] Optional `input:` stage
- [x] A mechanism must set every state field exactly once
- [x] Field access `s.x`, `u.y`
- [x] Arithmetic `+ - * // % ^`, unary `-`; `/` on floats
- [x] Exact integers — 128-bit today, arbitrary precision specified
- [x] NaN, infinity and divide-by-zero stop the job
- [x] `run … for N steps`, N+1 rows including the start
- [x] `sweep` over parameters, Cartesian product
- [x] One failing job does not stop the others
- [x] Positioned error messages
- [x] Indentation layout, `--` comments, numeric underscores

**Specified, not built yet**

- [ ] Comparisons, `and` / `or` / `not`, `if / then / else`
- [ ] Built-ins `sign`, `abs`, `min`, `max`, `sqrt`, `pow`
- [ ] `allowed:` — which inputs are legal
- [ ] `invariant` — what must always hold
- [ ] `until` — stop when a condition holds
- [ ] `step` readable as a value
- [ ] `..s` pass-through, and record update
- [ ] `type` — records, choices, aliases
- [ ] Named, swappable `input map`
- [ ] `decimal(N)` with explicit rounding
- [ ] Float tolerance with `within`
- [ ] `data` — replayed CSV series
- [ ] Seeded randomness, `runs N`, `seed`
- [ ] Pre-run checker for wiring, types and purity
- [ ] Typed results, CSV / Parquet / Vega-Lite export
- [ ] Parallel jobs, result store, checkpoint and resume
- [ ] Reachability — bounded search over every legal input

**Open questions**

- [ ] Lists, maps, agent populations
- [ ] Local `let`, user-defined `fn`
- [ ] Inductive invariant proofs, with runnable counterexamples

---

## Resources

- [`docs/modeling.md`](docs/modeling.md) — the from-zero modeling guide
- [`docs/guides/`](docs/guides/README.md) — the field guides
- [`docs/core-semantics.md`](docs/core-semantics.md) — what a program means, precisely
- [`examples/`](examples/) — the models used above and in the guides

Project working documents, for anyone who wants the reasoning: [`docs/registry.md`](docs/registry.md) records every feature's status and every decision with its alternatives and costs; [`docs/development-notes.md`](docs/development-notes.md) covers build order and risks.

**References.** Zargham & Shorish, *Generalized Dynamical Systems Part I: Foundations* (2022), [WU Vienna](https://research.wu.ac.at/ws/portalfiles/portal/23782375/Zargham_Shorish_GDS_Part_I_Foundations_2022.pdf) · Roxin, E. (1965), *On Generalized Dynamical Systems Defined by Contingent Equations*, J. Differential Equations 1 · [cadCAD](https://github.com/cadCAD-org), which showed GDS is a useful way to model complex systems.

## License

BSD 2-Clause — see [LICENSE](LICENSE).

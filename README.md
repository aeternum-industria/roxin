# Roxin

A domain-specific language for modeling, simulating, and checking **generalized dynamical systems** (GDS).

You describe a system (its state, who makes choices, and what the rules do), run it at scale, and export typed results to the tools you already use.

Named for Emilio Roxin, whose work in the 1960s on generalized dynamical systems is the mathematical origin of the idea.

> **Status: design phase.** Nothing is implemented yet. This README records the current design, what has been decided, what is still open, and what could go wrong. Syntax shown below is provisional.

---

## Why this exists

[cadCAD](https://github.com/cadCAD-org) showed that GDS is a useful way to model complex systems such as markets, token economies, and governance. It was built as a framework on top of Python, which forced heavy metaprogramming to get the structure it needed and made it hard to check anything before a long run started.

Roxin asks what happens if those ideas are first-class parts of a language:

- The compiler can reject a mis-wired model before it runs.
- A rule that must stay pure can be enforced instead of trusted.
- One program can be *simulated*, *explored exhaustively*, and *checked against invariants* without rewriting it.
- Results are typed, reproducible, and cached.

Roxin is also a vehicle for learning language design and implementation: type checking, effect systems, interpreters, and eventually compilation.

---

## GDS in 60 seconds

A GDS is a **state** plus a **rule** that produces the next state. The pieces:

| Piece | Meaning | Video-game analogy |
|---|---|---|
| State `x` | everything about the system right now | your save file |
| Input `u` | a choice made this step | the move you pick |
| Admissible inputs | the legal choices *in this state* | you can't buy a sword with 0 coins |
| Policy `g` | picks an input given the state | the player (human, bot, or dice) |
| Mechanism `f` | computes the next state from state and input | the game rules |
| Transition `h` | `h(x) = f(x, g(x))` | one full turn |
| Trajectory | `x0, x1, x2, ...` | the replay |

The central idea is **separating who decides from what happens**. Swap the policy without touching the rules, or change the rules without touching the policy.

Because inputs come from a *set* of legal choices, a GDS can answer three different questions about the same model:

1. **Simulate.** What happens if actors follow this policy?
2. **Reach.** What *could* happen, whatever the actors choose?
3. **Check.** Is there any reachable state that breaks a rule?

---

## A taste of Roxin (provisional syntax)

A savings pool: each step a user withdraws 10% of the balance, then the bank adds 1% interest.

```
space Pool { balance: float }
space Move { amount: float }

policy withdraw(s: Pool) -> Move
    allowed: 0 <= amount <= s.balance
    choose: amount = 0.10 * s.balance

mechanism apply(s: Pool, m: Move) -> Pool
    Pool { balance: (s.balance - m.amount) * 1.01 }

system Savings: Pool = withdraw >> apply

invariant NonNegative(s: Pool) = s.balance >= 0

run Savings from Pool { balance: 100 } for 10 steps
```

Expected first steps: `100 → 90.9 → 82.6281`.

Design intent for the surface language:

- Ordinary `name: Type` annotations everywhere. No invented type syntax.
- Keywords only for ideas with no everyday programming equivalent: `space`, `policy`, `mechanism`, `system`, `invariant`, `allowed`, `run`, and later `sweep` and `reach`.
- Domain, codomain, and time step need no keywords: signatures and `for N steps` carry them.

---

## Scope

**In scope**

- Spaces (typed records), policies, mechanisms, and series pipelines
- Invariants, parameter sweeps, Monte Carlo runs, seeded reproducibility
- Reachability analysis
- A small, pure expression language for rule bodies
- A closed set of built-in data connectors
- A typed result store with caching, plus export

**Out of scope (for now)**

- A general-purpose programming language
- Statistics and plotting inside the language (use the export layer instead)
- Notebook replacement
- User-defined effects and a package ecosystem
- Continuous time and ODE solvers

---

## Design decisions

### Decided

| Decision | Choice |
|---|---|
| Scope | A DSL for the model → experiment → results loop. Not general-purpose. |
| Analysis | Export layer: Parquet/CSV for R and Python, Vega-Lite for charts. |
| Syntax style | Standard typed annotations; keywords only for GDS concepts. |
| Build order | AST and semantics first, then evaluator, then parser, then surface syntax. |
| First implementation | Python tree-walking interpreter, treated as an executable specification. A faster implementation comes later and is tested against it. |
| `extern` (foreign code) | Deferred. Placeholder rules below. |
| Data connectors | Closed menu: CSV, Parquet, SQLite, HTTP GET. Effects only in `data` declarations and policies. |

### Current leans (not yet confirmed)

| Topic | Lean |
|---|---|
| Type equality of spaces | Exact structural (same field names and types). Width subtyping and row polymorphism as a later upgrade. |
| Policy vs mechanism | Two distinct forms. Policies may `choose` and `sample`; mechanisms must be pure, and the checker verifies it. |
| Admissible inputs | Written as set expressions (ranges, finite sets, products, filters) so they can be both checked and enumerated. Checked at runtime in v1. |
| Nondeterminism | Declared effects (`Choose`, `Sample`) with swappable handlers: Simulate(seed), Fixed(policy), Enumerate(depth). |
| Composition | Series (`>>`) only in v1. The typing rule for parallel composition is written now so the AST need not change later. |
| Substeps | Every step maps state to state, as in the cadCAD specification. |
| Checking | A pre-run checker for wiring, types, and mechanism purity. Admissibility and invariants checked at runtime. |
| Numbers | Exact decimals available alongside floats. |
| Expression language | Small and pure: records, lists, maps, `if` as an expression, `map`/`filter`/`fold`. Termination as an opt-in guarantee. |
| Parameters | A per-job record. Time-varying parameters live in the state explicitly. |
| Termination of a run | Maximum steps plus an absorbing-state test. |

### The `extern` placeholder rules

Foreign code is deferred, but four cheap rules keep the door open:

1. Every callable, built-in or later foreign, is looked up through **one function registry**.
2. Every function carries an **effect label** (pure, random, reads-file, network).
3. The word `extern` is **reserved** in the grammar.
4. Effects never appear inside mechanisms or ordinary expressions.

If a real model needs foreign code, the first candidate is WebAssembly for pure functions, since a sandboxed module can have its purity enforced rather than promised.

---

## Planned features

- Typed spaces, policies, mechanisms, and pipelines
- Invariants checked during simulation and over reachable states
- Parameter sweeps and Monte Carlo with run metadata (job index, seed, parameters)
- Reachability analysis (bounded breadth-first search over admissible inputs)
- Parallel execution of independent jobs
- A result store keyed by model, parameters, and seeds; checkpoint and resume
- Typed result tables, so a misspelled field fails before a long run
- Export to Parquet, CSV, and Vega-Lite chart specifications

---

## Roadmap

| Milestone | Build | Done when |
|---|---|---|
| M0 | Settle the open questions below; collect 3–5 real models from GDS practitioners | choices are written down |
| M1 | AST and a one-step evaluator with a fixed policy | savings pool matches `100 → 90.9 → 82.6281` |
| M2 | Trajectory loop, stop rule, output table | predator–prey reproduces step 2 and holds still at equilibrium |
| M3 | Admissible sets with runtime checks; seeded sampling | a bad input names the state and input; a seed repeats a run |
| M4 | Sweeps and Monte Carlo with run metadata | a 3×2 sweep yields exactly 6 jobs |
| M5 | Pre-run checker: wiring, types, mechanism purity | an impure mechanism is rejected before running |
| M6 | Reachability and invariant checking | the market model's reachable states all satisfy its invariant |
| M7 | s-expression reader, then the first surface syntax | all benchmark models are written as source text |
| M8 | Typed parallel composition; governance model | the governance model runs with parameters in state |

### Benchmark models (all must work without `extern`)

1. **Savings pool** and **discrete predator–prey** (equilibrium at prey = 100, predators = 50 for the chosen parameters)
2. **Constant-product market** (conserved product; a deliberate floating-point trap)
3. **Linear bonding curve** (exact conservation and lossless round trips)
4. **SIR epidemic** (conservation of S + I + R)
5. **Agent-based traders** (performance at 100, 1,000, and 10,000 agents)

Hand-computed expected values for these are in the project notes. Treat them as unverified until an implementation reproduces them.

---

## Open questions

| # | Question | Why it matters |
|---|---|---|
| Q1 | Where does `allowed` live: inside the policy or declared separately? How is the *evolving* space marked in a `system`? | Shapes the syntax and what the checker verifies at pipeline ends. |
| Q2 | Numbers and equality: decimals or floats by default? What tolerance for invariants? When are two states "equal"? | Floating-point rounding breaks conservation checks and prevents merging duplicate states in reachability. |
| Q3 | Reachability bounds: how to treat infinite or continuous inputs, how deep to search, what shape the result takes (set of states or transition graph). | Prevents state explosion and defines the feature. |
| Q4 | Parallel composition: do simultaneous policies see the same starting state? In what order are conflicting writes applied? | Required for multi-actor models and the agent-based benchmark. |
| Q5 | Who is the first audience (game balance, finance, biology), and so which vocabulary? | Three audiences means three vocabularies; serving all risks serving none. |
| Q6 | Performance target, and what measurement triggers a port from Python? | Decides when the executable specification becomes a reference. |
| Q7 | For GDS practitioners: is admissibility meant to be enforced? Is the policy/mechanism split mandatory? Which analytic ideas (reachability, controllability, invariants) are used in practice? What did the earlier Julia and Rust efforts learn? | The specification is silent or inconsistent on several of these. |
| Q8 | How many choices may one policy make per step? | Enumeration is much simpler if each policy makes one `Choose` per step. |

---

## Risks

- **Scope creep** toward general-purpose or analysis tooling. The biggest risk.
- **Unwritten semantics** causing endless syntax churn. Mitigation: write the formal core first.
- **Floating-point rounding** silently breaking invariant checks.
- **State explosion** in reachability.
- **Python speed** on large agent-based models.
- **Size of the build:** interpreter, checker, runtime, and result store, all from scratch. Mitigation: staged milestones that are each useful alone.
- **Adoption:** the cadCAD community faded, so the niche may be small. The project's value does not depend on adoption, but its direction might.
- **Unverified claims in the notes:** comparisons with other tools come from general knowledge, and some details of the reference implementation need checking against the repo.

---

## Known gaps in the design notes

- Parts II and III of the GDS paper series, the Rust and Julia implementations, and Roxin's original papers have not been read.
- Syntax for experiments, sweeps, and data declarations is a sketch.
- The result store, parallel runtime, and checkpointing are described but not specified.

---

## References

- Zargham & Shorish, *Generalized Dynamical Systems Part I: Foundations* (2022): <https://research.wu.ac.at/ws/portalfiles/portal/23782375/Zargham_Shorish_GDS_Part_I_Foundations_2022.pdf>
- cadCAD formal specification and reference implementation (`cadCAD-org/spec`, `cadCAD-org/cadcad-ri`)
- Roxin, E. (1965), *On generalized dynamical systems defined by contingent equations*, Journal of Differential Equations, vol. 1
- Nystrom, *Crafting Interpreters*: <https://craftinginterpreters.com>
- Pierce, *Types and Programming Languages* (records, subtyping)
- Pretnar, *An Introduction to Algebraic Effects and Handlers* (2015)
- Lamport, *Specifying Systems* (TLA+; closest existing practice to reachability plus invariants)

---

## License

Not yet chosen.

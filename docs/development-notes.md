# Roxin development notes

How the project is run: sequencing, risks, documentation strategy, and measurements. Everything about *what
the language is* — feature status, open questions, decision records — lives in
[`registry.md`](registry.md); everything about *how it is being developed* lives here.

Related documents: [`registry.md`](registry.md) — the source of truth for feature status, open questions and
every decision record — and [`core-semantics.md`](core-semantics.md) (precise semantics).

---

## Build order

*Replaces the M0–M9 roadmap, 2 Nov 2026.* The old plan assumed an s-expression reader first and "the first
surface syntax" at M8. Reality went differently: surface syntax exists, runs twelve examples, and M8 happened
out of order while M3's bytecode VM never started. The milestone list described an architecture that did not
happen, which made it actively misleading. Ordering below is by **dependency**, not by date.

Row IDs are [`registry.md`](registry.md) Part 1. Ordering is by what a feature *needs* in order to work — never
by how often something used it (P2).

### Wave 0 — clear the ground

Everything here is wrong today and all of it is small.

| | Why now |
|---|---|
| `RUN-20` `print_job` lost a space | One character, then regenerate 29 goldens and 3 doc blocks. **Do it before more goldens accumulate** |
| `SYN-02` six of nine `LexError` messages on the wrong variant | Wrong error text is worse than none |
| `SYN-05` `5--3` silently lexes as `5` | Silent wrong answer |
| `SYN-03` nine unreachable `TokenKind` variants | Includes `DotDot`, which LANG-41 will need |
| `RUN-23` panic on closed stdout | `roxin f.rox \| head` |
| `TOOL-21` stale `unknown_sweep_param.expected` | |
| The five `accidental` rows | A decision session, not code: `SYN-01`, `NUM-08`, `NUM-11`, `RUN-06`, `RUN-07` |
| **The test harness** | `tests/cli.rs` with `ROXIN_BLESS=1`. Belongs here, not later — it makes every wave below cheaper, and it is where the coverage work starts |

### Wave 1 — the expression core

`LANG-10` comparisons → `LANG-11` `and`/`or`/`not` → `LANG-12` `if/then/else`

> **The keystone is comparisons, not `if`.** `if` without comparisons branches on nothing. Every guard the
> language is reaching for — `if/then/else`, `until`, `invariant`, `allowed` — consumes a boolean, and
> comparisons are the only thing that produces one. That is a dependency, not a popularity contest.

Riding along once comparisons land: `LANG-19 until` and `NUM-06 within`, which need only a predicate apiece.

### Wave 2 — the two guards

`ADM-02` **first** — the AST shape. D14 settled that `allowed` keeps clause structure; the AST currently holds
`Option<Expr>` and the parser never fills it. Its own row says *fix before writing the parser*, and writing a
parser against the wrong shape is pure rework.

Then `ADM-01`/`ADM-03`, then `LANG-20 invariant`.

These two are what the README's pitch rests on. Until they exist, Roxin is a simulator.

### Wave 3 — the checker

`CHK-01`. Moves `LANG-07` and `NUM-04` from run time to check time, kills `CHK-02`, and is where `CHK-09`'s
explorability reporting lands per D19.

### Wave 4 — expressiveness

`LANG-24 let`, `LANG-43 fn`. Nothing blocks on these, which is why they are easy to postpone forever — and
everything after them involves writing real logic over real data, which is currently unwritable. A complex state
is worthless if no interesting logic can be expressed about it.

### Wave 5 — types

`NUM-09` range-refined `int` **first**. It is the only type that is both exact and enumerable, and it is what
makes `int` inputs explorable. Currently marked `deferred`, which undervalues it.

Then `LANG-41` record update (D22) → `LANG-17` records → `LANG-42 distinct` and `LANG-34` equality **together**,
since the second is hard to change later and the first decides it. Then finite-key maps, which are Tier 1 — see
the type-cost table in registry Part 3, and note it inverts the usual order: **a map is cheaper than a list**.

### Wave 6 — the explorer

`RUN-25` depth-1 → `CHK-03` reachability → `CHK-04` `X_C`. Needs Wave 2's `ADM-02`/`03`/`07`, and is far more
useful after Wave 5's `NUM-09`. `RUN-26`'s output shape must be settled before export ships — it now carries
three constraints, from D18, D19 and D20.

### Wave 7 — proofs

`CHK-05` inductive invariants, `CHK-11` counterexample presentation. `CHK-12` input recovery and `CHK-14`
P-controllability both fall out of Waves 6 and 7 rather than needing separate machinery.

### Parallel tracks

No dependencies on the waves; slot them wherever.

- **Randomness** (§7, D7). `uniform`/`runs`/`seed` depend on nothing else and nothing else depends on them.
- **`NUM-05 decimal`.** Nothing blocks it and it blocks nothing, but there is no correct type for money until it lands.
- **`LANG-18 data`.**
- **`TOOL-20` REPL rebuild.** Unblocked — D16 settled the model/experiment split that tells it what to load once and what to accept per line.
- **Release Part 0.** `Cargo.toml` is missing `license`, `description`, `repository`, `rust-version`; there is no `.github/`. `dist init` requires two of those and crates.io requires another.

### Two notes

**Testing is not a phase.** The old corpus was deleted in full (P2). Its replacement is built deliberately, a
batch per wave, arriving *with* the feature it exercises rather than ahead of it. The only standalone testing
work is Wave 0's harness, which should land before the first new models so that blessing a golden is one command
and a reviewed diff.

**The bytecode VM is deliberately absent.** `RUN-17` already records it as `deferred` and says the old M3
placement "belongs much later". The tree-walker runs everything and nothing in the current design needs a VM.

### Old milestone numbers

Some rows still cite the retired scheme.

| Old | Now |
|---|---|
| M0, M1, M2 | done — evaluator, trajectory loop, output table |
| M3 bytecode VM | `RUN-17`, deferred indefinitely |
| M4 `allowed` + sampling | Wave 2, plus the randomness track |
| M5 sweeps, Monte Carlo | sweeps done; Monte Carlo is the randomness track |
| M6 pre-run checker | Wave 3 |
| M7 reachability, metrics, invariants | Waves 2, 6 |
| M8 first surface syntax | done, out of order |
| M9 composing subsystems | `LANG-28`, deferred by D21 |

---

### Benchmark models

The benchmark values lived in the headers of the deleted corpus. Independently computed reference values for the
models the specification turns on are in [`core-semantics.md`](core-semantics.md) §14; feature status for each
benchmark is in [`registry.md`](registry.md) Part 1. The replacement corpus should carry its expected values the
same way — **computed independently of the interpreter**, so that a golden can disagree with the build and be
right.

## Documentation plan

- **The modeling guide, [`docs/modeling.md`](modeling.md) — drafted, 36 chapters, ~145 KB.** The on-ramp that comes *before* the field guides: those teach GDS in a reader's own field vocabulary and assume modeling is already a familiar activity, while this teaches modeling itself from zero and hands the reader to a guide at the end. Assumes no mathematics beyond arithmetic, no programming, no control theory, and no prior vocabulary; assumes only ordinary familiarity with whatever domain an example is drawn from. Held to [P4](registry.md) in full. Five acts, 36 chapters: **I — What you are actually doing** (no syntax at all); **II — Your first models**; **III — Decisions, consequences, and rules**; **IV — Being sure**; **V — Doing it for real**. One spine example carried end to end — a player with gold and potions, chosen because the decision, the consequence, the legality rule and the conservation law are all real rather than invented to satisfy the tool — with a bathtub as the second spine where rates and feedback are needed. Side examples rotate across domains so the core docs' one-third guard holds. Two chapters carry disproportionate weight and may not be trimmed: *Everything happens at once* (simultaneous mechanism evaluation, the misconception that silently produces wrong models forever) and *Starting from a blank page* (how to turn something you care about into a model at all, which nothing else in the documentation covers). Roughly a third of the document is P4 clause 3 material — concepts Roxin rejects, taught properly. It doubles as the author's own reference while building, which is why it is written early rather than last.
- **Core reference and tutorial:** domain-neutral, using a balanced mix of examples. The thermostat, marbles, predator–prey, SIR, and market models already span control, games, biology, and economics. A rough guard: no field supplies more than about a third of the examples in the core docs.
- **Audience guides:** same chapters, same order, same terms, different example families. The test for a guide is that a model written from one guide is readable by someone who learned from another. The guides must say plainly that unbounded `int` can diverge from a real contract at overflow boundaries, and show how to state the bound as an invariant (for example `invariant NoOverflow(s: Pool) = s.balance < 2^256`).
- **Terminology rule:** a guide may introduce a concept through a familiar word once, then switches to the Roxin term for good. Roxin ships no domain aliases.
- **Defaults reference:** every default listed in one page (see the defaults table), kept in sync with the implementation.
- **Decision log:** `docs/registry.md` records every decision with its alternatives and reasoning; new decisions are appended, and changes are added as dated notes, never silent edits.

---

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

Specific open gaps are Part 1 rows in [`registry.md`](registry.md) with status `open` or `accidental`.

## Performance notes (informal, prototype)

These numbers come from a throwaway prototype of the evaluator, not from the repository build, and from one 2-vCPU
Xeon (Rust 1.97 release build, Python 3.13, cadCAD 0.5.3, pandas 3.1.0). They are here to say *where to look*, not
to make claims. Re-measure on the real build before quoting any of them.

| Workload (thermostat model) | cadCAD 0.5.3 (execute only) | Roxin tree-walker prototype |
|---|---|---|
| 1 job, 8 steps | 0.014 s | 27 µs |
| 1 job, 200,000 steps | 3.1 s | 0.46 s |
| 1,000 jobs, 200 steps each | 25.4 s | 0.35 s |

Caveats that matter: the model is as small as a model gets; cadCAD does much more per step (state history, deep
copies, substeps, a results table), so this measures "framework overhead on a trivial model", not "modeling speed";
jobs ran sequentially on both sides (cadCAD in its local mode); and a tree-walker's advantage shrinks as models get
heavier. Both engines produced identical final values for the thermostat.

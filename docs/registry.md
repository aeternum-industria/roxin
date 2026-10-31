# Roxin registry

**Source of truth** for what exists, what state it is in, why it was decided, and what is unresolved.

This file replaced `docs/decisions.md`, which was folded in wholesale (Part 2) and deleted.

- **Part 1 — Index.** Every feature and decision point, its **status**, and **open questions**, grouped by subject. Status is a
  column, never a table — nothing is organised by "done" or "in progress". The `Decided`
  column cites a decision record in Part 2, the way a paper cites a reference.
- **Part 2 — Decision records.** The reasoning: alternatives, why, costs, revisit triggers. Append-only; a
  decision is never silently edited — if it changes, add a *Revisited* note with the date and reason.
- `docs/core-semantics.md` owns **precise meaning**. Point at the section; never restate it here.
- `README.md` is presentation only. It summarizes what is settled here. It never drives a decision.

**Lifecycle.** An item starts `open` or `accidental` → gets argued → becomes a D-number in Part 2 → its Part 1
row shrinks to a citation with status `designed` → gets built → status `runs`.

**Status vocabulary**

| | |
|---|---|
| `runs` | works in the current build, verified by running it |
| `designed` | specified, not built; syntax may still change |
| `open` | wanted, not yet decided |
| `accidental` | **implemented but never decided** — behavior exists and is observable; needs ratification or correction |
| `deferred` | wanted, deliberately not now |
| `rejected` | deliberately not doing; reason recorded |
| `defect` | known bug |

**Rule:** nothing gets built without a row in Part 1. Every working session starts by opening this file.

---

# Part 1 — Index
## 1. Language core

| ID | Item | Status | Decided | Spec | Open questions |
|---|---|---|---|---|---|
| LANG-01 | `param` with typed default | runs | — | §3 | See NUM-08, NUM-09 for evaluation-order issues |
| LANG-02 | `state` record declaration | runs | — | §3 | Only `int`/`float`/`bool` work at runtime; `decimal` and named types error |
| LANG-03 | `step` with `input:` / `mechanism:` | runs | [D2](#d2-a-mechanism-must-account-for-every-state-field-s-copies-the-rest), [D3](#d3-input-fields-are-independent) | §4 | — |
| LANG-04 | `system` with `>>` composition | runs | — | §5 | — |
| LANG-05 | `run … from … for N steps` | runs | — | §6 | — |
| LANG-06 | `sweep` (Cartesian product) | runs | — | §6.1 | Zip-style sweeps unresolved — see RUN-04 |
| LANG-07 | Mechanism must set every field exactly once | runs | [D2](#d2-a-mechanism-must-account-for-every-state-field-s-copies-the-rest) | §4 | Enforced at run time, first step only; moves to checker — see CHK-01 |
| LANG-08 | Optional `input:` stage | runs | — | §4 | Confirmed by paper §5.2 — cite it |
| LANG-09 | Field access `s.x`, `u.y` | runs | — | §2.3 | — |
| LANG-10 | Comparisons (`== != < <= > >=`) | designed | — | §2.3 | Float `==` needs tolerance — see NUM-06 |
| LANG-11 | `and` / `or` / `not` | designed | — | §2.3 | — |
| LANG-12 | `if / then / else` as an expression | designed | — | §2.3 | **Gates 9 of 20 pending fixtures.** Highest-value unbuilt feature |
| LANG-13 | Built-in calls (`sign`, `abs`, `min`, `max`, `sqrt`) | designed | — | §2.3 | Full math set unresolved — see LANG-26 |
| LANG-14 | `..s` pass-through in mechanisms | designed | [D2](#d2-a-mechanism-must-account-for-every-state-field-s-copies-the-rest) | §4 | — |
| LANG-15 | `step` readable in `input:` / `allowed:` | designed | — | §5 | — |
| LANG-16 | Named, swappable `input map` | designed | — | §3 | — |
| LANG-17 | `type` — records, choices, aliases | designed | — | §3 | Choices have no payloads in v1; see LANG-25 |
| LANG-18 | `data` — replayed CSV series | designed | — | §3 | Running past end of data is an error; `for N steps` from data length unresolved — see RUN-05 |
| LANG-19 | `until` stopping predicate | designed | [D10](#d10-order-of-checks-in-a-run) | §6 | — |
| LANG-20 | `invariant` | designed | [D8](#d8-a-violating-state-is-recorded-as-the-jobs-final-row-flagged), [D10](#d10-order-of-checks-in-a-run) | §6 | **Cite paper §4.4** — property `P` along a trajectory. Stop saying it is outside the formalism |
| LANG-21 | Per-step derived observable (currently `metric`) | designed | — | §3 | **Rename.** Paper's "metric" is `d_X` (Assumption 3.2), a distance function. Candidates: `observe`, `derived`, `track`, `report` |
| LANG-22 | Lists, maps | open | — | — | **The gap against the GDS premise.** Part I motivates GDS on arbitrary data structures; Roxin has flat scalar records. Either build, or state in the README which GDS models Roxin is for |
| LANG-23 | Agent populations / graphs | open | — | — | Depends on LANG-22. Needed by every agent-based model in the cadCAD corpus |
| LANG-24 | Local `let` bindings | open | — | — | Collected from real cadCAD models |
| LANG-25 | Sum types with payloads + `match` | open | — | — | cadCAD models pass strings and `'N/A'` padding for action dispatch; this is the typed replacement |
| LANG-26 | Float↔int conversions, integer sqrt, non-integer powers | open | — | §11 | Which math functions ship in v1 |
| LANG-27 | Bounded iteration | open | — | — | A line-search `while` loop appears in a cadCAD control model |
| LANG-28 | Output map `y = C(x, d)` | deferred | — | — | **Prerequisite for subsystem composition (M9).** gds-core's Tier 0 blocker; both paper authors are arguing about it. Read their resolution first |
| LANG-29 | Mark input fields `decision` vs `disturbance` | deferred | — | — | Part I Def 2.4 deliberately unifies them, so this is an extension, not a fix. Needed only if controllability (∃ input) is ever asked alongside safety (∀ input) |
| LANG-29 | Continuous time / ODE | rejected | — | — | Out of scope. Note Def 2.6 itself calls `f` a discrete transition |
| LANG-30 | Non-Markovian state history | rejected | — | — | cadCAD passes `StateHistory` everywhere; this violates Def 2.9 `h : X → X`. History goes in state explicitly |
| LANG-31 | Arbitrary host objects as state/params | rejected | — | — | Paper §5.2 permits it; cadCAD and gds-core both do it. **Rejecting it is Roxin's reason to exist** — it is what makes checking, enumeration and serialization possible |
| LANG-32 | `policy` as a keyword | rejected | — | — | Paper says Input Map. Keep the rule, but stop calling `policy` cadCAD-only vocabulary — gds-core has a `Policy` role under Zargham's direction |
| LANG-33 | Domain aliases (turn, block, generation) | rejected | — | — | One vocabulary; guides bridge once then switch |
| LANG-34 | Structural type equality for states and records | open | — | — | Lean is exact structural equality; width subtyping and row polymorphism later |
| LANG-36 | `map` / `filter` / `fold` over collections | open | — | — | Depends on lists/maps. Termination as an opt-in guarantee |
| LANG-37 | `extern` — foreign code | deferred | — | §11 | Four placeholder rules hold the door open: one function registry, effect labels, `extern` reserved, effects never inside mechanisms. WebAssembly is the first candidate |
| LANG-38 | Restricted state update map `f\|_x` | deferred | — | — | Paper Def 2.7; the object an inductive proof quantifies over. Nobody implements it, including gds-core |

## 2. Syntax and lexing

| ID | Item | Status | Decided | Spec | Open questions |
|---|---|---|---|---|---|
| SYN-01 | `^` is right-associative, binds tighter than unary minus | accidental | **needs D-number** | **missing from §2.3** | Implemented (`Caret` bp 31/30 vs `PREFIX_BP` 25) and pinned by `precedence.rox`, but the operator list does not mention `^` at all |
| SYN-02 | Six of nine `LexError` messages paired with the wrong variant | defect | — | — | A 4-cycle plus a swap. `BadIndent`, `IndentTooDeep`, `TabInIndent` are correct. No fixture catches it (severity high) |
| SYN-03 | Nine unreachable `TokenKind` variants | defect | — | — | `Bool`, `Decimal`, `Float`, `Int`, `Let`, `Match`, `String`, `Substep`, `Text`. `dead_code` exempts `pub` items so the compiler stays silent (severity low) |
| SYN-04 | `with` drift | defect | — | — | §2.3 shows `s with { … }`; D13 says there is no `with`. Both editor grammars follow §2.3 (severity low) |
| SYN-05 | `5--3` silently becomes `5` | defect | — | — | Comment swallows the subtraction. No fixture, no decision record (severity low) |
| SYN-06 | Indentation layout — four spaces, tabs rejected, brackets suspend | runs | D12 | §2 | `layout.rs` still carries `// TODO: Maybe allow 2/4 space indents both?` |
| SYN-07 | Lexical conventions — number underscores, four string escapes, ASCII identifiers, `--` comments | runs | D13 | §2 | No hex or binary literals yet |

## 3. Admissibility

| ID | Item | Status | Decided | Spec | Open questions |
|---|---|---|---|---|---|
| ADM-01 | `allowed:` checked at run time | designed | — | §8 | M4 acceptance test is `thermostat_allowed.rox` |
| ADM-02 | `allowed` keeps **clause structure** in the AST | designed | [D14](#d14-allowed-keeps-clause-structure) | §8 | Settled in discussion: a single `Option<Expr>` simulates fine but discards the structure enumeration needs. AST currently has `allowed: Option<Expr>` and the parser never fills it — **fix before writing the parser** |
| ADM-03 | Membership clauses narrow-only; conditionals go **inside** bounds | designed | [D15](#d15-membership-clauses-narrow-only-conditionals-go-inside-bounds) | §8 | Settled in discussion: `heat in 0..(if s.emergency then 1000 else 5)`, not `s.emergency or (heat in 0..5)`. Guarantees enumerability by construction. Cost: "no restriction" must be written as a wide-but-finite bound |
| ADM-04 | State-dependent bounds | designed | — | §8 | Not an extension — Def 2.5 **is** `U : X → ℘(U)`. A fixed literal range is the degenerate case |
| ADM-05 | Sweeping a parameter used in `allowed` | designed | — | §6.1 | Sweeps *the rules*, not the behavior — mechanism design in one line. Paper §5.1's governance example |
| ADM-06 | Rules that change mid-run | designed | — | §8 | Resolved by ADM-03 + state. Whatever toggles a rule must be visible in the state; a `phase`/`mode` field is the normal pattern. **Note:** a Roxin `step` is not a stage of the run — every step runs every tick |
| ADM-07 | Enumerable admissible sets for reachability | designed | — | §8 | `int` range, `decimal(N)` range, finite set, choice type. Float ranges are not enumerable |

## 4. Numbers

| ID | Item | Status | Decided | Spec | Open questions |
|---|---|---|---|---|---|
| NUM-01 | No default numeric type | designed | — | §2 | Evaluator currently promotes int→float in mixed expressions; decided rule is a type error. **Divergence** |
| NUM-02 | `int` is arbitrary precision | designed | [D1](#d1-int-is-an-arbitrary-precision-integer) | §2 | Build uses `i128`; `34!` stops the job. `factorial_overflow` fixture pins the divergence, `.expected-d1` holds the target |
| NUM-03 | `//` floor division, `%` floored modulo | runs | [D5](#d5-integer-division-is--floor-and--floored-modulo--on-ints-is-a-type-error) | §2.2 | — |
| NUM-04 | `/` on two ints is an error | runs | [D5](#d5-integer-division-is--floor-and--floored-modulo--on-ints-is-a-type-error) | §2.2 | Currently a run-time error; should be check-time |
| NUM-05 | `decimal(N)` with explicit `round` | designed | [D9](#d9-decimal-multiplication-grows-precision-division-only-appears-inside-round) | §2.2 | Errors at run time today as unsupported |
| NUM-06 | Float `==`/`!=` require a tolerance | designed | [D6](#d6-relative-tolerance-is-measured-against-the-larger-magnitude) | §2.1 | — |
| NUM-07 | NaN / infinity / divide-by-zero stops the job | runs | [D4](#d4-nan-infinity-and-division-by-zero-stop-the-job) | §2.2 | — |
| SYN-01 | Param defaults may reference earlier params | accidental | **needs decision** | contradicts §3 | Order-dependent: forward reference fails with `unknown name`. §3 says "name resolution is by declaration, in any order" |
| NUM-08 | Derived params follow sweeps | accidental | **needs decision** | — | `param b = a * 10` with `sweep a in [1,3]` gives b = 10, 30. Genuinely useful, but invisible: the job header records only `a`. Should derived params become result columns? |
| NUM-09 | Range-refined integer types (`type U256 = int in 0..2^256-1`) | deferred | [D1](#d1-int-is-an-arbitrary-precision-integer) | §11 | Syntax and checking details |
| NUM-10 | State metric `d_X` | open | — | — | Paper Assumption 3.2. **D6's open item — the rule for merging near-equal float states — *is* this.** Name it `d_X` when you build it |

## 5. Checking and verification

| ID | Item | Status | Decided | Spec | Open questions |
|---|---|---|---|---|---|
| CHK-01 | Pre-run checker: wiring, types, purity | designed | — | §9 | Ten rules listed in §9. Moves LANG-07 and NUM-04 from run time to check time |
| CHK-02 | Duplicate declarations rejected | defect | — | §3 | §3 says "names are unique within their kind" — **nothing enforces it.** Two `state S` declarations both register; `find_state` returns the first |
| CHK-03 | Reachability (bounded search over `allowed`) | designed | — | §8 | Depends on ADM-02, ADM-03, ADM-07. **Roxin enumerates; gds-core samples** — this is the differentiator, protect it |
| CHK-04 | Configuration space `X_C` | open | — | — | Paper Def 4.2. Largest mutually-reachable set. Cheap once CHK-03 exists; gds-core computes it as the largest SCC |
| CHK-05 | Inductive invariant proofs via solver | open | — | — | Unbounded `int` maps to SMT integer theory (D1). gds-core uses `sympy.simplify` on an implication, which is far weaker |
| CHK-06 | Attainability correspondence `F` | rejected | — | — | Paper Def 3.1. Analysis machinery; nobody implements it |
| CHK-07 | Contingent derivative, Lipschitz, existence theory | rejected | — | — | Paper §3. Requires convexity that general data structures cannot satisfy |
| CHK-08 | Local controllability | rejected | — | — | Paper Thm 4.4 requires `X ⊆ ℝⁿ` |

## 6. Execution and results

| ID | Item | Status | Decided | Spec | Open questions |
|---|---|---|---|---|---|
| RUN-01 | `for N steps` → N+1 rows | runs | — | §6 | — |
| RUN-02 | Per-job failure; other jobs continue; rows kept | runs | [D4](#d4-nan-infinity-and-division-by-zero-stop-the-job), [D8](#d8-a-violating-state-is-recorded-as-the-jobs-final-row-flagged) | §6 | — |
| RUN-03 | Job numbering, first sweep clause slowest | runs | — | §6.1 | Documented, untested |
| RUN-04 | Zip-style sweeps | open | — | — | cadCAD zips equal-length lists and broadcasts length-1. Roxin needs an explicit choice |
| RUN-05 | `for N steps` where N comes from data length | open | — | — | Interacts with RUN-06 |
| RUN-06 | Step count evaluated per job, after overrides | accidental | **needs decision** | — | `for n steps` with `sweep n in [1,3]` yields 2- and 4-row jobs. Useful, undocumented |
| RUN-07 | Sweep value lists evaluated in the default-param env | accidental | **needs decision** | — | So a sweep list may reference params. Interacts with NUM-08 and NUM-09 |
| RUN-08 | `runs N`, `seed`, reproducible RNG | designed | [D7](#d7-random-numbers-use-pinned-algorithms-so-runs-reproduce-exactly) | §7 | xoshiro256++, SplitMix64, Box–Muller, pinned `libm`. `random_walk.rox` pins one reading of job-seed derivation — confirm which is intended |
| RUN-09 | Randomness legal only in `input:` | designed | [D7](#d7-random-numbers-use-pinned-algorithms-so-runs-reproduce-exactly) | §7 | **Warranted by paper Def 2.4** — stochastic influence enters as an input |
| RUN-10 | Results schema | designed | — | §6.2 | Columns listed in §6.2. **Hardest thing to change later** — every export format and downstream analysis sees it |
| RUN-11 | Machine-readable output mode (NDJSON) | open | — | — | Would make the pretty format non-load-bearing, make float-tolerance comparison numeric, and deliver §6.2 early. **Decide before writing the test harness** |
| RUN-12 | KPI level (per-run, per-configuration) | open | — | — | `runs 1000` is meaningless without "mean final balance across runs". gds-core calls this `psuu.KPI` = Metric + Aggregation. Use `kpi` — the ecosystem's word |
| RUN-13 | CSV / Parquet / Vega-Lite export | designed | — | §11 | — |
| RUN-14 | Parallel jobs, result store, checkpoint/resume | designed | — | §11 | Jobs are independent, so results must not change |
| RUN-15 | `record substeps` | designed | — | §5 | Syntax provisional |
| RUN-16 | `keep running` per invariant | designed | [D8](#d8-a-violating-state-is-recorded-as-the-jobs-final-row-flagged) | §11 | Syntax provisional |
| RUN-17 | Bytecode VM | deferred | — | — | Tree-walker stays as oracle. **Roadmap currently has this at M3; it belongs much later** |
| RUN-18 | Policy-output merging | rejected | [D3](#d3-input-fields-are-independent) | — | cadCAD folds parallel policy outputs with `policy_ops=[lambda a,b: a+b]`. `U_x` is a set and `g` selects one `u`; merging has no formal counterpart |
| RUN-19 | Substeps as first-class semantics | rejected | — | — | Bookkeeping the paper has no concept of; recording stays opt-in (RUN-15) |
| RUN-20 | `print_job` lost a space | defect | — | — | `"job {} {}"` vs the two-space format every golden was generated from. Accounts for all 30 fixture mismatches and all 3 `verify_docs.py` failures (severity medium) |
| RUN-21 | Self-describing results | designed | — | §6.2 | Results record the effective settings used (seed, tolerance, step counting), so a file describes how it was produced |
| RUN-22 | Global mutable state | rejected | — | — | An experiment is a value, not an entry in a global registry |
| RUN-23 | Panic on closed stdout | defect | — | — | `roxin f.rox \| head` panics with a broken pipe. Unpleasant in exactly the pipelines fixtures and `verify_docs.py` use (severity low) |

## 7. Tooling, testing, release

| ID | Item | Status | Decided | Spec | Open questions |
|---|---|---|---|---|---|
| TOOL-01 | Fixture harness in `tests/cli.rs` | open | — | — | **File is empty; `cargo test` reports 0 passed.** 52 fixtures, 47 goldens, an independent oracle and written comparison rules all exist with no runner |
| TOOL-02 | `ROXIN_BLESS=1` golden regeneration | open | — | — | Turns a format change from an afternoon into a reviewed `git diff` |
| TOOL-03 | Pending fixtures fail when they start **passing** | open | — | — | Turns the corpus into a feature-completion detector |
| TOOL-04 | Unit tests per `LexError` variant | open | — | — | Would have caught SYN-02. No `today/` fixture exercises a lex error |
| TOOL-05 | Property tests (D5 identity, lexer never panics) | open | — | — | `a == (a // b) * b + a % b` is a universally quantified claim; example tests only sample it |
| TOOL-06 | `reference.py` + `verify_docs.py` in CI | open | — | — | Both work today and are run by nobody. `verify_docs.py` exits 1 correctly — usable as a gate with no wrapper |
| TOOL-07 | `.gitattributes` with `eol=lf` | open | — | — | Before the first Windows CI run, or 29 fixtures fail on carriage returns |
| TOOL-08 | License triple reconciled | open | — | — | **Blocks all packaging.** `LICENSE` is BSD-2-Clause, README says "not yet chosen", `Cargo.toml` has no `license` field |
| TOOL-09 | `Cargo.toml` metadata | open | — | — | Missing `description`, `repository` (required by `dist init`), `rust-version`, `keywords`, `categories` |
| TOOL-10 | MSRV policy + `rust-toolchain.toml` | open | — | — | Edition 2024 needs ≥1.85; developing on 1.99. An untested MSRV is a guess |
| TOOL-11 | CI workflow | open | — | — | fmt, clippy, 3-platform test matrix, MSRV, both oracles |
| TOOL-12 | release-plz | open | — | — | **Gotcha:** tags pushed with the default `GITHUB_TOKEN` do not trigger other workflows — needs a GitHub App token. Set `git_release_enable = false` so dist owns the Release |
| TOOL-13 | cargo-dist | open | — | — | Installers: shell, powershell, homebrew, msi. Scoop/WinGet/AUR/Nix are **not** dist publishers |
| TOOL-14 | crates.io Trusted Publishing | open | — | — | OIDC via `rust-lang/crates-io-auth-action`; no stored token |
| TOOL-15 | Scoop, AUR, WinGet, nixpkgs | deferred | — | — | **Rule: do not add a channel until its update is automated.** A stale manifest is worse than none |
| TOOL-16 | `cargo xtask gen` for editor grammars | open | — | — | Promote `keyword()` to a `pub const KEYWORDS` table; generate tmLanguage + Emacs `rx` + completions + man page; `--check` in CI. Would have prevented SYN-04 |
| TOOL-17 | Tree-sitter grammar | deferred | — | — | Unifies all editors + GitHub highlighting, but duplicates the parser against moving syntax. Revisit ~M8 |
| TOOL-18 | Stability table (what is / is not semver-protected) | open | — | — | Must declare pretty output and error wording **unstable**, or RUN-20 becomes a breaking change forever |
| TOOL-19 | Two-axis versioning (implementation vs language level) | open | — | — | A change that alters what existing source *means* is the only kind that can cost someone a wrong result |
| TOOL-20 | REPL | defect | — | §11 | Reads one line and parses it as a whole module, so no indented declaration can ever be typed. Either a continuation reader or scope it down to expressions |
| TOOL-21 | `unknown_sweep_param.expected` is stale | defect | — | — | Build now adds line/column. The analogous path in `eval_params` still uses `Span {0,0}` (severity low) |
| TOOL-23 | Syntax for experiments and A/B variants | open | — | — | Currently a sketch. Interacts with named `input map` |

---

# Part 2 — Decision records

Standing principles these entries rely on (see the README):

- **Defaults over configuration.** The 99% case must need no configuration. Every option has a named default chosen for the common case, rarely used machinery is hidden, and anything hidden stays configurable. A new feature is judged by how little it costs someone who does not use it. One of cadCAD's biggest pain points was the boilerplate needed before a basic run would execute.

**Boundary rule.** A default is acceptable where a wrong default is visible and harmless. Where a wrong default could silently weaken a guarantee, the author must decide and the checker refuses to guess.
- **Verification rule.** Every feature must state how it appears to a solver, or be marked unverifiable.
- **Neutrality.** No field's vocabulary or conventions are privileged.

---

## D1. `int` is an arbitrary-precision integer

**Decision.** `int` is a mathematical integer with no fixed width. Overflow does not exist. Implementations may use a fast small-integer path with a big-integer fallback. Range-refined integers (`type U256 = int in 0..2^256-1`, `type Percent = int in 0..100`), reusing the existing `in a..b` range form, are a planned later addition; a value outside its type's range is a run-time error. They are not in v1.

**Alternatives considered.**
- Fixed width (for example 64-bit) with wraparound or error on overflow.
- Several fixed-width types (`i32`, `u64`, `u256`, and so on) from the start.

**Why.**
- Models describe mathematics, and the integers of a model should behave like integers. Overflow in a model would be an accident of the implementation, not a property of the system being modeled.
- Verification: unbounded integers map directly onto the integer theory of SMT solvers. Fixed width would force bit-vector reasoning about wraparound, which is much harder for solvers and for people.
- Tokenomics work involves values near and above 2^64 and 2^256. A fixed-width default would either break such models or force a type zoo on everyone.
- Range-refinement reuses a form the language already has (`in a..b` from `allowed`) instead of inventing a second constraint syntax, and it leaves the door open to model fixed-width contracts exactly, when that matters.

**Costs and caveats.**
- An unbounded `int` can diverge from a real contract at overflow boundaries. This is stated plainly in the guides. A model can state the bound as an invariant: `invariant NoOverflow(s: Pool) = s.balance < 2^256`.
- Big-integer fallback costs some speed; the fast path keeps the common case cheap.
- Range-refined types sit in mild tension with the boundary rule (the type silently constrains values). It is acceptable because the author writes the range, the violation is a loud error, and unbounded behavior is the exact mathematical one.

**Revisit if.** Benchmarks show the big-integer fallback matters, or real models need wraparound arithmetic (then add explicit modular operations, not a default).

---

## D2. A mechanism must account for every state field; `..s` copies the rest

**Decision.** A `mechanism:` record must set or copy every field of the state type. A field it neither sets nor copies is a check-time error. `..s` copies every unmentioned field from the old state. All expressions read the old state, so updates are simultaneous.

```
mechanism:
  temp = s.temp + k * (heat - s.temp)
  ..s
```

**Alternatives considered.** Unmentioned fields stay unchanged with no marker (implicit keep).

**Why.**
- Forgetting to update a field is exactly the silent mistake the boundary rule exists for. Under implicit keep, a typo or omission is indistinguishable from "I meant it to stay the same", and the model runs and produces plausible wrong numbers.
- `..s` is one short line, and it records an author decision ("these are passthroughs"). It forces the author to consider the whole state.
- Verification: the mechanism is a total function from old state to new state with nothing filled in by hidden defaults.

**Costs.** One extra line in models where some fields do not change.

**Revisit if.** Real models show the line is a recurring annoyance. Even then the fix would be sugar, not removal of the check.

---

## D3. Input fields are independent

**Decision.** Input fields do not read each other (they are evaluated in declaration order, but an expression may not refer to a sibling field). The admissible set is the Cartesian product of per-field membership sets, filtered by any joint predicates. Any coupling between fields is an explicit predicate.

**Alternatives considered.** Coupled from the start: one combined rule defines the admissible set, with no per-field structure.

**Why.**
- Enumeration: reachability must list every admissible input. A product of finite per-field sets is directly walkable, and predicates just filter it. A single combined rule would require a search for valid combinations.
- Proofs: per-field ranges and predicates become plain linear constraints.
- Readability and determinism: the common case ("each decision has its own limits") reads directly, random-draw order is simple, and coupling is visible when it exists.
- It matches how the GDS paper treats the input as one space with an admissible subset.

**Costs.** A coupled rule such as "buy and sell cannot both be nonzero" needs a predicate on top of the per-field sets.

**Revisit if.** Composition of subsystems (milestone M9) needs inputs that depend on each other across subsystems.

---

## D4. NaN, infinity, and division by zero stop the job

**Decision.** A NaN or infinite float result, or division by zero of any numeric type, is an arithmetic error. The job stops, the error is recorded with the step, field, and expression, and other jobs in the sweep continue. There is no propagate mode.

**Alternatives considered.**
- Follow IEEE: let NaN and infinity flow through later steps.
- Configurable: stop by default, opt in to propagate.

**Why.**
- NaN spreads silently. One early bad division turns every later value into NaN, and every comparison with NaN is false, so an invariant such as `x >= 0` can pass or fail for the wrong reason. That is a wrong default that is invisible.
- Verification: solvers cannot reason about NaN the way they reason about numbers. If the checker can prove no division by zero is reachable, that is a clear guarantee.
- `int` and `decimal` have no IEEE fallback, so division by zero is an error for them under every option. Making floats consistent is simpler to explain.

**Costs.** A model that deliberately uses infinity (for example a "never" sentinel) must encode it differently.

**Revisit if.** A real model needs infinity as a value. Add an explicit opt-in then; do not build it speculatively.

---

## D5. Integer division is `//` (floor) and `%` (floored modulo); `/` on ints is a type error

**Decision.** `//` is floor division and `%` is floored modulo, so `a == (a // b) * b + a % b` always holds and `%` takes the sign of the divisor. `7 // 2 = 3`, `-7 // 2 = -4`, `-7 % 2 = 1`. Plain `/` on two `int` values is a check error ("use `//`, or convert to `decimal` or `float`"). `//` and `%` by zero are errors (D4).

**Alternatives considered.**
- Truncated division (C, Rust, Java), where `-7 / 2 = -3` and remainders can be negative.
- `/` on ints silently floors, truncates, or returns a float or rational.

**Why.**
- Languages disagree on `/` for integers, so any silent choice is a trap, and in tokenomics the rounding direction is money. This is the same reasoning as the decimal rule that division needs an explicit `round`.
- Floored modulo keeps results non-negative for a positive modulus, which is what "step mod 7" and wraparound indexing want. It matches Python, which many modelers already know.
- Verification: both operations are expressible in integer arithmetic, and floor division by a constant stays linear.

**Costs.** Slightly different from the hardware default. Not meaningful here.

**Revisit if.** There is a strong reason to match Rust's truncation (for example interoperating with generated Rust code). The author noted this can be revisited.

---

## D6. Relative tolerance is measured against the larger magnitude

**Decision.** `a == b within p%` means `|a - b| <= (p / 100) * max(|a|, |b|)`. A purely relative tolerance never makes a nonzero number equal to 0.

**Alternatives considered.** Relative to the second operand (`|a - b| <= tol * |b|`) or to the first.

**Why.**
- Symmetry: equality must not depend on argument order. Measuring against one operand makes `a == b` and `b == a` disagree near the boundary. With the symmetric test, reachability does not depend on which state was found first.
- No division, so no divide-by-zero case to define.
- Zero is deliberately exact under a relative tolerance: relative error against zero is meaningless (every nonzero number is infinitely far away in relative terms). Where "near zero" is meant, the author states an absolute tolerance. This follows the boundary rule: the author decides.

**Costs and caveats.**
- Tolerance equality is not transitive (`a ≈ b` and `b ≈ c` do not imply `a ≈ c`). Reachability therefore needs a deterministic rule for merging near-equal states (for example, keep the first one found). That rule is **not yet decided**.
- Near-equal states on opposite sides of a rounding boundary may fail to merge. That costs search size, never correctness.

**Revisit if.** The merge rule for reachability is designed, or users ask for a combined relative-or-absolute form as a built-in.

---

## D7. Random numbers use pinned algorithms so runs reproduce exactly

**Decision.**
- **Generator:** xoshiro256++.
- **Seeding:** SplitMix64 expands the 64-bit job seed into the generator's four state words. Job j's seed is `splitmix64(base ^ j)`, so parallel jobs never share a stream and results do not depend on thread scheduling.
- **Base seed:** if the run gives none, the runtime draws one from the operating system and records it in the results.
- **Uniform floats:** `u01 = (next_u64 >> 11) * 2^-53`, exactly representable.
- **Normal draws:** Box–Muller with a fresh pair on each call and no caching: `z = sqrt(-2 ln(1 - u1)) * cos(2π u2)`, then `mu + sigma * z`.
- **Math functions** (`ln`, `cos`, `sqrt`, and so on): a pinned pure-Rust `libm`, not the platform's.

**Alternatives considered.** Use whatever generator and math library the platform or a convenient crate provides; allow the implementation to choose and merely record the name.

**Why.**
- "Same seed, same result" is only a promise if it holds across machines, across Roxin versions, and across the bytecode VM and the tree-walking oracle (which must agree on every test).
- Platform math libraries can differ in the last bit between operating systems and CPUs, which would silently break reproducibility across machines.
- xoshiro256++ is fast, statistically strong, and short enough to specify exactly. SplitMix64 is the standard way to seed it. Box–Muller is simple to specify exactly.
- Per-job derived streams make parallel runs reproducible regardless of thread count.

**Costs.** Box–Muller (two uniforms per pair of normals) and a pinned `libm` are slower than the platform's best.

**Revisit if.** Benchmarks show sampling is a bottleneck. A different generator or sampler would be a versioned change, recorded in the results file so old results remain interpretable.

---

## D8. A violating state is recorded as the job's final row, flagged

**Decision.** When an invariant is false after a step, the job stops and the violating state is saved as the job's last row, with a flag and the name of the violated invariant. Nothing after it is recorded. For admissibility, arithmetic, or data errors no new state exists, so the last row is the previous state and the error record carries the offending state and input.

**Alternatives considered.** Leave the violating state out: the last saved row is the last good state and the violation appears only in the error record.

**Why.**
- Debugging: the first question after a violation is "what did the state look like when it broke?" With the row saved you can inspect it and compare it with the row before. Without it, you only know the previous state and the step number and must rerun to see the bad state.
- Honesty: the state exists, because the mechanism produced it. The flag marks it as not a legitimate part of the trajectory, so analysis can filter it with one condition.
- Invariant violations are the one failure where a new state exists, so recording it is consistent with how the other failures are treated.

**Costs.** Analysis must know about the flag, for example when averaging a metric across jobs. The flag is a column in every row, so filtering is one condition. A results file with a flagged final row must be read with the error table in mind.

**Revisit if.** Users routinely find the flagged row gets in the way of analysis, in which case the default could change while keeping the row available through an option.

---

## D9. Decimal multiplication grows precision; division only appears inside `round`

**Decision.**
- `decimal(N) * decimal(M)` is exact and has type `decimal(N+M)`. Example: `1.25 * 0.5` is `decimal(2) * decimal(1)`, giving `decimal(3)` and the value `0.625`.
- `decimal(N) * int` is exact and stays `decimal(N)`.
- `+` and `-` on `decimal(N)` are exact and stay `decimal(N)`.
- To store a product in a `decimal(N)` field, the author writes `round(e, mode)` with `down`, `up`, or `nearest`.
- `/` on decimals is legal only as the direct argument of `round`, as in `round(a / b, down)`. The result takes the precision of the field or annotation it is stored in.

**Alternatives considered.**
- Multiplication silently rounds back to a fixed precision with a default mode.
- Decimals carry unlimited precision and division produces a repeating or rational value.
- Allow `/` anywhere and round at the end.

**Why.**
- Exactness is the point of `decimal`. If multiplication silently rounded, `decimal` would offer little over a float in the cases it exists for (money, token amounts).
- Rounding direction is a policy: it decides who gets the leftover fraction, and over thousands of steps it adds up. Writing it where it happens puts the policy in the model and the audit trail. This is the boundary rule.
- Most divisions have no finite exact result (`1 / 3`), so the rounding mode has to be part of the operation, not applied afterwards.
- Verification: a decimal is an integer scaled by a power of ten, so decimal arithmetic maps onto the solver's integer theory.

**Costs.** Verbosity. Chains of multiplications grow precision quickly (three products of `decimal(2)` are `decimal(6)`), so models often write `round` after each product. This is judged the right default for this audience because the alternative hides the rounding.

**Revisit if.** Real models show a pattern of `round` boilerplate worth a named helper, which would be sugar over the same rules.

---

## D10. Order of checks in a run

**Decision.**
1. Check invariants on the initial state before step 1. A violation stops the job and records the start state as the flagged row (D8).
2. Before each step, test `until` on the current state. If it holds, the run ends cleanly and that state is the final row.
3. After each step, check invariants on the new state.

If a state both violates an invariant and satisfies `until`, the violation wins. After the last step of `for N steps`, the run ends without testing `until` again. A step that raises an arithmetic, admissibility, or data error never reaches the post-step invariant check, because no new state exists.

**Alternatives considered.** Skip the initial-state check and only check after steps; test `until` after each step instead of before.

**Why.**
- A bad start should fail loudly. A model that starts outside its own invariants (a typo in a parameter or initial value) would otherwise run on, or never report the problem. It also matches the proof pattern: an inductive invariant must hold at the start before it can be shown to be preserved.
- Testing `until` before each step matches the paper's stopping rule (`s = A(x_{k-1}) ∨ (k ≥ K)`): the decision to stop is made on the state already in hand. An `until` already true at the start does zero steps, which is what makes a state absorbing.
- Invariants take priority over `until` so a stopping rule cannot hide a broken state.
- `for N steps` is the `k ≥ K` part of the same rule; once step N has run, no further step would have run anyway.

**Costs.** One extra invariant evaluation per job on the initial state. Negligible.

**Revisit if.** A real model needs `until` to be evaluated on the post-step state for a different reason (for example, stopping after a state that triggers but does not break an invariant).

---

## Project policies

### P1. The implementation is written entirely by hand

**Policy.** All code in the Roxin repository (the Rust implementation, tests, and tooling) is written by the author, by hand. AI assistance is used for documentation, brainstorming, design review, and explanation. AI may show illustrative snippets and examples in conversation, including Rust language and library examples, but does not write code that goes into the repository. This may change in the future; if it does, it will be recorded here with a date.

**Why.**
- The primary purpose of the project is learning how languages are designed and implemented. Many properties of the design only become clear when the author writes the code and formalizes the relationships.
- The author wants to be able to state plainly that the language was implemented entirely by hand.

**What this means in practice.**
- Documents in `docs/` and the README may be drafted with AI help. Code never is.
- The "illustrative Rust types" in `docs/core-semantics.md` are documentation of the intended shape, not source to copy; the implementation is the author's own.
- Review is welcome: the author may paste their code and ask for explanations, bug hunting, or Rust idiom advice, and then make the changes themselves.

---

---
## D11. Float properties are verified on a best-effort ladder

*Recovered entry.* Referenced in `development-notes.md` but never written up; the alternatives and costs were
never recorded.

**Decision.** A float property is *proved* when the stated tolerance is larger than the provable error bound;
otherwise it is *checked on N runs*, or *unverified*. Difficulty: hard.

**Open.** The provable-error-bound machinery does not exist, so every float property is currently *checked* or
*unverified*. Needs a real write-up before anything depends on it.

---

## D12. Indentation is syntax; brackets suspend it

*Recovered entry.* Referenced in `development-notes.md` and the README but never written up.

**Decision.** Indentation is significant: spaces only, exactly four per level, tabs rejected. A line ending in
`:` opens a block. Brackets suspend layout, so anything inside `( )`, `[ ]` or `{ }` may wrap freely.

**Why.** The blocks in a model are its structure, and a language that reads layout shows the structure it
means; refusing tabs removes a class of invisible bugs.

**Open.** `layout.rs` carries `// TODO: Maybe allow 2/4 space indents both? TBD.` — decide and remove the TODO,
or record the alternative here.

---

## D13. Lexical conventions

*Recovered entry.* Referenced in `development-notes.md` but never written up.

**Decision.** Underscores allowed in numbers. Strings have exactly four escapes. Identifiers are ASCII-only.
There is **no `with`** — record update is `{ ..s, f: v }`. Clause words such as `map` and `seed` are not
reserved. `--` comments only. No hex or binary literals yet.

**Open.**
- `core-semantics.md` §2.3 still shows `s with { … }`, contradicting this entry, and both editor grammars
  follow §2.3. One of them is wrong — registry SYN-04.
- `5--3` silently lexes as `5` followed by a comment. Deliberate or not, it is unrecorded — registry SYN-05.

---

## D14. `allowed` keeps clause structure

**Decision.** `allowed:` is represented as structured clauses — membership clauses (`field in set`) kept
distinct from predicates — not collapsed into a single boolean expression.

**Alternatives considered.** `StepDecl.allowed: Option<Expr>`, one expression evaluated to true or false.

**Why.**
- Simulation needs only a yes/no answer, so a single expression suffices *for running a model*. Enumeration
  does not: to walk the admissible set you iterate the membership sets and apply predicates as a filter. A bare
  boolean can only be tested against a `u` you already hold, never used to generate candidates.
- Enumerating `allowed` exactly is what distinguishes Roxin from gds-core, whose `gds_analysis/reachability.py`
  cites the same Definition 4.1 but samples, because `U_x` there is a Python predicate it cannot invert.
- `core-semantics.md` §8 already describes `allowed` as a conjunction of clauses with the two kinds
  distinguished. This entry records why that distinction is load-bearing rather than stylistic.

**Costs.** A richer AST node, and a parser that must distinguish the two clause forms rather than reusing the
expression parser wholesale.

**Deadline.** `StepDecl` declares `allowed: Option<Expr>` and `parse_step` never fills it. Whichever shape
ships first settles this silently, so fix the AST before writing the parser for `allowed`.

**Revisit if.** Reachability is abandoned, or a clause form appears that cannot be expressed as
membership-plus-predicate.

---

## D15. Membership clauses narrow only; conditionals go inside bounds

**Decision.** Every input field carries a membership clause that is always in force. Its bounds may be
expressions over `s`, parameters and `step`, so they vary with state — but the clause never disappears.
Predicates may only narrow the set further; they may never widen it.

A rule that applies during part of a run and not another is written by moving the conditional **inside** the
bound, not by guarding the clause with a disjunction:

```
allowed: heat in 0..(if s.emergency then 1000 else 5)      -- bound varies, always finite
allowed: s.emergency or (heat in 0..5)                      -- rejected: bound vanishes in one branch
```

**Alternatives considered.**
- Allow arbitrary disjunction, normalize to disjunctive normal form, and check per-alternative that every input
  field is finitely bounded.
- Allow arbitrary disjunction and enumerate the union over branches.

**Why.**
- Enumerability becomes a property of the *grammar* rather than something the checker must prove. Every field
  always has a finite membership clause, so `R(x)` is always walkable: evaluate bounds at `x`, take the
  product, filter by predicates.
- The failure mode being prevented is specific. `or` itself is harmless — `heat in 0..5 or heat in 10..15` is
  two finite sets. The problem is a branch in which a field ends up with *no* bound, as in
  `s.emergency or (heat in 0..5)`, where the left side never mentions `heat`, so when emergency holds every
  possible `heat` qualifies and nothing finite remains.
- Definition 2.5 is `U : X → ℘(U)`, a map from state. State-dependent bounds are the formalism itself, not an
  extension; a fixed literal range is the degenerate case.
- No DNF normalization, no branch-coverage analysis, no "is this disjunction safe" rule in the checker.

**Costs.** "This restriction no longer applies" must be written as a wide-but-finite bound rather than a
vanishing clause. Judged acceptable: an actuator with genuinely no limit is rarely what a modeller means, and
the forced bound is usually the more honest model.

**Note.** Whatever toggles a rule must be visible in the state — a `phase` or `mode` field is the normal
pattern. A Roxin `step` is not a stage of the run; every step in a system runs every tick, so run phases cannot
be modelled by swapping which step is active.

**Revisit if.** A real model needs a genuinely unbounded admissible set for some states and sampling mode is
not an acceptable answer.

---

# Part 3 — Reference

## Project facts worth not re-deriving

| Fact | Detail |
|---|---|
| GDS Parts II and III were never written | Confirmed directly by Zargham. There is **no authoritative implementation spec**. Every `designed` tag here is a choice in territory the authors described and did not finish |
| Roxin's README table is accurate | Verified definition-by-definition against paper §2 and Table 1 |
| Reachability design is Definition 4.1 | `R(x) := ∪_{u∈U_x} {f(x,u)}` — enumerate `allowed`, ignore `g`. Cite it |
| cadCAD has no metric concept | All 15 occurrences are bookkeeping columns. Roxin's `metric` came from PSUU/BlockScience practice, not cadCAD |
| cadCAD has no admissibility vocabulary | `admissib` 0, `invariant` 0, `feasible` 0, `legal` 0, `constraint` 0 across the whole repo |
| Performance numbers are prototype-only | Development notes say re-measure on the real build. The 1,000-job sweep (25.4 s → 0.35 s) is nearest the 60× figure |

## Defaults

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

## How a solver sees each feature

Every feature must state how it appears to a solver, or be explicitly marked unverifiable. The central proof is
an inductive invariant: it holds at the start, and for every state satisfying it and every admissible input,
the mechanism's result satisfies it again. Proof support is planned, not built.

| Feature | How a solver sees it | Difficulty |
|---|---|---|
| `int` | integer arithmetic (unbounded, so no wraparound reasoning) | straightforward |
| `decimal(N)` | integers scaled by a power of ten; `round` is a defined integer operation | straightforward |
| `allowed` | declarative constraints; ranges and membership become linear constraints | straightforward |
| Pure mechanisms | a mechanism is a function from old state and input to new state | straightforward |
| `sample` | a nondeterministic choice: the proof must hold for every outcome | moderate |
| `float` | best effort (D11): a tolerance larger than the provable error bound gives *proved*; otherwise *checked on N runs* or *unverified* | hard |
| Lists and maps | need quantifiers or bounded unrolling | hard |
